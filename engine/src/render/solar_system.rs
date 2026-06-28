use bevy::math::primitives::Sphere;
use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};
use crate::simulation::bodies::{BodyClass, BodyId, CelestialBodyDefinition, SOLAR_SYSTEM_BODIES};
use crate::simulation::catalog::{body_position_meters, solar_system_runtime_state};
use crate::time::SimulationClock;

const SPACE_AMBIENT_BRIGHTNESS: f32 = 0.018;

const PLANET_SURFACE_FEATURE_COUNT: usize = 96;
const PLANET_SURFACE_RADIUS_FACTOR: f32 = 1.012;
const PLANET_SURFACE_MIN_SCALE: f32 = 0.010;
const PLANET_SURFACE_MAX_SCALE: f32 = 0.034;

const PLANET_BAND_MARKERS: usize = 168;
const PLANET_BAND_MARKER_RADIUS: f32 = 0.034;
const JUPITER_BAND_Y_FACTORS: [f32; 7] = [-0.54, -0.36, -0.18, 0.0, 0.18, 0.36, 0.54];
const SATURN_BAND_Y_FACTORS: [f32; 5] = [-0.42, -0.21, 0.0, 0.21, 0.42];

#[derive(Component, Debug, Clone, Copy)]
pub struct PlanetSurfaceFeatureVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
    pub radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PlanetBandMarkerVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
    pub band_y: f32,
    pub band_radius: f32,
}

mod earth;
mod labels;
mod orbits;
mod saturn;
mod starfield;
mod sun;

use self::earth::*;
use self::labels::*;
use self::orbits::*;
use self::saturn::*;
use self::starfield::*;
use self::sun::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrbitVisibilityMode {
    All,
    PlanetaryOnly,
    None,
}

impl Default for OrbitVisibilityMode {
    fn default() -> Self {
        Self::All
    }
}

impl OrbitVisibilityMode {
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::PlanetaryOnly,
            Self::PlanetaryOnly => Self::None,
            Self::None => Self::All,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::PlanetaryOnly => "planetary",
            Self::None => "none",
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarBodyVisual {
    pub id: BodyId,
}

pub struct SolarSystemRenderPlugin;

impl Plugin for SolarSystemRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_earth_surface_landmasses)
            .add_systems(Update, update_earth_surface_landmasses);

        app.add_systems(Startup, spawn_earth_atmosphere_and_clouds)
            .add_systems(
                Update,
                (update_earth_atmosphere_layers, update_earth_cloud_features),
            );

        app.insert_resource(AmbientLight {
            color: Color::srgb(0.04, 0.045, 0.055),
            brightness: SPACE_AMBIENT_BRIGHTNESS,
            ..default()
        });
        app.add_systems(Startup, spawn_planet_surface_detail_layer)
            .add_systems(Startup, spawn_real_solar_halo_glow)
            .add_systems(
                Update,
                (update_planet_surface_features, update_planet_band_markers),
            );

        app.insert_resource(LabelVisibilityMode::default())
            .insert_resource(OrbitVisibilityMode::default())
            .add_systems(Startup, spawn_solar_system_visuals)
            .add_systems(
                Update,
                (
                    keyboard_label_controls,
                    keyboard_orbit_controls,
                    update_solar_system_visuals,
                    update_orbit_markers,
                    update_ring_markers,
                    update_solar_surface_features,
                    update_solar_corona_markers,
                    update_solar_body_labels,
                    apply_label_visibility,
                    apply_orbit_visibility,
                ),
            );
    }
}

