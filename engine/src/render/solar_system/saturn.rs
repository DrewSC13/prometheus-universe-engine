use super::{body_visual_position, BodyId, SimulationClock};

use bevy::prelude::*;

pub(super) const SATURN_RING_MARKERS: usize = 192;
pub(super) const SATURN_RING_INNER_RADIUS: f32 = 1.65;
pub(super) const SATURN_RING_OUTER_RADIUS: f32 = 2.35;
pub(super) const SATURN_RING_MARKER_RADIUS: f32 = 0.025;
pub(super) const SATURN_RING_MESH_SEGMENTS: usize = 192;
pub(super) const SATURN_RING_MESH_Y_SCALE: f32 = 0.42;
pub(super) const SATURN_RING_MESH_BANDS: [(f32, f32); 3] =
    [(1.66, 1.82), (1.89, 2.06), (2.13, 2.34)];

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct SaturnRingMeshVisual {
    pub parent_body_id: BodyId,
    pub band_index: usize,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct RingMarkerVisual {
    pub parent_body_id: BodyId,
    pub index: usize,
    pub total: usize,
    pub ring_radius: f32,
}

pub(super) fn spawn_saturn_ring_mesh_bands(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    parent_body_id: BodyId,
) {
    for (band_index, (inner_radius, outer_radius)) in SATURN_RING_MESH_BANDS.iter().enumerate() {
        let mesh = meshes.add(build_saturn_ring_annulus_mesh(
            *inner_radius,
            *outer_radius,
            SATURN_RING_MESH_SEGMENTS,
        ));

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(Vec3::ZERO),
            SaturnRingMeshVisual {
                parent_body_id,
                band_index,
            },
            Name::new(format!("Saturn Ring Mesh Band {}", band_index)),
        ));
    }
}

fn build_saturn_ring_annulus_mesh(inner_radius: f32, outer_radius: f32, segments: usize) -> Mesh {
    let segments = segments.max(3);
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(segments * 6);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(segments * 6);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(segments * 6);

    for index in 0..segments {
        let a0 = std::f32::consts::TAU * index as f32 / segments as f32;
        let a1 = std::f32::consts::TAU * (index + 1) as f32 / segments as f32;

        let outer0 = Vec3::new(
            a0.cos() * outer_radius,
            0.0,
            a0.sin() * outer_radius * SATURN_RING_MESH_Y_SCALE,
        );
        let inner0 = Vec3::new(
            a0.cos() * inner_radius,
            0.0,
            a0.sin() * inner_radius * SATURN_RING_MESH_Y_SCALE,
        );
        let outer1 = Vec3::new(
            a1.cos() * outer_radius,
            0.0,
            a1.sin() * outer_radius * SATURN_RING_MESH_Y_SCALE,
        );
        let inner1 = Vec3::new(
            a1.cos() * inner_radius,
            0.0,
            a1.sin() * inner_radius * SATURN_RING_MESH_Y_SCALE,
        );

        let vertices = [outer0, inner0, outer1, outer1, inner0, inner1];
        let vertex_uvs = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
        ];

        for (vertex, uv) in vertices.into_iter().zip(vertex_uvs) {
            positions.push([vertex.x, vertex.y, vertex.z]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push(uv);
        }
    }

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

pub(super) fn spawn_ring_markers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    parent_body_id: BodyId,
) {
    if parent_body_id == BodyId::Saturn {
        spawn_saturn_ring_mesh_bands(commands, meshes, material.clone(), parent_body_id);
    }

    for ring_radius in [SATURN_RING_INNER_RADIUS, SATURN_RING_OUTER_RADIUS] {
        for index in 0..SATURN_RING_MARKERS {
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_scale(Vec3::splat(SATURN_RING_MARKER_RADIUS)),
                Visibility::Visible,
                RingMarkerVisual {
                    parent_body_id,
                    index,
                    total: SATURN_RING_MARKERS,
                    ring_radius,
                },
            ));
        }
    }
}

pub(super) fn update_ring_markers(
    simulation_clock: Res<SimulationClock>,
    mut queries: ParamSet<(
        Query<(&RingMarkerVisual, &mut Transform)>,
        Query<(&SaturnRingMeshVisual, &mut Transform)>,
    )>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    {
        let mut marker_query = queries.p0();

        for (ring, mut transform) in marker_query.iter_mut() {
            let Some(parent_position) = body_visual_position(ring.parent_body_id, days_since_j2000)
            else {
                continue;
            };

            let angle = std::f32::consts::TAU * ring.index as f32 / ring.total as f32;

            let ring_position = Vec3::new(
                angle.cos() * ring.ring_radius,
                0.0,
                angle.sin() * ring.ring_radius * SATURN_RING_MESH_Y_SCALE,
            );

            transform.translation = parent_position + ring_position;
        }
    }

    {
        let mut mesh_query = queries.p1();

        for (ring_mesh, mut transform) in mesh_query.iter_mut() {
            let Some(parent_position) =
                body_visual_position(ring_mesh.parent_body_id, days_since_j2000)
            else {
                continue;
            };

            transform.translation = parent_position;
        }
    }
}
