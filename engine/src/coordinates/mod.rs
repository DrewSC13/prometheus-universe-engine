use glam::{DVec3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalPosition {
    pub meters_from_origin: DVec3,
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
}
