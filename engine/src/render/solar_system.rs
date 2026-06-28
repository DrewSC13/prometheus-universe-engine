use bevy::math::primitives::Sphere;
use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};
use crate::simulation::bodies::{
    BodyClass, BodyId, CelestialBodyDefinition, OrbitDefinition, SOLAR_SYSTEM_BODIES,
};
use crate::simulation::catalog::{body_position_meters, solar_system_runtime_state};
use crate::time::SimulationClock;

const AU_METERS: f64 = 149_597_870_700.0;

const PLANET_ORBIT_MARKERS: usize = 128;
const SATELLITE_ORBIT_MARKERS: usize = 64;

const PLANET_ORBIT_MARKER_RADIUS: f32 = 0.025;
const SATELLITE_ORBIT_MARKER_RADIUS: f32 = 0.020;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelVisibilityMode {
    MajorOnly,
    All,
    None,
}

impl Default for LabelVisibilityMode {
    fn default() -> Self {
        Self::MajorOnly
    }
}

impl LabelVisibilityMode {
    pub fn next(self) -> Self {
        match self {
            Self::MajorOnly => Self::All,
            Self::All => Self::None,
            Self::None => Self::MajorOnly,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MajorOnly => "major",
            Self::All => "all",
            Self::None => "none",
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarBodyVisual {
    pub id: BodyId,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarBodyLabel {
    pub id: BodyId,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct OrbitMarkerVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
}

pub struct SolarSystemRenderPlugin;

impl Plugin for SolarSystemRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LabelVisibilityMode::default())
            .add_systems(Startup, spawn_solar_system_visuals)
            .add_systems(
                Update,
                (
                    keyboard_label_controls,
                    update_solar_system_visuals,
                    update_orbit_markers,
                    update_solar_body_labels,
                    apply_label_visibility,
                ),
            );
    }
}

fn spawn_solar_system_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.12,
        ..default()
    });

    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(32, 18));
    let orbit_marker_sphere = meshes.add(Sphere::new(1.0).mesh().uv(12, 8));

    let planet_orbit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.45, 0.9),
        emissive: LinearRgba::rgb(0.02, 0.05, 0.12),
        ..default()
    });

    let satellite_orbit_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.55, 0.55),
        emissive: LinearRgba::rgb(0.05, 0.05, 0.05),
        ..default()
    });

    for body in SOLAR_SYSTEM_BODIES {
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

        spawn_label(
            &mut commands,
            body.name,
            body.id,
            label_font_size(body),
            label_color(body),
        );

        if let Some(orbit) = body.orbit {
            let total = orbit_marker_count(orbit);
            let marker_radius = orbit_marker_radius(orbit);
            let orbit_material = if orbit.parent == BodyId::Sun {
                planet_orbit_material.clone()
            } else {
                satellite_orbit_material.clone()
            };

            for index in 0..total {
                commands.spawn((
                    Mesh3d(orbit_marker_sphere.clone()),
                    MeshMaterial3d(orbit_material.clone()),
                    Transform::from_scale(Vec3::splat(marker_radius)),
                    OrbitMarkerVisual {
                        body_id: body.id,
                        index,
                        total,
                    },
                ));
            }
        }
    }

    commands.spawn((
        PointLight {
            intensity: 12_000_000.0,
            range: 1_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn keyboard_label_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut label_visibility_mode: ResMut<LabelVisibilityMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        *label_visibility_mode = label_visibility_mode.next();
        info!("Label visibility mode: {}", label_visibility_mode.as_str());
    }
}

fn body_emissive_color(body: &CelestialBodyDefinition) -> LinearRgba {
    match body.class {
        BodyClass::Star => LinearRgba::rgb(1.0, 0.55, 0.08),
        _ => LinearRgba::BLACK,
    }
}

fn label_font_size(body: &CelestialBodyDefinition) -> f32 {
    match body.class {
        BodyClass::Star => 42.0,
        BodyClass::GasGiant | BodyClass::IceGiant => 30.0,
        BodyClass::TerrestrialPlanet => 26.0,
        BodyClass::NaturalSatellite => 22.0,
    }
}

fn label_vertical_offset(body: &CelestialBodyDefinition) -> f32 {
    match body.class {
        BodyClass::Star => body.visual_radius + 2.2,
        BodyClass::GasGiant | BodyClass::IceGiant => body.visual_radius + 1.5,
        BodyClass::TerrestrialPlanet => body.visual_radius + 1.1,
        BodyClass::NaturalSatellite => body.visual_radius + 0.9,
    }
}

fn label_color(body: &CelestialBodyDefinition) -> Color {
    match body.class {
        BodyClass::Star => Color::srgb(1.0, 0.92, 0.35),
        BodyClass::TerrestrialPlanet => Color::srgb(0.88, 0.95, 1.0),
        BodyClass::GasGiant => Color::srgb(1.0, 0.78, 0.52),
        BodyClass::IceGiant => Color::srgb(0.55, 0.95, 1.0),
        BodyClass::NaturalSatellite => Color::srgb(0.82, 0.82, 0.82),
    }
}

fn spawn_label(
    commands: &mut Commands,
    text: &'static str,
    id: BodyId,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Visible,
        SolarBodyLabel { id },
    ));
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

