use bevy::prelude::*;

use crate::coordinates::{GlobalPosition, GlobalPositionComponent};
use crate::interaction::selection::SelectedBody;
use crate::render::solar_system::solar_body_visual_position;
use crate::simulation::bodies::{BodyId, SOLAR_SYSTEM_BODIES};
use crate::simulation::catalog::body_definition;
use crate::time::SimulationClock;

const FOCUS_CAMERA_MIN_DISTANCE: f32 = 4.0;
const FOCUS_CAMERA_MAX_DISTANCE: f32 = 72.0;
const FOCUS_CAMERA_DISTANCE_FACTOR: f32 = 8.0;
const FOCUS_CAMERA_OFFSET_DIRECTION: Vec3 = Vec3::new(0.0, 0.38, 1.0);
const FOCUS_SATELLITE_SYSTEM_DISTANCE_FACTOR: f32 = 3.90;
const FOCUS_SATELLITE_SYSTEM_PADDING: f32 = 1.25;

pub struct BodyFocusPlugin;

impl Plugin for BodyFocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_focus_selected_body);
    }
}

fn keyboard_focus_selected_body(
    keyboard: Res<ButtonInput<KeyCode>>,
    selected_body: Option<Res<SelectedBody>>,
    simulation_clock: Res<SimulationClock>,
    mut camera_query: Query<(&mut Transform, Option<&mut GlobalPositionComponent>), With<Camera3d>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyG) {
        return;
    }

    let Some(selected_body) = selected_body.as_deref() else {
        info!("Focus selected body skipped: selection resource is unavailable.");
        return;
    };

    let Some(body_id) = selected_body.id else {
        info!("Focus selected body skipped: no selected body.");
        return;
    };

    let Some(body) = body_definition(body_id) else {
        info!("Focus selected body skipped: selected body is not in catalog.");
        return;
    };

    let Some(target_position) =
        solar_body_visual_position(body_id, simulation_clock.0.days_since_j2000())
    else {
        info!("Focus selected body skipped: selected body has no visual position.");
        return;
    };

    let focus_transform = focus_camera_transform_for_body(
        body_id,
        target_position,
        body.visual_radius,
        simulation_clock.0.days_since_j2000(),
    );

    for (mut transform, global_position) in camera_query.iter_mut() {
        *transform = focus_transform;

        if let Some(mut global_position) = global_position {
            global_position.position = GlobalPosition::ZERO;
        }
    }

    info!("Focused selected body: {}", body.name);
}

fn focus_camera_transform_for_body(
    body_id: BodyId,
    target_position: Vec3,
    body_visual_radius: f32,
    days_since_j2000: f64,
) -> Transform {
    let distance = focus_camera_distance_for_body(
        body_id,
        target_position,
        body_visual_radius,
        days_since_j2000,
    );
    let offset_direction = FOCUS_CAMERA_OFFSET_DIRECTION.normalize_or_zero();
    let camera_position = target_position + offset_direction * distance;

    Transform::from_translation(camera_position).looking_at(target_position, Vec3::Y)
}

pub fn focus_camera_distance_for_body(
    body_id: BodyId,
    target_position: Vec3,
    body_visual_radius: f32,
    days_since_j2000: f64,
) -> f32 {
    let body_distance = focus_camera_distance(body_visual_radius);
    let system_distance =
        satellite_system_focus_distance(body_id, target_position, days_since_j2000)
            .unwrap_or(body_distance);

    body_distance.max(system_distance)
}

pub fn satellite_system_focus_distance(
    body_id: BodyId,
    target_position: Vec3,
    days_since_j2000: f64,
) -> Option<f32> {
    if body_id == BodyId::Sun {
        return None;
    }

    let mut farthest_satellite_radius = 0.0_f32;

    for satellite in SOLAR_SYSTEM_BODIES
        .iter()
        .filter(|satellite| satellite.orbit.is_some_and(|orbit| orbit.parent == body_id))
    {
        let Some(satellite_position) = solar_body_visual_position(satellite.id, days_since_j2000)
        else {
            continue;
        };

        let local_radius = satellite_position.distance(target_position) + satellite.visual_radius;

        farthest_satellite_radius = farthest_satellite_radius.max(local_radius);
    }

    if farthest_satellite_radius <= 0.0 {
        return None;
    }

    Some(
        ((farthest_satellite_radius + FOCUS_SATELLITE_SYSTEM_PADDING)
            * FOCUS_SATELLITE_SYSTEM_DISTANCE_FACTOR)
            .clamp(FOCUS_CAMERA_MIN_DISTANCE, FOCUS_CAMERA_MAX_DISTANCE),
    )
}

pub fn focus_camera_distance(body_visual_radius: f32) -> f32 {
    (body_visual_radius * FOCUS_CAMERA_DISTANCE_FACTOR)
        .clamp(FOCUS_CAMERA_MIN_DISTANCE, FOCUS_CAMERA_MAX_DISTANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_camera_distance_scales_with_body_radius() {
        assert!(focus_camera_distance(3.5) > focus_camera_distance(0.55));
    }

    #[test]
    fn focus_camera_distance_keeps_small_bodies_observable() {
        assert!(focus_camera_distance(0.09) >= FOCUS_CAMERA_MIN_DISTANCE);
    }

    #[test]
    fn focus_camera_distance_has_upper_bound() {
        assert_eq!(focus_camera_distance(100.0), FOCUS_CAMERA_MAX_DISTANCE);
    }

    #[test]
    fn satellite_system_focus_distance_ignores_sun_planets() {
        assert!(satellite_system_focus_distance(BodyId::Sun, Vec3::ZERO, 0.0).is_none());
    }

    #[test]
    fn jupiter_focus_distance_includes_galilean_moon_system() {
        let days_since_j2000 = 0.0;
        let jupiter_position =
            solar_body_visual_position(BodyId::Jupiter, days_since_j2000).unwrap();
        let distance = focus_camera_distance_for_body(
            BodyId::Jupiter,
            jupiter_position,
            body_definition(BodyId::Jupiter).unwrap().visual_radius,
            days_since_j2000,
        );

        assert!(
            distance
                > focus_camera_distance(body_definition(BodyId::Jupiter).unwrap().visual_radius)
        );
        assert!(distance >= 18.0);
    }

    #[test]
    fn saturn_focus_distance_includes_major_moon_system() {
        let days_since_j2000 = 0.0;
        let saturn_position = solar_body_visual_position(BodyId::Saturn, days_since_j2000).unwrap();
        let distance = focus_camera_distance_for_body(
            BodyId::Saturn,
            saturn_position,
            body_definition(BodyId::Saturn).unwrap().visual_radius,
            days_since_j2000,
        );

        assert!(
            distance
                > focus_camera_distance(body_definition(BodyId::Saturn).unwrap().visual_radius)
        );
        assert!(distance >= 16.0);
    }

    #[test]
    fn focus_camera_transform_looks_at_target() {
        let target = Vec3::new(2.0, 0.5, -4.0);
        let transform = focus_camera_transform_for_body(
            crate::simulation::bodies::BodyId::Mercury,
            target,
            1.0,
            0.0,
        );

        let forward = transform.rotation * Vec3::NEG_Z;
        let expected_direction = (target - transform.translation).normalize();

        assert!(forward.dot(expected_direction) > 0.99);
        assert!(transform.translation.distance(target) >= FOCUS_CAMERA_MIN_DISTANCE);
    }
}
