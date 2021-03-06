use hexasphere::shapes::IcoSphere;

use crate::geometry::{Mesh, Vertex};

pub struct Icosphere {
    subdivisions: usize,
    radius: f32,
}

impl Icosphere {
    pub fn new(subdivisions: usize, radius: f32) -> Self {
        Self {
            subdivisions,
            radius,
        }
    }
}

impl From<Icosphere> for Mesh {
    fn from(icosphere: Icosphere) -> Self {
        let Icosphere {
            subdivisions,
            radius,
        } = icosphere;
        if subdivisions >= 80 {
            /*
            Number of triangles:
            N = 20

            Number of edges:
            E = 30

            Number of vertices:
            V = 12

            Number of points within a triangle (triangular numbers):
            inner(s) = (s^2 + s) / 2

            Number of points on an edge:
            edges(s) = s

            Add up all vertices on the surface:
            vertices(s) = edges(s) * E + inner(s - 1) * N + V

            Expand and simplify. Notice that the triangular number formula has roots at -1, and 0, so translating it one to the right fixes it.
            subdivisions(s) = 30s + 20((s^2 - 2s + 1 + s - 1) / 2) + 12
            subdivisions(s) = 30s + 10s^2 - 10s + 12
            subdivisions(s) = 10(s^2 + 2s) + 12

            Factor an (s + 1) term to simplify in terms of calculation
            subdivisions(s) = 10(s + 1)^2 + 12 - 10
            resulting_vertices(s) = 10(s + 1)^2 + 2
            */
            let temp = subdivisions + 1;
            let number_of_resulting_points = temp * temp * 10 + 2;

            panic!(
                "Cannot create an icosphere of {} subdivisions due to there being too many vertices being generated: {}. (Limited to 65535 vertices or 79 subdivisions)",
                subdivisions,
                number_of_resulting_points
            );
        }
        let generated = IcoSphere::new(subdivisions, |point| {
            let inclination = point.z.acos();
            let azumith = point.y.atan2(point.x);

            let norm_inclination = 1.0 - (inclination / std::f32::consts::PI);
            let norm_azumith = (azumith / std::f32::consts::PI) * 0.5;

            [norm_inclination, norm_azumith]
        });

        let raw_points = generated.raw_points();

        let points = raw_points
            .iter()
            .map(|&p| (p * radius).into())
            .collect::<Vec<[f32; 3]>>();

        let normals = raw_points
            .iter()
            .copied()
            .map(Into::into)
            .collect::<Vec<[f32; 3]>>();

        let uvs = generated.raw_data().to_owned();

        let mut vertices = Vec::with_capacity(points.len());
        for i in 0..points.len() {
            vertices.push(Vertex {
                position: points[i].into(),
                normal: normals[i].into(),
                tex_coord: uvs[i].into(),
            });
        }

        let mut indices = Vec::with_capacity(generated.indices_per_main_triangle() * 20);
        for i in 0..20 {
            generated.get_indices(i, &mut indices);
        }

        Self { vertices, indices }
    }
}
