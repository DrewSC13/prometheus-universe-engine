use super::{body_visual_position, deterministic_noise, spherical_fibonacci_direction};

use crate::simulation::bodies::{BodyId, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::prelude::*;

pub(super) const PLANET_SURFACE_FEATURE_COUNT: usize = 96;
pub(super) const PLANET_SURFACE_RADIUS_FACTOR: f32 = 1.012;
pub(super) const PLANET_SURFACE_MIN_SCALE: f32 = 0.010;
pub(super) const PLANET_SURFACE_MAX_SCALE: f32 = 0.034;
pub(super) const PLANET_BAND_MARKERS: usize = 168;
pub(super) const PLANET_BAND_MARKER_RADIUS: f32 = 0.034;
pub(super) const JUPITER_BAND_Y_FACTORS: [f32; 7] = [-0.54, -0.36, -0.18, 0.0, 0.18, 0.36, 0.54];
pub(super) const SATURN_BAND_Y_FACTORS: [f32; 5] = [-0.42, -0.21, 0.0, 0.21, 0.42];

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct PlanetSurfaceFeatureVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct PlanetBandMarkerVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
    pub band_y: f32,
    pub band_radius: f32,
}

pub(super) fn spawn_planet_surface_detail_layer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let detail_mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 18));

    let planet_detail_materials = [
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.34, 0.95),
            emissive: LinearRgba::rgb(0.005, 0.012, 0.030),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.92, 0.84),
            emissive: LinearRgba::rgb(0.018, 0.018, 0.014),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.22, 0.08),
            emissive: LinearRgba::rgb(0.020, 0.005, 0.002),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.50, 0.32),
            emissive: LinearRgba::rgb(0.014, 0.009, 0.004),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.46, 0.88, 1.0),
            emissive: LinearRgba::rgb(0.008, 0.026, 0.040),
            ..default()
        }),
    ];

    let jupiter_band_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.78, 0.52),
        emissive: LinearRgba::rgb(0.030, 0.018, 0.008),
        ..default()
    });

    let jupiter_dark_band_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.48, 0.27, 0.14),
        emissive: LinearRgba::rgb(0.012, 0.006, 0.002),
        ..default()
    });

    let saturn_band_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.72, 0.46),
        emissive: LinearRgba::rgb(0.020, 0.014, 0.006),
        ..default()
    });

    let saturn_dark_band_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.52, 0.40, 0.24),
        emissive: LinearRgba::rgb(0.010, 0.007, 0.003),
        ..default()
    });

    for body in SOLAR_SYSTEM_BODIES.iter() {
        if is_planetary_detail_body(body.id) {
            let feature_count = planet_surface_feature_count(body.id);

            for index in 0..feature_count {
                let scale = planet_surface_feature_scale(index, body.visual_radius);
                let material =
                    planet_detail_materials[planet_surface_material_index(body.id, index)].clone();

                commands.spawn((
                    Mesh3d(detail_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_scale(Vec3::splat(scale)),
                    PlanetSurfaceFeatureVisual {
                        body_id: body.id,
                        index,
                        total: feature_count,
                        radius: body.visual_radius * PLANET_SURFACE_RADIUS_FACTOR,
                    },
                ));
            }
        }

        if has_planet_band_visual(body.id) {
            let Some(band_factors) = planet_band_y_factors(body.id) else {
                continue;
            };

            for (band_index, y_factor) in band_factors.iter().enumerate() {
                let band_y = body.visual_radius * y_factor;
                let band_radius = (body.visual_radius.powi(2) - band_y.powi(2)).sqrt() * 1.045;

                let material = match body.id {
                    BodyId::Jupiter => {
                        if band_index % 2 == 0 {
                            jupiter_band_material.clone()
                        } else {
                            jupiter_dark_band_material.clone()
                        }
                    }
                    BodyId::Saturn => {
                        if band_index % 2 == 0 {
                            saturn_band_material.clone()
                        } else {
                            saturn_dark_band_material.clone()
                        }
                    }
                    _ => jupiter_band_material.clone(),
                };

                for index in 0..PLANET_BAND_MARKERS {
                    commands.spawn((
                        Mesh3d(detail_mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_scale(Vec3::splat(PLANET_BAND_MARKER_RADIUS)),
                        PlanetBandMarkerVisual {
                            body_id: body.id,
                            index,
                            total: PLANET_BAND_MARKERS,
                            band_y,
                            band_radius,
                        },
                    ));
                }
            }
        }
    }
}

pub(super) fn update_planet_surface_features(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&PlanetSurfaceFeatureVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    for (feature, mut transform) in query.iter_mut() {
        let Some(body_position) = body_visual_position(feature.body_id, days_since_j2000) else {
            continue;
        };

        let phase = days_since_j2000 as f32 * planet_surface_rotation_speed(feature.body_id);
        let direction =
            planet_surface_direction(feature.index, feature.total, feature.body_id, phase);

        transform.translation = body_position + direction * feature.radius;
    }
}

pub(super) fn update_planet_band_markers(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&PlanetBandMarkerVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    for (band, mut transform) in query.iter_mut() {
        let Some(body_position) = body_visual_position(band.body_id, days_since_j2000) else {
            continue;
        };

        let phase = days_since_j2000 as f32 * planet_surface_rotation_speed(band.body_id) * 1.6;
        let angle = std::f32::consts::TAU * band.index as f32 / band.total as f32 + phase;

        let local_position = Vec3::new(
            angle.cos() * band.band_radius,
            band.band_y,
            angle.sin() * band.band_radius * 0.94,
        );

        transform.translation = body_position + local_position;
    }
}

pub(super) fn is_planetary_detail_body(id: BodyId) -> bool {
    matches!(
        id,
        BodyId::Mercury
            | BodyId::Venus
            | BodyId::Earth
            | BodyId::Mars
            | BodyId::Jupiter
            | BodyId::Saturn
            | BodyId::Uranus
            | BodyId::Neptune
    )
}

pub(super) fn has_planet_band_visual(id: BodyId) -> bool {
    matches!(id, BodyId::Jupiter | BodyId::Saturn)
}

pub(super) fn planet_surface_feature_count(id: BodyId) -> usize {
    match id {
        BodyId::Jupiter | BodyId::Saturn => PLANET_SURFACE_FEATURE_COUNT + 44,
        BodyId::Uranus | BodyId::Neptune => PLANET_SURFACE_FEATURE_COUNT,
        BodyId::Earth | BodyId::Mars | BodyId::Venus => PLANET_SURFACE_FEATURE_COUNT - 18,
        BodyId::Mercury => PLANET_SURFACE_FEATURE_COUNT - 30,
        _ => PLANET_SURFACE_FEATURE_COUNT,
    }
}

pub(super) fn planet_surface_rotation_speed(id: BodyId) -> f32 {
    match id {
        BodyId::Jupiter => 0.070,
        BodyId::Saturn => 0.055,
        BodyId::Earth => 0.040,
        BodyId::Mars => 0.034,
        BodyId::Uranus | BodyId::Neptune => 0.026,
        _ => 0.018,
    }
}

pub(super) fn planet_surface_direction(index: usize, total: usize, id: BodyId, phase: f32) -> Vec3 {
    let id_offset = match id {
        BodyId::Mercury => 0.1,
        BodyId::Venus => 0.7,
        BodyId::Earth => 1.2,
        BodyId::Mars => 1.8,
        BodyId::Jupiter => 2.4,
        BodyId::Saturn => 3.0,
        BodyId::Uranus => 3.6,
        BodyId::Neptune => 4.2,
        _ => 0.0,
    };

    spherical_fibonacci_direction(index, total, phase + id_offset)
}

pub(super) fn planet_surface_feature_scale(index: usize, body_visual_radius: f32) -> f32 {
    let noise = deterministic_noise(index, 31.171);
    let base =
        PLANET_SURFACE_MIN_SCALE + (PLANET_SURFACE_MAX_SCALE - PLANET_SURFACE_MIN_SCALE) * noise;

    base * body_visual_radius.clamp(0.55, 2.25).sqrt()
}

pub(super) fn planet_surface_material_index(id: BodyId, index: usize) -> usize {
    match id {
        BodyId::Earth => match index % 6 {
            0 | 3 => 0,
            1 => 1,
            2 => 4,
            _ => 0,
        },
        BodyId::Mars => 2,
        BodyId::Jupiter | BodyId::Saturn => {
            if index % 4 == 0 {
                1
            } else {
                3
            }
        }
        BodyId::Uranus | BodyId::Neptune => 4,
        BodyId::Venus => 3,
        BodyId::Mercury => 1,
        _ => 1,
    }
}

pub(super) fn planet_band_y_factors(id: BodyId) -> Option<&'static [f32]> {
    match id {
        BodyId::Jupiter => Some(&JUPITER_BAND_Y_FACTORS),
        BodyId::Saturn => Some(&SATURN_BAND_Y_FACTORS),
        _ => None,
    }
}
