use super::{body_visual_position, realistic_scene_units_from_meters, OrbitVisibilityMode};

use crate::simulation::bodies::{BodyId, OrbitDefinition, SOLAR_SYSTEM_BODIES};
use crate::time::SimulationClock;

use bevy::prelude::*;

pub(super) const PLANET_ORBIT_MARKERS: usize = 128;
pub(super) const SATELLITE_ORBIT_MARKERS: usize = 64;
pub(super) const PLANET_ORBIT_MARKER_RADIUS: f32 = 0.025;
pub(super) const SATELLITE_ORBIT_MARKER_RADIUS: f32 = 0.020;

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct OrbitMarkerVisual {
    pub body_id: BodyId,
    pub index: usize,
    pub total: usize,
}

pub(super) fn spawn_orbit_markers(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    planet_orbit_material: Handle<StandardMaterial>,
    satellite_orbit_material: Handle<StandardMaterial>,
    body_id: BodyId,
    orbit: OrbitDefinition,
) {
    let total = orbit_marker_count(orbit);
    let marker_radius = orbit_marker_radius(orbit);
    let orbit_material = if orbit.parent == BodyId::Sun {
        planet_orbit_material
    } else {
        satellite_orbit_material
    };

    for index in 0..total {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(orbit_material.clone()),
            Transform::from_scale(Vec3::splat(marker_radius)),
            Visibility::Visible,
            OrbitMarkerVisual {
                body_id,
                index,
                total,
            },
        ));
    }
}

pub(super) fn keyboard_orbit_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut orbit_visibility_mode: ResMut<OrbitVisibilityMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        *orbit_visibility_mode = orbit_visibility_mode.next();
        info!("Orbit visibility mode: {}", orbit_visibility_mode.as_str());
    }
}

pub(super) fn update_orbit_markers(
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
        let visual_radius = realistic_orbit_radius(orbit);

        transform.translation = parent_visual_position + circle_position * visual_radius;
    }
}

pub(super) fn apply_orbit_visibility(
    orbit_visibility_mode: Res<OrbitVisibilityMode>,
    mut query: Query<(&OrbitMarkerVisual, &mut Visibility)>,
) {
    for (marker, mut visibility) in query.iter_mut() {
        let visible = match *orbit_visibility_mode {
            OrbitVisibilityMode::All => true,
            OrbitVisibilityMode::PlanetaryOnly => is_planetary_orbit(marker.body_id),
            OrbitVisibilityMode::None => false,
        };

        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub(super) fn is_planetary_orbit(body_id: BodyId) -> bool {
    SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == body_id)
        .and_then(|body| body.orbit)
        .is_some_and(|orbit| orbit.parent == BodyId::Sun)
}

pub(super) fn realistic_orbit_radius(orbit: OrbitDefinition) -> f32 {
    realistic_scene_units_from_meters(orbit.semi_major_axis_meters)
}

pub(super) fn orbit_marker_count(orbit: OrbitDefinition) -> usize {
    if orbit.parent == BodyId::Sun {
        PLANET_ORBIT_MARKERS
    } else {
        SATELLITE_ORBIT_MARKERS
    }
}

pub(super) fn orbit_marker_radius(orbit: OrbitDefinition) -> f32 {
    if orbit.parent == BodyId::Sun {
        PLANET_ORBIT_MARKER_RADIUS
    } else {
        SATELLITE_ORBIT_MARKER_RADIUS
    }
}