fn spawn_solar_system_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ClearColor(Color::srgb(0.002, 0.004, 0.012)));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.55, 0.62, 0.85),
        brightness: 0.055,
        ..default()
    });

    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(32, 18));
    let small_sphere = meshes.add(Sphere::new(1.0).mesh().uv(12, 8));

    let planet_orbit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.42, 0.95),
        emissive: LinearRgba::rgb(0.018, 0.040, 0.105),
        ..default()
    });

    let satellite_orbit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.58, 0.58, 0.58),
        emissive: LinearRgba::rgb(0.045, 0.045, 0.045),
        ..default()
    });

    let saturn_ring_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.86, 0.74, 0.54),
        emissive: LinearRgba::rgb(0.070, 0.055, 0.035),
        ..default()
    });

    let starfield_materials = [
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.80, 1.0),
            emissive: LinearRgba::rgb(0.42, 0.50, 0.88),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.93, 0.78),
            emissive: LinearRgba::rgb(0.55, 0.48, 0.32),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.97, 1.0),
            emissive: LinearRgba::rgb(0.68, 0.72, 0.95),
            ..default()
        }),
    ];

    let solar_surface_materials = [
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.68, 0.14),
            emissive: LinearRgba::rgb(1.35, 0.58, 0.10),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.88, 0.34),
            emissive: LinearRgba::rgb(1.60, 0.88, 0.18),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.32, 0.06),
            emissive: LinearRgba::rgb(0.90, 0.20, 0.035),
            ..default()
        }),
    ];

    let solar_corona_materials = [
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.68, 0.18),
            emissive: LinearRgba::rgb(1.75, 0.62, 0.12),
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.86, 0.42),
            emissive: LinearRgba::rgb(1.20, 0.72, 0.22),
            ..default()
        }),
    ];

    spawn_starfield(&mut commands, small_sphere.clone(), &starfield_materials);

    for body in SOLAR_SYSTEM_BODIES.iter() {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(body.color_srgb[0], body.color_srgb[1], body.color_srgb[2]),
            emissive: body_emissive_color(body),
            ..default()
        });

        commands.spawn((
            Mesh3d(sphere.clone()),
            MeshMaterial3d(material),
            Transform::from_scale(Vec3::splat(body.visual_radius)),
            SolarBodyVisual { id: body.id },
            GlobalPositionComponent {
                position: GlobalPosition::ZERO,
            },
        ));

        if body.id == BodyId::Sun {
            spawn_solar_point_light(&mut commands);
            spawn_solar_surface_features(
                &mut commands,
                small_sphere.clone(),
                &solar_surface_materials,
                body.visual_radius,
            );

            spawn_solar_corona_markers(
                &mut commands,
                small_sphere.clone(),
                &solar_corona_materials,
                body.visual_radius,
            );
        }

        spawn_label(
            &mut commands,
            body.name,
            body.id,
            label_font_size(body),
            label_color(body),
        );

        if has_ring_visual(body.id) {
            spawn_ring_markers(
                &mut commands,
                &mut meshes,
                small_sphere.clone(),
                saturn_ring_material.clone(),
                body.id,
            );
        }

        if let Some(orbit) = body.orbit {
            spawn_orbit_markers(
                &mut commands,
                small_sphere.clone(),
                planet_orbit_material.clone(),
                satellite_orbit_material.clone(),
                body.id,
                orbit,
            );
        }
    }

    commands.spawn((
        PointLight {
            intensity: 28_000_000.0,
            range: 1_200.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn body_emissive_color(body: &CelestialBodyDefinition) -> LinearRgba {
    match body.class {
        BodyClass::Star => LinearRgba::rgb(2.25, 1.05, 0.22),
        _ => LinearRgba::BLACK,
    }
}

fn update_solar_system_visuals(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(
        &SolarBodyVisual,
        &mut GlobalPositionComponent,
        &mut Transform,
    )>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let state = solar_system_runtime_state(days_since_j2000);

    for (body_visual, mut global_position, mut transform) in query.iter_mut() {
        let Some(runtime_body) = state.iter().find(|body| body.id == body_visual.id) else {
            continue;
        };

        global_position.position = GlobalPosition {
            meters_from_origin: runtime_body.physical_position_meters,
        };

        if let Some(visual_position) = body_visual_position(body_visual.id, days_since_j2000) {
            transform.translation = visual_position;
        }
    }
}

fn has_ring_visual(id: BodyId) -> bool {
    matches!(id, BodyId::Saturn)
}

fn body_visual_position(id: BodyId, days_since_j2000: f64) -> Option<Vec3> {
    let body = SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == id)?;

    match body.orbit {
        Some(orbit) => {
            let parent_visual_position = body_visual_position(orbit.parent, days_since_j2000)?;

            let body_physical_position = body_position_meters(id, days_since_j2000)?;
            let parent_physical_position = body_position_meters(orbit.parent, days_since_j2000)?;

            let physical_direction =
                (body_physical_position - parent_physical_position).normalize_or_zero();

            Some(
                parent_visual_position
                    + physical_direction.as_vec3() * educational_orbit_radius(orbit),
            )
        }
        None => Some(Vec3::ZERO),
    }
}

fn deterministic_noise(index: usize, seed: f32) -> f32 {
    ((index as f32 * seed).sin() * 43_758.547).fract().abs()
}

fn spherical_fibonacci_direction(index: usize, total: usize, phase: f32) -> Vec3 {
    let i = index as f32 + 0.5;
    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());

    let y = 1.0 - (i / total as f32) * 2.0;
    let radius = (1.0 - y * y).sqrt();
    let theta = golden_angle * i + phase;

    Vec3::new(theta.cos() * radius, y, theta.sin() * radius).normalize()
}

fn sun_visual_radius() -> f32 {
    SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Sun)
        .map(|body| body.visual_radius)
        .unwrap_or(3.5)
}