fn update_orbit_markers(
    simulation_clock: Res<SimulationClock>,
    mut query: Query<(&OrbitMarkerVisual, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    for (marker, mut transform) in query.iter_mut() {
        let Some(body) = SOLAR_SYSTEM_BODIES
            .iter()
            .find(|body| body.id == marker.body_id)
        else {
            continue;
        };

        let Some(orbit) = body.orbit else {
            continue;
        };

        let Some(parent_visual_position) = body_visual_position(orbit.parent, days_since_j2000)
        else {
            continue;
        };

        let angle = std::f32::consts::TAU * marker.index as f32 / marker.total as f32;
        let circle_position = Vec3::new(angle.cos(), 0.0, angle.sin());
        let visual_radius = educational_orbit_radius(orbit);

        transform.translation = parent_visual_position + circle_position * visual_radius;
    }
}

fn update_solar_body_labels(
    simulation_clock: Res<SimulationClock>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<SolarBodyLabel>)>,
    mut query: Query<(&SolarBodyLabel, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();
    let camera_rotation = camera_query
        .iter()
        .next()
        .map(|transform| transform.rotation)
        .unwrap_or(Quat::IDENTITY);

    for (label, mut transform) in query.iter_mut() {
        let Some(body) = SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == label.id) else {
            continue;
        };

        let Some(visual_position) = body_visual_position(label.id, days_since_j2000) else {
            continue;
        };

        transform.translation = visual_position + Vec3::new(0.0, label_vertical_offset(body), 0.0);
        transform.rotation = camera_rotation;
    }
}

fn apply_label_visibility(
    label_visibility_mode: Res<LabelVisibilityMode>,
    mut query: Query<(&SolarBodyLabel, &mut Visibility)>,
) {
    for (label, mut visibility) in query.iter_mut() {
        let visible = match *label_visibility_mode {
            LabelVisibilityMode::MajorOnly => is_major_body_label(label.id),
            LabelVisibilityMode::All => true,
            LabelVisibilityMode::None => false,
        };

        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn is_major_body_label(id: BodyId) -> bool {
    matches!(
        id,
        BodyId::Sun
            | BodyId::Mercury
            | BodyId::Venus
            | BodyId::Earth
            | BodyId::Mars
            | BodyId::Jupiter
            | BodyId::Saturn
            | BodyId::Uranus
            | BodyId::Neptune
    )
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

fn educational_orbit_radius(orbit: OrbitDefinition) -> f32 {
    if orbit.parent == BodyId::Sun {
        let au = orbit.semi_major_axis_meters / AU_METERS;
        10.0 + au.sqrt() as f32 * 10.0
    } else {
        2.6
    }
}

fn orbit_marker_count(orbit: OrbitDefinition) -> usize {
    if orbit.parent == BodyId::Sun {
        PLANET_ORBIT_MARKERS
    } else {
        SATELLITE_ORBIT_MARKERS
    }
}

fn orbit_marker_radius(orbit: OrbitDefinition) -> f32 {
    if orbit.parent == BodyId::Sun {
        PLANET_ORBIT_MARKER_RADIUS
    } else {
        SATELLITE_ORBIT_MARKER_RADIUS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_visibility_mode_cycles_in_expected_order() {
        assert_eq!(
            LabelVisibilityMode::MajorOnly.next(),
            LabelVisibilityMode::All
        );
        assert_eq!(LabelVisibilityMode::All.next(), LabelVisibilityMode::None);
        assert_eq!(
            LabelVisibilityMode::None.next(),
            LabelVisibilityMode::MajorOnly
        );
    }

    #[test]
    fn major_labels_include_planets_but_not_moon() {
        assert!(is_major_body_label(BodyId::Sun));
        assert!(is_major_body_label(BodyId::Earth));
        assert!(is_major_body_label(BodyId::Jupiter));
        assert!(!is_major_body_label(BodyId::Moon));
    }

    #[test]
    fn star_label_is_larger_than_earth_label() {
        let sun = SOLAR_SYSTEM_BODIES
            .iter()
            .find(|body| body.id == BodyId::Sun)
            .unwrap();

        let earth = SOLAR_SYSTEM_BODIES
            .iter()
            .find(|body| body.id == BodyId::Earth)
            .unwrap();

        assert!(label_font_size(sun) > label_font_size(earth));
    }

    #[test]
    fn star_label_has_larger_vertical_offset_than_moon_label() {
        let sun = SOLAR_SYSTEM_BODIES
            .iter()
            .find(|body| body.id == BodyId::Sun)
            .unwrap();

        let moon = SOLAR_SYSTEM_BODIES
            .iter()
            .find(|body| body.id == BodyId::Moon)
            .unwrap();

        assert!(label_vertical_offset(sun) > label_vertical_offset(moon));
    }
}
