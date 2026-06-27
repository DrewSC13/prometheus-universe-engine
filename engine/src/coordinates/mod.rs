use bevy::math::{DVec3, Vec3};
use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalPosition {
    pub meters_from_origin: DVec3,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct GlobalPositionComponent {
    pub position: GlobalPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorId {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalRenderPosition {
    pub position: Vec3,
}

impl Default for GlobalPositionComponent {
    fn default() -> Self {
        Self {
            position: GlobalPosition::ZERO,
        }
    }
}

impl GlobalPosition {
    pub const ZERO: Self = Self {
        meters_from_origin: DVec3::ZERO,
    };

    pub fn new_meters(x: f64, y: f64, z: f64) -> Self {
        Self {
            meters_from_origin: DVec3::new(x, y, z),
        }
    }

    pub fn distance_to(self, other: Self) -> f64 {
        self.meters_from_origin.distance(other.meters_from_origin)
    }

    pub fn translated_by_meters(self, delta: DVec3) -> Self {
        Self {
            meters_from_origin: self.meters_from_origin + delta,
        }
    }
}

pub fn to_local_position(
    object_global: GlobalPosition,
    camera_global: GlobalPosition,
) -> LocalRenderPosition {
    let delta = object_global.meters_from_origin - camera_global.meters_from_origin;

    LocalRenderPosition {
        position: delta.as_vec3(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_global_to_local_relative_to_camera() {
        let object = GlobalPosition::new_meters(10_000.0, 0.0, 0.0);
        let camera = GlobalPosition::new_meters(9_000.0, 0.0, 0.0);

        let local = to_local_position(object, camera);

        assert_eq!(local.position, Vec3::new(1_000.0, 0.0, 0.0));
    }

    #[test]
    fn supports_lunar_distance_without_local_precision_loss() {
        let moon_distance_m = 384_000_000.0;
        let object = GlobalPosition::new_meters(moon_distance_m, 0.0, 0.0);
        let camera = GlobalPosition::new_meters(moon_distance_m - 1_000.0, 0.0, 0.0);

        let local = to_local_position(object, camera);

        assert_eq!(local.position.x, 1_000.0);
    }

    #[test]
    fn translates_global_position_in_meters() {
        let position = GlobalPosition::new_meters(1.0, 2.0, 3.0);
        let moved = position.translated_by_meters(DVec3::new(10.0, 20.0, 30.0));

        assert_eq!(moved, GlobalPosition::new_meters(11.0, 22.0, 33.0));
    }
}
