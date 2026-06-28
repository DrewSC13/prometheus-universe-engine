use super::{body_visual_position, deterministic_noise, spherical_fibonacci_direction};

use crate::simulation::bodies::{BodyId, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::math::primitives::Sphere;
use bevy::prelude::*;

pub(super) const SOLAR_POINT_LIGHT_INTENSITY: f32 = 18_000_000.0;
pub(super) const SOLAR_POINT_LIGHT_RANGE: f32 = 260.0;
pub(super) const SOLAR_POINT_LIGHT_RADIUS: f32 = 12.0;
pub(super) const REAL_SOLAR_HALO_LAYER_COUNT: usize = 5;
pub(super) const REAL_SOLAR_HALO_RADIUS_FACTORS: [f32; REAL_SOLAR_HALO_LAYER_COUNT] =
    [1.08, 1.22, 1.42, 1.68, 2.05];
pub(super) const REAL_SOLAR_HALO_ALPHA_VALUES: [f32; REAL_SOLAR_HALO_LAYER_COUNT] =
    [0.016, 0.008, 0.0035, 0.0012, 0.0003];
pub(super) const REAL_SOLAR_LIGHT_INTENSITY: f32 = 90_000.0;
pub(super) const REAL_SOLAR_LIGHT_RANGE: f32 = 420.0;
pub(super) const SOLAR_SURFACE_FEATURE_COUNT: usize = 220;
pub(super) const SOLAR_SURFACE_RADIUS_FACTOR: f32 = 1.018;
pub(super) const SOLAR_SURFACE_MIN_SCALE: f32 = 0.055;
pub(super) const SOLAR_SURFACE_MAX_SCALE: f32 = 0.135;
pub(super) const SOLAR_CORONA_MARKERS_PER_SHELL: usize = 260;
pub(super) const SOLAR_CORONA_INNER_RADIUS_FACTOR: f32 = 1.28;
pub(super) const SOLAR_CORONA_OUTER_RADIUS_FACTOR: f32 = 1.86;
pub(super) const SOLAR_CORONA_INNER_SCALE: f32 = 0.080;
pub(super) const SOLAR_CORONA_OUTER_SCALE: f32 = 0.055;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct SolarSurfaceFeatureVisual {
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct SolarCoronaMarkerVisual {
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

#[derive(Component, Debug)]
pub(super) struct RealSolarHaloLayer {
    pub layer: usize,
}

#[derive(Component, Debug)]
pub(super) struct RealSolarHaloLight;

pub(super) fn spawn_solar_point_light(commands: &mut Commands) {
    commands.spawn((
        PointLight {
            intensity: SOLAR_POINT_LIGHT_INTENSITY,
            range: SOLAR_POINT_LIGHT_RANGE,
            radius: SOLAR_POINT_LIGHT_RADIUS,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Solar Point Light"),
    ));
}

pub(super) fn spawn_solar_surface_features(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &[Handle<StandardMaterial>; 3],
    sun_visual_radius: f32,
) {
    for index in 0..SOLAR_SURFACE_FEATURE_COUNT {
        let direction = solar_surface_direction(index, 0.0);
        let position = direction * sun_visual_radius * SOLAR_SURFACE_RADIUS_FACTOR;
        let scale = solar_surface_feature_scale(index);
        let material = materials[solar_surface_material_index(index)].clone();

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            SolarSurfaceFeatureVisual {
                index,
                total: SOLAR_SURFACE_FEATURE_COUNT,
                radius: sun_visual_radius * SOLAR_SURFACE_RADIUS_FACTOR,
            },
        ));
    }
}

pub(super) fn spawn_solar_corona_markers(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &[Handle<StandardMaterial>; 2],
    sun_visual_radius: f32,
) {
    for (shell_index, radius_factor) in [
        SOLAR_CORONA_INNER_RADIUS_FACTOR,
        SOLAR_CORONA_OUTER_RADIUS_FACTOR,
    ]
    .iter()
    .enumerate()
    {
        let radius = sun_visual_radius * radius_factor;
        let scale = if shell_index == 0 {
            SOLAR_CORONA_INNER_SCALE
        } else {
            SOLAR_CORONA_OUTER_SCALE
        };

        for index in 0..SOLAR_CORONA_MARKERS_PER_SHELL {
            let direction = solar_corona_direction(index, shell_index, 0.0);
            let position = direction * radius;
            let material = materials[shell_index].clone();

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                SolarCoronaMarkerVisual {
                    index,
                    total: SOLAR_CORONA_MARKERS_PER_SHELL,
                    radius,
                },
            ));
        }
    }
}

pub(super) fn update_solar_surface_features(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&SolarSurfaceFeatureVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let phase = days_since_j2000 as f32 * 0.018;

    let Some(sun_position) = body_visual_position(BodyId::Sun, days_since_j2000) else {
        return;
    };

    for (feature, mut transform) in query.iter_mut() {
        let direction = solar_surface_direction(feature.index, phase);
        transform.translation = sun_position + direction * feature.radius;
    }
}

pub(super) fn update_solar_corona_markers(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&SolarCoronaMarkerVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let phase = days_since_j2000 as f32 * 0.010;

    let Some(sun_position) = body_visual_position(BodyId::Sun, days_since_j2000) else {
        return;
    };

    for (corona, mut transform) in query.iter_mut() {
        let shell_hint = if corona.radius < sun_visual_radius() * 1.5 {
            0
        } else {
            1
        };

        let direction = solar_corona_direction(corona.index, shell_hint, phase);
        transform.translation = sun_position + direction * corona.radius;
    }
}

pub(super) fn solar_surface_direction(index: usize, phase: f32) -> Vec3 {
    spherical_fibonacci_direction(index, SOLAR_SURFACE_FEATURE_COUNT, phase)
}

pub(super) fn solar_corona_direction(index: usize, shell_hint: usize, phase: f32) -> Vec3 {
    spherical_fibonacci_direction(
        index + shell_hint * 17,
        SOLAR_CORONA_MARKERS_PER_SHELL,
        phase * (1.0 + shell_hint as f32 * 0.35),
    )
}

pub(super) fn solar_surface_feature_scale(index: usize) -> f32 {
    let noise = deterministic_noise(index, 21.371);

    SOLAR_SURFACE_MIN_SCALE + (SOLAR_SURFACE_MAX_SCALE - SOLAR_SURFACE_MIN_SCALE) * noise
}

pub(super) fn solar_surface_material_index(index: usize) -> usize {
    match index % 7 {
        0 => 2,
        1 | 4 => 1,
        _ => 0,
    }
}

pub(super) fn spawn_real_solar_halo_glow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_radius = solar_catalog_visual_radius();

    for layer in 0..REAL_SOLAR_HALO_LAYER_COUNT {
        let radius = sun_radius * REAL_SOLAR_HALO_RADIUS_FACTORS[layer];
        let alpha = REAL_SOLAR_HALO_ALPHA_VALUES[layer];

        let mesh = meshes.add(Sphere::new(radius).mesh().uv(64, 32));
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.56, 0.12, alpha),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            emissive: LinearRgba::rgb(95.0, 52.0, 12.0),
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::ZERO),
            RealSolarHaloLayer { layer },
        ));
    }

    commands.spawn((
        PointLight {
            intensity: REAL_SOLAR_LIGHT_INTENSITY,
            range: REAL_SOLAR_LIGHT_RANGE,
            color: Color::srgb(1.0, 0.74, 0.34),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        RealSolarHaloLight,
    ));
}

fn solar_catalog_visual_radius() -> f32 {
    SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Sun)
        .map(|body| body.visual_radius)
        .unwrap_or(4.0)
}
