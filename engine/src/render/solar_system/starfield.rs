use super::deterministic_noise;

use bevy::prelude::*;

pub(super) const STARFIELD_STAR_COUNT: usize = 1800;
pub(super) const STARFIELD_RADIUS: f32 = 4_800.0;
pub(super) const STARFIELD_MIN_SCALE: f32 = 0.016;
pub(super) const STARFIELD_MAX_SCALE: f32 = 0.075;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct StarfieldStarVisual;

pub(super) fn spawn_starfield(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &[Handle<StandardMaterial>; 3],
) {
    for index in 0..STARFIELD_STAR_COUNT {
        let position = starfield_position(index);
        let scale = starfield_scale(index);
        let material = materials[starfield_material_index(index)].clone();

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            StarfieldStarVisual,
        ));
    }
}

pub(super) fn starfield_position(index: usize) -> Vec3 {
    let i = index as f32 + 0.5;

    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());
    let y = 1.0 - (i / STARFIELD_STAR_COUNT as f32) * 2.0;
    let radius = (1.0 - y * y).sqrt();
    let theta = golden_angle * i;

    Vec3::new(theta.cos() * radius, y, theta.sin() * radius) * STARFIELD_RADIUS
}

pub(super) fn starfield_scale(index: usize) -> f32 {
    let noise = deterministic_noise(index, 12.9898);

    STARFIELD_MIN_SCALE + (STARFIELD_MAX_SCALE - STARFIELD_MIN_SCALE) * noise
}

pub(super) fn starfield_material_index(index: usize) -> usize {
    match index % 11 {
        0 | 5 => 1,
        1 | 6 | 9 => 0,
        _ => 2,
    }
}
