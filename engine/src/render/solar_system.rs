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
        app.add_systems(Startup, spawn_solar_system_visuals)
            .add_systems(
                Update,
                (
                    update_solar_system_visuals,
                    update_orbit_markers,
                    update_solar_body_labels,
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

        spawn_label(&mut commands, body.name, body.id);

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

fn body_emissive_color(body: &CelestialBodyDefinition) -> LinearRgba {
    match body.class {
        BodyClass::Star => LinearRgba::rgb(1.0, 0.55, 0.08),
        _ => LinearRgba::BLACK,
    }
}

fn spawn_label(commands: &mut Commands, text: &'static str, id: BodyId) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 0.0),
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
    mut query: Query<(&SolarBodyLabel, &mut Transform)>,
) {
    let days_since_j2000 = simulation_clock.0.days_since_j2000();

    for (label, mut transform) in query.iter_mut() {
        let Some(body) = SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == label.id) else {
            continue;
        };

        let Some(visual_position) = body_visual_position(label.id, days_since_j2000) else {
            continue;
        };

        transform.translation = visual_position + Vec3::new(0.0, body.visual_radius + 0.7, 0.0);
    }
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
