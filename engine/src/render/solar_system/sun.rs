use super::{
    axial_tilt_rotation, body_visual_position, deterministic_noise, spherical_fibonacci_direction,
    sun_visual_radius,
};

use crate::simulation::bodies::{body_rotation_angle_radians, BodyId, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::math::primitives::Sphere;
use bevy::prelude::*;

pub(super) const SOLAR_POINT_LIGHT_INTENSITY: f32 = 24_000_000.0;
pub(super) const SOLAR_POINT_LIGHT_RANGE: f32 = 340.0;
pub(super) const SOLAR_POINT_LIGHT_RADIUS: f32 = 18.0;
pub(super) const REAL_SOLAR_HALO_LAYER_COUNT: usize = 6;
pub(super) const REAL_SOLAR_HALO_RADIUS_FACTORS: [f32; REAL_SOLAR_HALO_LAYER_COUNT] =
    [1.015, 1.07, 1.16, 1.34, 1.66, 2.08];
pub(super) const REAL_SOLAR_HALO_ALPHA_VALUES: [f32; REAL_SOLAR_HALO_LAYER_COUNT] =
    [0.060, 0.036, 0.018, 0.007, 0.0022, 0.0006];
pub(super) const REAL_SOLAR_LIGHT_INTENSITY: f32 = 135_000.0;
pub(super) const REAL_SOLAR_LIGHT_RANGE: f32 = 560.0;
pub(super) const SOLAR_SURFACE_FEATURE_COUNT: usize = 620;
pub(super) const SOLAR_SURFACE_RADIUS_FACTOR: f32 = 1.009;
pub(super) const SOLAR_SURFACE_MIN_SCALE: f32 = 0.030;
pub(super) const SOLAR_SURFACE_MAX_SCALE: f32 = 0.115;
pub(super) const SOLAR_CORONA_MARKERS_PER_SHELL: usize = 220;
pub(super) const SOLAR_CORONA_INNER_RADIUS_FACTOR: f32 = 1.06;
pub(super) const SOLAR_CORONA_OUTER_RADIUS_FACTOR: f32 = 1.46;
pub(super) const SOLAR_CORONA_INNER_SCALE: f32 = 0.040;
pub(super) const SOLAR_CORONA_OUTER_SCALE: f32 = 0.024;
const SOLAR_SURFACE_PULSE_AMPLITUDE: f32 = 0.10;
const SOLAR_SURFACE_PULSE_SPEED: f32 = 2.4;
const SOLAR_CORONA_PULSE_AMPLITUDE: f32 = 0.13;
const SOLAR_CORONA_RADIAL_PULSE_AMPLITUDE: f32 = 0.028;
const SOLAR_CORONA_PULSE_SPEED: f32 = 1.35;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct SolarSurfaceFeatureVisual {
    pub index: usize,
    pub radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct SolarCoronaMarkerVisual {
    pub index: usize,
    pub radius: f32,
}

#[derive(Component, Debug)]
pub(super) struct RealSolarHaloLayer;

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
        let scale = solar_corona_marker_base_scale(shell_index);

        for index in 0..SOLAR_CORONA_MARKERS_PER_SHELL {
            let direction = solar_corona_direction(index, shell_index, 0.0);
            let position = direction * radius;
            let material = materials[shell_index].clone();

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_scale(Vec3::splat(scale)),
                SolarCoronaMarkerVisual { index, radius },
            ));
        }
    }
}

pub(super) fn update_solar_surface_features(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&SolarSurfaceFeatureVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let phase = body_rotation_angle_radians(BodyId::Sun, days_since_j2000);

    let Some(sun_position) = body_visual_position(BodyId::Sun, days_since_j2000) else {
        return;
    };

    for (feature, mut transform) in query.iter_mut() {
        let direction =
            axial_tilt_rotation(BodyId::Sun) * solar_surface_direction(feature.index, phase);
        transform.translation = sun_position + direction * feature.radius;
        transform.scale = Vec3::splat(solar_surface_feature_animated_scale(
            feature.index,
            days_since_j2000,
        ));
    }
}

