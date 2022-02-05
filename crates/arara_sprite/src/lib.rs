mod bundle;
mod render;
mod sprite;
mod texture_atlas;

use arara_app::{App, Plugin};
use arara_asset::{AddAsset, Assets, HandleUntyped};
use arara_render::{
    clear_phase_system, DrawFunctions, Mesh, RenderPhase, RenderPhases, RenderStage,
    SpecializedPipelines, Square,
};
use bevy_reflect::TypeUuid;

use crate::{
    render::{
        draw_function::DrawSprite,
        extract_phase::{extract_sprite_entities, ExtractedSprites},
        phase_items::Transparent2D,
        pipelines::SpritePipeline,
        prepare_phase::prepare_sprite_phase,
        queue_phase::queue_sprite_phase,
    },
    texture_atlas::TextureAtlas,
};

pub mod prelude {
    pub use crate::{
        bundle::{SpriteBundle, SpriteSheetBundle},
        texture_atlas::{TextureAtlas, TextureAtlasSprite},
    };
}

pub const QUAD_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 13424831283718905134);

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TextureAtlas>()
            .init_resource::<SpritePipeline>()
            .init_resource::<SpecializedPipelines<SpritePipeline>>()
            .init_resource::<DrawFunctions<Transparent2D>>()
            .init_resource::<RenderPhase<Transparent2D>>()
            .init_resource::<ExtractedSprites>()
            .add_system_to_stage(RenderStage::Extract, extract_sprite_entities)
            .add_system_to_stage(RenderStage::Prepare, prepare_sprite_phase)
            .add_system_to_stage(RenderStage::Queue, queue_sprite_phase)
            .add_system_to_stage(RenderStage::Cleanup, clear_phase_system::<Transparent2D>);

        let draw_sprite = DrawSprite::new(&mut app.world);
        app.world
            .get_resource::<DrawFunctions<Transparent2D>>()
            .unwrap()
            .write()
            .add(draw_sprite);

        app.world
            .get_resource_mut::<Assets<Mesh>>()
            .unwrap()
            .set_untracked(QUAD_MESH_HANDLE, Mesh::from(Square::default()));

        app.world
            .get_resource_mut::<RenderPhases>()
            .unwrap()
            .add::<Transparent2D>();
    }
}
