use anyhow::Result;
use arara_asset::{AssetIoError, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use arara_logger::warn;
use arara_render::{Image, ImageType, Mesh, TextureError, Vertex};
use arara_utils::HashMap;
use glium::index::PrimitiveType;
use gltf::{mesh::Mode, Primitive};
use std::path::Path;
use thiserror::Error;

use crate::Gltf;

/// An error that occurs when loading a glTF file.
#[derive(Error, Debug)]
pub enum GltfError {
    #[error("unsupported primitive mode")]
    UnsupportedPrimitive { mode: Mode },
    #[error("invalid glTF file: {0}")]
    Gltf(#[from] gltf::Error),
    #[error("binary blob is missing")]
    MissingBlob,
    #[error("failed to decode base64 mesh data")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("unsupported buffer format")]
    BufferFormatUnsupported,
    #[error("invalid image mime type: {0}")]
    InvalidImageMimeType(String),
    #[error("You may need to add the feature for the file format: {0}")]
    ImageError(#[from] TextureError),
    #[error("failed to load an asset path: {0}")]
    AssetIoError(#[from] AssetIoError),
}

/// Loads glTF files with all of their data as their corresponding arara representations.
#[derive(Default)]
pub struct GltfLoader;

impl AssetLoader for GltfLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move { Ok(load_gltf(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["gltf", "glb"]
    }
}

/// Loads an entire glTF file.
async fn load_gltf<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), GltfError> {
    let gltf = gltf::Gltf::from_slice(bytes)?;
    let buffer_data = load_buffers(&gltf, load_context, load_context.path()).await?;

    let mut meshes = vec![];
    let mut named_meshes = HashMap::default();
    for mesh in gltf.meshes() {
        let mut primitives = vec![];
        for primitive in mesh.primitives() {
            let primitive_label = primitive_label(&mesh, &primitive);
            let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));
            let _primitive_topology = get_primitive_topology(primitive.mode())?;

            let positions = reader.read_positions().unwrap();

            let normals = reader.read_normals().unwrap();

            let tex_coords = reader.read_tex_coords(0).unwrap().into_f32().into_iter();

            let mut vertices = Vec::with_capacity(positions.len());
            for ((position, normal), tex_coord) in positions.zip(normals).zip(tex_coords) {
                vertices.push(Vertex {
                    position,
                    normal,
                    tex_coord,
                });
            }

            let indices = reader.read_indices().unwrap().into_u32().collect();

            let mesh = Mesh { vertices, indices };

            let mesh_handle =
                load_context.set_labeled_asset(&primitive_label, LoadedAsset::new(mesh));
            primitives.push(super::GltfPrimitive { mesh: mesh_handle });
        }
        let handle = load_context.set_labeled_asset(
            &mesh_label(&mesh),
            LoadedAsset::new(super::GltfMesh { primitives }),
        );
        if let Some(name) = mesh.name() {
            named_meshes.insert(name.to_string(), handle.clone());
        }
        meshes.push(handle);
    }

    if gltf.textures().len() == 1 {
        for gltf_texture in gltf.textures() {
            let (texture, label) = load_texture(gltf_texture, &buffer_data, load_context).await?;
            load_context.set_labeled_asset(&label, LoadedAsset::new(texture));
        }
    } else {
        load_context
            .task_pool()
            .scope(|scope| {
                gltf.textures().for_each(|gltf_texture| {
                    let load_context: &LoadContext = load_context;
                    let buffer_data = &buffer_data;
                    scope.spawn(async move {
                        load_texture(gltf_texture, buffer_data, load_context).await
                    });
                });
            })
            .into_iter()
            .filter_map(|res| {
                if let Err(err) = res.as_ref() {
                    warn!("Error loading glTF texture: {}", err);
                }
                res.ok()
            })
            .for_each(|(texture, label)| {
                load_context.set_labeled_asset(&label, LoadedAsset::new(texture));
            });
    }

    load_context.set_default_asset(LoadedAsset::new(Gltf {
        meshes,
        named_meshes,
    }));

    Ok(())
}

/// Loads a glTF texture as a arara [`Image`] and returns it together with its label.
async fn load_texture<'a>(
    gltf_texture: gltf::Texture<'a>,
    buffer_data: &[Vec<u8>],
    load_context: &LoadContext<'a>,
) -> Result<(Image, String), GltfError> {
    let texture = match gltf_texture.source().source() {
        gltf::image::Source::View { view, mime_type } => {
            let start = view.offset() as usize;
            let end = (view.offset() + view.length()) as usize;
            let buffer = &buffer_data[view.buffer().index()][start..end];
            Image::from_buffer(buffer, ImageType::MimeType(mime_type))?
        }
        gltf::image::Source::Uri { uri, mime_type } => {
            let uri = percent_encoding::percent_decode_str(uri)
                .decode_utf8()
                .unwrap();
            let uri = uri.as_ref();
            let (bytes, image_type) = match DataUri::parse(uri) {
                Ok(data_uri) => (data_uri.decode()?, ImageType::MimeType(data_uri.mime_type)),
                Err(()) => {
                    let parent = load_context.path().parent().unwrap();
                    let image_path = parent.join(uri);
                    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

                    let extension = Path::new(uri).extension().unwrap().to_str().unwrap();
                    let image_type = ImageType::Extension(extension);

                    (bytes, image_type)
                }
            };

            Image::from_buffer(
                &bytes,
                mime_type.map(ImageType::MimeType).unwrap_or(image_type),
            )?
        }
    };
    // texture.sampler_descriptor = texture_sampler(&gltf_texture);
    // if (linear_textures).contains(&gltf_texture.index()) {
    //     texture.texture_descriptor.format = TextureFormat::Rgba8Unorm;
    // }

    Ok((texture, texture_label(&gltf_texture)))
}

/// Returns the label for the `mesh`.
fn mesh_label(mesh: &gltf::Mesh) -> String {
    format!("Mesh{}", mesh.index())
}

/// Returns the label for the `mesh` and `primitive`.
fn primitive_label(mesh: &gltf::Mesh, primitive: &Primitive) -> String {
    format!("Mesh{}/Primitive{}", mesh.index(), primitive.index())
}

/// Returns the label for the `texture`.
fn texture_label(texture: &gltf::Texture) -> String {
    format!("Texture{}", texture.index())
}

/// Maps the `primitive_topology` form glTF to `glium`.
fn get_primitive_topology(mode: Mode) -> Result<PrimitiveType, GltfError> {
    match mode {
        Mode::Triangles => Ok(PrimitiveType::TrianglesList),
        // Mode::TriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        // Mode::Points => Ok(PrimitiveTopology::PointList),
        // Mode::Lines => Ok(PrimitiveTopology::LineList),
        // Mode::LineStrip => Ok(PrimitiveTopology::LineStrip),
        mode => Err(GltfError::UnsupportedPrimitive { mode }),
    }
}

/// Loads the raw glTF buffer data for a specific glTF file.
async fn load_buffers(
    gltf: &gltf::Gltf,
    load_context: &LoadContext<'_>,
    asset_path: &Path,
) -> Result<Vec<Vec<u8>>, GltfError> {
    const VALID_MIME_TYPES: &[&str] = &["application/octet-stream", "application/gltf-buffer"];

    let mut buffer_data = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Uri(uri) => {
                let uri = percent_encoding::percent_decode_str(uri)
                    .decode_utf8()
                    .unwrap();
                let uri = uri.as_ref();
                let buffer_bytes = match DataUri::parse(uri) {
                    Ok(data_uri) if VALID_MIME_TYPES.contains(&data_uri.mime_type) => {
                        data_uri.decode()?
                    }
                    Ok(_) => return Err(GltfError::BufferFormatUnsupported),
                    Err(()) => {
                        // TODO: Remove this and add dep
                        let buffer_path = asset_path.parent().unwrap().join(uri);
                        let buffer_bytes = load_context.read_asset_bytes(buffer_path).await?;
                        buffer_bytes
                    }
                };
                buffer_data.push(buffer_bytes);
            }
            gltf::buffer::Source::Bin => {
                if let Some(blob) = gltf.blob.as_deref() {
                    buffer_data.push(blob.into());
                } else {
                    return Err(GltfError::MissingBlob);
                }
            }
        }
    }

    Ok(buffer_data)
}