fn spawn_planet_surface_detail_layer(
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

fn update_planet_surface_features(
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

fn update_planet_band_markers(
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

fn is_planetary_detail_body(id: BodyId) -> bool {
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

fn has_planet_band_visual(id: BodyId) -> bool {
    matches!(id, BodyId::Jupiter | BodyId::Saturn)
}

fn planet_surface_feature_count(id: BodyId) -> usize {
    match id {
        BodyId::Jupiter | BodyId::Saturn => PLANET_SURFACE_FEATURE_COUNT + 44,
        BodyId::Uranus | BodyId::Neptune => PLANET_SURFACE_FEATURE_COUNT,
        BodyId::Earth | BodyId::Mars | BodyId::Venus => PLANET_SURFACE_FEATURE_COUNT - 18,
        BodyId::Mercury => PLANET_SURFACE_FEATURE_COUNT - 30,
        _ => PLANET_SURFACE_FEATURE_COUNT,
    }
}

fn planet_surface_rotation_speed(id: BodyId) -> f32 {
    match id {
        BodyId::Jupiter => 0.070,
        BodyId::Saturn => 0.055,
        BodyId::Earth => 0.040,
        BodyId::Mars => 0.034,
        BodyId::Uranus | BodyId::Neptune => 0.026,
        _ => 0.018,
    }
}

fn planet_surface_direction(index: usize, total: usize, id: BodyId, phase: f32) -> Vec3 {
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

fn planet_surface_feature_scale(index: usize, body_visual_radius: f32) -> f32 {
    let noise = deterministic_noise(index, 31.171);
    let base =
        PLANET_SURFACE_MIN_SCALE + (PLANET_SURFACE_MAX_SCALE - PLANET_SURFACE_MIN_SCALE) * noise;

    base * body_visual_radius.clamp(0.55, 2.25).sqrt()
}

fn planet_surface_material_index(id: BodyId, index: usize) -> usize {
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

fn planet_band_y_factors(id: BodyId) -> Option<&'static [f32]> {
    match id {
        BodyId::Jupiter => Some(&JUPITER_BAND_Y_FACTORS),
        BodyId::Saturn => Some(&SATURN_BAND_Y_FACTORS),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod real_solar_halo_glow_tests;

#[cfg(test)]
mod earth_atmosphere_cloud_tests;

#[cfg(test)]
mod earth_landmass_tests;

#[cfg(test)]
mod saturn_ring_mesh_tests;
