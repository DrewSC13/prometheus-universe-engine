use super::{body_visual_position, deterministic_noise, spherical_fibonacci_direction};

use crate::simulation::bodies::{BodyId, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::math::primitives::Sphere;
use bevy::prelude::*;

pub(super) const EARTH_LANDMASS_SAMPLE_COUNT: usize = 384;
pub(super) const EARTH_LANDMASS_RADIUS_FACTOR: f32 = 1.019;
pub(super) const EARTH_LANDMASS_MIN_SCALE: f32 = 0.016;
pub(super) const EARTH_LANDMASS_MAX_SCALE: f32 = 0.050;
pub(super) const EARTH_LANDMASS_ROTATION_SPEED: f32 = 0.034;
pub(super) const EARTH_LANDMASS_CLUSTER_COUNT: usize = 7;
pub(super) const EARTH_LANDMASS_CLUSTER_SEEDS: [usize; EARTH_LANDMASS_CLUSTER_COUNT] =
    [7, 19, 42, 86, 133, 211, 307];
pub(super) const EARTH_LANDMASS_CLUSTER_THRESHOLDS: [f32; EARTH_LANDMASS_CLUSTER_COUNT] =
    [0.775, 0.790, 0.805, 0.782, 0.812, 0.795, 0.785];
pub(super) const EARTH_ATMOSPHERE_LAYER_COUNT: usize = 3;
pub(super) const EARTH_ATMOSPHERE_RADIUS_FACTORS: [f32; EARTH_ATMOSPHERE_LAYER_COUNT] =
    [1.045, 1.075, 1.115];
pub(super) const EARTH_ATMOSPHERE_ALPHA_VALUES: [f32; EARTH_ATMOSPHERE_LAYER_COUNT] =
    [0.115, 0.060, 0.026];
pub(super) const EARTH_CLOUD_FEATURE_COUNT: usize = 144;
pub(super) const EARTH_CLOUD_RADIUS_FACTOR: f32 = 1.026;
pub(super) const EARTH_CLOUD_MIN_SCALE: f32 = 0.010;
pub(super) const EARTH_CLOUD_MAX_SCALE: f32 = 0.030;
pub(super) const EARTH_CLOUD_ROTATION_SPEED: f32 = 0.052;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct EarthLandmassFeatureVisual {
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct EarthAtmosphereLayer {
    pub layer: usize,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct EarthCloudFeatureVisual {
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

pub(super) fn spawn_earth_atmosphere_and_clouds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth_radius = earth_catalog_visual_radius();

    for layer in 0..EARTH_ATMOSPHERE_LAYER_COUNT {
        let radius = earth_radius * EARTH_ATMOSPHERE_RADIUS_FACTORS[layer];
        let alpha = EARTH_ATMOSPHERE_ALPHA_VALUES[layer];

        let mesh = meshes.add(Sphere::new(radius).mesh().uv(48, 24));
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.22, 0.58, 1.0, alpha),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::ZERO),
            EarthAtmosphereLayer { layer },
            Name::new(format!("Earth Atmosphere Layer {}", layer)),
        ));
    }

    let cloud_mesh = meshes.add(Sphere::new(1.0).mesh().uv(16, 8));
    let cloud_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.96, 0.98, 1.0, 0.78),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for index in 0..EARTH_CLOUD_FEATURE_COUNT {
        let scale = earth_cloud_feature_scale(index, earth_radius);

        commands.spawn((
            Mesh3d(cloud_mesh.clone()),
            MeshMaterial3d(cloud_material.clone()),
            Transform::from_scale(Vec3::splat(scale)),
            EarthCloudFeatureVisual {
                index,
                total: EARTH_CLOUD_FEATURE_COUNT,
                radius: earth_radius * EARTH_CLOUD_RADIUS_FACTOR,
            },
            Name::new(format!("Earth Cloud Feature {}", index)),
        ));
    }
}

pub(super) fn update_earth_atmosphere_layers(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&EarthAtmosphereLayer, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    let Some(earth_position) = body_visual_position(BodyId::Earth, days_since_j2000) else {
        return;
    };

    for (layer, mut transform) in query.iter_mut() {
        let pulse = (days_since_j2000 as f32 * 0.018 + layer.layer as f32).sin() * 0.006;
        transform.translation = earth_position;
        transform.scale = Vec3::splat(1.0 + pulse);
    }
}

pub(super) fn update_earth_cloud_features(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&EarthCloudFeatureVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    let Some(earth_position) = body_visual_position(BodyId::Earth, days_since_j2000) else {
        return;
    };

    let phase = days_since_j2000 as f32 * EARTH_CLOUD_ROTATION_SPEED;

    for (cloud, mut transform) in query.iter_mut() {
        let direction = earth_cloud_direction(cloud.index, cloud.total, phase);
        transform.translation = earth_position + direction * cloud.radius;
    }
}

