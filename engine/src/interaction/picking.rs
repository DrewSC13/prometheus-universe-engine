use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::interaction::selection::SelectedBody;
use crate::render::solar_system::SolarBodyVisual;
use crate::simulation::bodies::BodyId;
use crate::simulation::catalog::body_definition;

const PICKING_RADIUS_FACTOR: f32 = 1.35;
const PICKING_MIN_PADDING: f32 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyPickHit {
    pub id: BodyId,
    pub distance: f32,
}

pub struct BodyPickingPlugin;

impl Plugin for BodyPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_pick_solar_body);
    }
}

fn mouse_pick_solar_body(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    body_query: Query<(&SolarBodyVisual, &Transform)>,
    mut selected_body: ResMut<SelectedBody>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(window) = windows.iter().next() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some((camera, camera_transform)) = camera_query.iter().next() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let ray_origin = ray.origin;
    let ray_direction = ray.get_point(1.0) - ray.origin;

    let candidates = body_query.iter().filter_map(|(body_visual, transform)| {
        let body = body_definition(body_visual.id)?;
        Some((
            body_visual.id,
            transform.translation,
            picking_radius(body.visual_radius),
        ))
    });

    let Some(hit) = closest_body_pick_hit(ray_origin, ray_direction, candidates) else {
        return;
    };

    selected_body.id = Some(hit.id);

    if let Some(body) = body_definition(hit.id) {
        info!("Mouse selected body: {}", body.name);
    }
}

pub fn closest_body_pick_hit<I>(
    ray_origin: Vec3,
    ray_direction: Vec3,
    candidates: I,
) -> Option<BodyPickHit>
where
    I: IntoIterator<Item = (BodyId, Vec3, f32)>,
{
    candidates
        .into_iter()
        .filter_map(|(id, center, radius)| {
            ray_sphere_intersection_distance(ray_origin, ray_direction, center, radius)
                .map(|distance| BodyPickHit { id, distance })
        })
        .min_by(|a, b| a.distance.total_cmp(&b.distance))
}

pub fn ray_sphere_intersection_distance(
    ray_origin: Vec3,
    ray_direction: Vec3,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> Option<f32> {
    if sphere_radius <= 0.0 {
        return None;
    }

    let direction = ray_direction.normalize_or_zero();

    if direction == Vec3::ZERO {
        return None;
    }

    let origin_to_center = ray_origin - sphere_center;
    let half_b = origin_to_center.dot(direction);
    let c = origin_to_center.length_squared() - sphere_radius * sphere_radius;
    let discriminant = half_b * half_b - c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let near_distance = -half_b - sqrt_discriminant;

    if near_distance >= 0.0 {
        return Some(near_distance);
    }

    let far_distance = -half_b + sqrt_discriminant;

    if far_distance >= 0.0 {
        Some(far_distance)
    } else {
        None
    }
}

pub fn picking_radius(body_visual_radius: f32) -> f32 {
    (body_visual_radius * PICKING_RADIUS_FACTOR).max(body_visual_radius + PICKING_MIN_PADDING)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_sphere_intersection_hits_front_sphere() {
        let distance =
            ray_sphere_intersection_distance(Vec3::ZERO, Vec3::Z, Vec3::new(0.0, 0.0, 5.0), 1.0)
                .unwrap();

        assert!((distance - 4.0).abs() < 0.001);
    }

    #[test]
    fn ray_sphere_intersection_misses_offset_sphere() {
        let distance =
            ray_sphere_intersection_distance(Vec3::ZERO, Vec3::Z, Vec3::new(5.0, 0.0, 5.0), 1.0);

        assert_eq!(distance, None);
    }

    #[test]
    fn ray_sphere_intersection_ignores_sphere_behind_ray() {
        let distance =
            ray_sphere_intersection_distance(Vec3::ZERO, Vec3::Z, Vec3::new(0.0, 0.0, -5.0), 1.0);

        assert_eq!(distance, None);
    }

    #[test]
    fn closest_body_pick_hit_prefers_nearest_hit() {
        let hit = closest_body_pick_hit(
            Vec3::ZERO,
            Vec3::Z,
            [
                (BodyId::Mars, Vec3::new(0.0, 0.0, 8.0), 1.0),
                (BodyId::Earth, Vec3::new(0.0, 0.0, 5.0), 1.0),
            ],
        )
        .unwrap();

        assert_eq!(hit.id, BodyId::Earth);
    }

    #[test]
    fn picking_radius_keeps_small_bodies_clickable() {
        assert!(picking_radius(0.09) >= 0.33);
        assert!(picking_radius(1.0) > 1.0);
    }
}
