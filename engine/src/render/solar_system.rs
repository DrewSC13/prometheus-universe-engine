use bevy::math::primitives::Sphere;
use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};
use crate::interaction::selection::SelectedBody;
use crate::simulation::bodies::{
    body_axial_tilt_degrees, body_rotation_angle_radians, BodyClass, BodyId,
    CelestialBodyDefinition, SOLAR_SYSTEM_BODIES,
};
use crate::simulation::catalog::{
    body_definition, body_position_meters, solar_system_runtime_state,
};
use crate::time::SimulationClock;

const SPACE_AMBIENT_BRIGHTNESS: f32 = 0.018;
const SELECTED_BODY_INDICATOR_RADIUS_FACTOR: f32 = 1.28;
const SELECTED_BODY_INDICATOR_MIN_PADDING: f32 = 0.38;

mod earth;
mod labels;
mod orbits;
mod planet_surface;
mod saturn;
mod starfield;
mod sun;

use self::earth::*;
pub use self::labels::LabelVisibilityMode;
use self::labels::{
    apply_label_visibility, keyboard_label_controls, label_color, label_font_size, spawn_label,
    update_solar_body_labels,
};
use self::orbits::*;
use self::planet_surface::{
    spawn_planet_surface_detail_layer, update_planet_band_markers, update_planet_surface_features,
};
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

#[derive(Component, Debug)]
pub struct SelectedBodyIndicatorVisual;

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
                    update_selected_body_indicator,
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

    let selected_body_indicator_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.35, 0.85, 1.0, 0.22),
        emissive: LinearRgba::rgb(0.12, 0.42, 0.75),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    spawn_starfield(&mut commands, small_sphere.clone(), &starfield_materials);
    spawn_selected_body_indicator(
        &mut commands,
        sphere.clone(),
        selected_body_indicator_material,
    );

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

fn spawn_selected_body_indicator(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) {
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_scale(Vec3::ZERO),
        Visibility::Hidden,
        SelectedBodyIndicatorVisual,
    ));
}

fn update_selected_body_indicator(
    simulation_clock: Res<SimulationClock>,
    selected_body: Option<Res<SelectedBody>>,
    mut query: Query<(&mut Transform, &mut Visibility), With<SelectedBodyIndicatorVisual>>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let selected_body_id = selected_body.as_deref().and_then(|selection| selection.id);

    let Some(body_id) = selected_body_id else {
        hide_selected_body_indicators(&mut query);
        return;
    };

    let Some(body) = body_definition(body_id) else {
        hide_selected_body_indicators(&mut query);
        return;
    };

    let Some(position) = body_visual_position(body_id, days_since_j2000) else {
        hide_selected_body_indicators(&mut query);
        return;
    };

    for (mut transform, mut visibility) in query.iter_mut() {
        transform.translation = position;
        transform.scale = Vec3::splat(selected_body_indicator_scale(body.visual_radius));
        *visibility = Visibility::Visible;
    }
}

fn hide_selected_body_indicators(
    query: &mut Query<(&mut Transform, &mut Visibility), With<SelectedBodyIndicatorVisual>>,
) {
    for (mut transform, mut visibility) in query.iter_mut() {
        transform.scale = Vec3::ZERO;
        *visibility = Visibility::Hidden;
    }
}

fn selected_body_indicator_scale(body_visual_radius: f32) -> f32 {
    (body_visual_radius * SELECTED_BODY_INDICATOR_RADIUS_FACTOR)
        .max(body_visual_radius + SELECTED_BODY_INDICATOR_MIN_PADDING)
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
            transform.rotation = axial_body_rotation(body_visual.id, days_since_j2000);
        }
    }
}

fn axial_body_rotation(id: BodyId, days_since_j2000: f64) -> Quat {
    axial_tilt_rotation(id)
        * Quat::from_rotation_y(body_rotation_angle_radians(id, days_since_j2000))
}

fn axial_tilt_rotation(id: BodyId) -> Quat {
    Quat::from_rotation_z(body_axial_tilt_degrees(id).to_radians())
}

fn has_ring_visual(id: BodyId) -> bool {
    matches!(id, BodyId::Saturn)
}

pub fn solar_body_visual_position(id: BodyId, days_since_j2000: f64) -> Option<Vec3> {
    body_visual_position(id, days_since_j2000)
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