fn earth_catalog_visual_radius() -> f32 {
    SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Earth)
        .map(|body| body.visual_radius)
        .unwrap_or(0.65)
}

pub(super) fn earth_cloud_direction(index: usize, total: usize, phase: f32) -> Vec3 {
    spherical_fibonacci_direction(index, total, phase + 1.337)
}

pub(super) fn earth_cloud_feature_scale(index: usize, earth_visual_radius: f32) -> f32 {
    let noise = deterministic_noise(index, 42.4242);
    let base = EARTH_CLOUD_MIN_SCALE + (EARTH_CLOUD_MAX_SCALE - EARTH_CLOUD_MIN_SCALE) * noise;

    base * earth_visual_radius.clamp(0.55, 1.25).sqrt()
}

pub(super) fn spawn_earth_surface_landmasses(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth_radius = earth_catalog_visual_radius();

    let vegetation_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.16, 0.50, 0.24, 0.88),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let arid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.62, 0.46, 0.23, 0.82),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let land_mesh = meshes.add(Sphere::new(1.0).mesh().uv(12, 6));

    for index in 0..EARTH_LANDMASS_SAMPLE_COUNT {
        let direction = earth_landmass_direction(index, EARTH_LANDMASS_SAMPLE_COUNT, 0.0);
        let strength = earth_landmass_cluster_strength(direction);

        if strength <= 0.0 {
            continue;
        }

        let scale = earth_landmass_feature_scale(index, earth_radius, strength);
        let biome_noise = deterministic_noise(index, 83.771);
        let material = if biome_noise > 0.64 {
            arid_material.clone()
        } else {
            vegetation_material.clone()
        };

        commands.spawn((
            Mesh3d(land_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_scale(Vec3::splat(scale)),
            EarthLandmassFeatureVisual {
                index,
                total: EARTH_LANDMASS_SAMPLE_COUNT,
                radius: earth_radius * EARTH_LANDMASS_RADIUS_FACTOR,
            },
            Name::new(format!("Earth Landmass Feature {}", index)),
        ));
    }
}

pub(super) fn update_earth_surface_landmasses(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&EarthLandmassFeatureVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    let Some(earth_position) = body_visual_position(BodyId::Earth, days_since_j2000) else {
        return;
    };

    let phase = days_since_j2000 as f32 * EARTH_LANDMASS_ROTATION_SPEED;

    for (landmass, mut transform) in query.iter_mut() {
        let direction = earth_landmass_direction(landmass.index, landmass.total, phase);
        transform.translation = earth_position + direction * landmass.radius;
    }
}

pub(super) fn earth_landmass_direction(index: usize, total: usize, phase: f32) -> Vec3 {
    spherical_fibonacci_direction(index, total, phase + 0.618)
}

pub(super) fn earth_landmass_cluster_strength(direction: Vec3) -> f32 {
    let mut strongest = 0.0_f32;

    for cluster_index in 0..EARTH_LANDMASS_CLUSTER_COUNT {
        let seed = EARTH_LANDMASS_CLUSTER_SEEDS[cluster_index];
        let threshold = EARTH_LANDMASS_CLUSTER_THRESHOLDS[cluster_index];
        let center = spherical_fibonacci_direction(
            seed,
            EARTH_LANDMASS_SAMPLE_COUNT,
            0.271 + cluster_index as f32 * 0.173,
        );

        let dot = direction.dot(center);

        if dot > threshold {
            let strength = ((dot - threshold) / (1.0 - threshold)).clamp(0.0, 1.0);
            strongest = strongest.max(strength);
        }
    }

    strongest
}

pub(super) fn earth_landmass_feature_scale(
    index: usize,
    earth_visual_radius: f32,
    strength: f32,
) -> f32 {
    let noise = deterministic_noise(index, 121.818);
    let base =
        EARTH_LANDMASS_MIN_SCALE + (EARTH_LANDMASS_MAX_SCALE - EARTH_LANDMASS_MIN_SCALE) * noise;

    base * (0.45 + strength * 0.75) * earth_visual_radius.clamp(0.55, 1.35).sqrt()
}

#[cfg(test)]
pub(super) fn earth_landmass_visible_sample_count() -> usize {
    (0..EARTH_LANDMASS_SAMPLE_COUNT)
        .filter(|index| {
            let direction = earth_landmass_direction(*index, EARTH_LANDMASS_SAMPLE_COUNT, 0.0);
            earth_landmass_cluster_strength(direction) > 0.0
        })
        .count()
}