struct DataUri<'a> {
    mime_type: &'a str,
    base64: bool,
    data: &'a str,
}

fn split_once(input: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut iter = input.splitn(2, delimiter);
    Some((iter.next()?, iter.next()?))
}

impl<'a> DataUri<'a> {
    fn parse(uri: &'a str) -> Result<DataUri<'a>, ()> {
        let uri = uri.strip_prefix("data:").ok_or(())?;
        let (mime_type, data) = split_once(uri, ',').ok_or(())?;

        let (mime_type, base64) = match mime_type.strip_suffix(";base64") {
            Some(mime_type) => (mime_type, true),
            None => (mime_type, false),
        };

        Ok(DataUri {
            mime_type,
            base64,
            data,
        })
    }

    fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        if self.base64 {
            base64::decode(self.data)
        } else {
            Ok(self.data.as_bytes().to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::GltfNode;

    impl GltfNode {
        fn empty() -> Self {
            GltfNode {
                children: vec![],
                mesh: None,
                transform: arara_transform::prelude::Transform::identity(),
            }
        }
    }
    #[test]
    fn node_hierarchy_single_node() {
        let result = resolve_node_hierarchy(
            vec![("l1".to_string(), GltfNode::empty(), vec![])],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "l1");
        assert_eq!(result[0].1.children.len(), 0);
    }

    #[test]
    fn node_hierarchy_no_hierarchy() {
        let result = resolve_node_hierarchy(
            vec![
                ("l1".to_string(), GltfNode::empty(), vec![]),
                ("l2".to_string(), GltfNode::empty(), vec![]),
            ],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "l1");
        assert_eq!(result[0].1.children.len(), 0);
        assert_eq!(result[1].0, "l2");
        assert_eq!(result[1].1.children.len(), 0);
    }

    #[test]
    fn node_hierarchy_simple_hierarchy() {
        let result = resolve_node_hierarchy(
            vec![
                ("l1".to_string(), GltfNode::empty(), vec![1]),
                ("l2".to_string(), GltfNode::empty(), vec![]),
            ],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "l1");
        assert_eq!(result[0].1.children.len(), 1);
        assert_eq!(result[1].0, "l2");
        assert_eq!(result[1].1.children.len(), 0);
    }

    #[test]
    fn node_hierarchy_hierarchy() {
        let result = resolve_node_hierarchy(
            vec![
                ("l1".to_string(), GltfNode::empty(), vec![1]),
                ("l2".to_string(), GltfNode::empty(), vec![2]),
                ("l3".to_string(), GltfNode::empty(), vec![3, 4, 5]),
                ("l4".to_string(), GltfNode::empty(), vec![6]),
                ("l5".to_string(), GltfNode::empty(), vec![]),
                ("l6".to_string(), GltfNode::empty(), vec![]),
                ("l7".to_string(), GltfNode::empty(), vec![]),
            ],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 7);
        assert_eq!(result[0].0, "l1");
        assert_eq!(result[0].1.children.len(), 1);
        assert_eq!(result[1].0, "l2");
        assert_eq!(result[1].1.children.len(), 1);
        assert_eq!(result[2].0, "l3");
        assert_eq!(result[2].1.children.len(), 3);
        assert_eq!(result[3].0, "l4");
        assert_eq!(result[3].1.children.len(), 1);
        assert_eq!(result[4].0, "l5");
        assert_eq!(result[4].1.children.len(), 0);
        assert_eq!(result[5].0, "l6");
        assert_eq!(result[5].1.children.len(), 0);
        assert_eq!(result[6].0, "l7");
        assert_eq!(result[6].1.children.len(), 0);
    }

    #[test]
    fn node_hierarchy_cyclic() {
        let result = resolve_node_hierarchy(
            vec![
                ("l1".to_string(), GltfNode::empty(), vec![1]),
                ("l2".to_string(), GltfNode::empty(), vec![0]),
            ],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn node_hierarchy_missing_node() {
        let result = resolve_node_hierarchy(
            vec![
                ("l1".to_string(), GltfNode::empty(), vec![2]),
                ("l2".to_string(), GltfNode::empty(), vec![]),
            ],
            PathBuf::new().as_path(),
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "l2");
        assert_eq!(result[0].1.children.len(), 0);
    }
}