pub(super) fn update_solar_corona_markers(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&SolarCoronaMarkerVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let phase = body_rotation_angle_radians(BodyId::Sun, days_since_j2000) * 0.55;

    let Some(sun_position) = body_visual_position(BodyId::Sun, days_since_j2000) else {
        return;
    };

    for (corona, mut transform) in query.iter_mut() {
        let shell_hint = if corona.radius < sun_visual_radius() * 1.5 {
            0
        } else {
            1
        };

        let direction = axial_tilt_rotation(BodyId::Sun)
            * solar_corona_direction(corona.index, shell_hint, phase);
        let radius_multiplier =
            solar_corona_radius_multiplier(corona.index, shell_hint, days_since_j2000);

        transform.translation = sun_position + direction * corona.radius * radius_multiplier;
        transform.scale = Vec3::splat(solar_corona_marker_animated_scale(
            corona.index,
            shell_hint,
            days_since_j2000,
        ));
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
    let base_scale =
        SOLAR_SURFACE_MIN_SCALE + (SOLAR_SURFACE_MAX_SCALE - SOLAR_SURFACE_MIN_SCALE) * noise;

    match solar_surface_material_index(index) {
        2 => base_scale * 1.55,
        1 => base_scale * 0.92,
        _ => base_scale,
    }
}

pub(super) fn solar_surface_feature_animated_scale(index: usize, days_since_j2000: f64) -> f32 {
    let base_scale = solar_surface_feature_scale(index);
    let phase = days_since_j2000 as f32 * SOLAR_SURFACE_PULSE_SPEED
        + deterministic_noise(index, 71.77) * std::f32::consts::TAU;

    base_scale * (1.0 + phase.sin() * SOLAR_SURFACE_PULSE_AMPLITUDE)
}

pub(super) fn solar_corona_marker_base_scale(shell_hint: usize) -> f32 {
    if shell_hint == 0 {
        SOLAR_CORONA_INNER_SCALE
    } else {
        SOLAR_CORONA_OUTER_SCALE
    }
}

pub(super) fn solar_corona_marker_animated_scale(
    index: usize,
    shell_hint: usize,
    days_since_j2000: f64,
) -> f32 {
    let base_scale = solar_corona_marker_base_scale(shell_hint);
    let phase = days_since_j2000 as f32 * SOLAR_CORONA_PULSE_SPEED
        + shell_hint as f32 * 1.73
        + deterministic_noise(index + shell_hint * 31, 91.113) * std::f32::consts::TAU;

    base_scale * (1.0 + phase.sin() * SOLAR_CORONA_PULSE_AMPLITUDE)
}

pub(super) fn solar_corona_radius_multiplier(
    index: usize,
    shell_hint: usize,
    days_since_j2000: f64,
) -> f32 {
    let phase = days_since_j2000 as f32 * SOLAR_CORONA_PULSE_SPEED
        + shell_hint as f32 * 0.87
        + deterministic_noise(index + shell_hint * 43, 37.917) * std::f32::consts::TAU;

    1.0 + phase.sin() * SOLAR_CORONA_RADIAL_PULSE_AMPLITUDE
}

pub(super) fn solar_surface_material_index(index: usize) -> usize {
    let dark_spot_noise = deterministic_noise(index, 113.913);
    let hot_cell_noise = deterministic_noise(index, 41.707);

    if dark_spot_noise > 0.895 {
        2
    } else if hot_cell_noise > 0.56 {
        1
    } else {
        0
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
            base_color: Color::srgba(1.0, 0.50, 0.08, alpha),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            emissive: LinearRgba::rgb(150.0, 76.0, 16.0),
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::ZERO),
            RealSolarHaloLayer,
        ));
    }

    commands.spawn((
        PointLight {
            intensity: REAL_SOLAR_LIGHT_INTENSITY,
            range: REAL_SOLAR_LIGHT_RANGE,
            color: Color::srgb(1.0, 0.58, 0.22),
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
