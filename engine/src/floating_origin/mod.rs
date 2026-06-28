use bevy::prelude::*;

use crate::camera::FreeCamera;
use crate::coordinates::{to_local_position, GlobalPosition, GlobalPositionComponent};

#[derive(Debug, Clone, Copy)]
pub struct FloatingOrigin {
    pub origin_global: GlobalPosition,
    pub recenter_threshold_meters: f64,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct FloatingOriginRuntime {
    pub origin: FloatingOrigin,
}

pub struct FloatingOriginRuntimePlugin;

impl Plugin for FloatingOriginRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FloatingOriginRuntime::default())
            .add_systems(Update, recenter_floating_origin_runtime);
    }
}

impl Default for FloatingOriginRuntime {
    fn default() -> Self {
        Self {
            origin: FloatingOrigin::new(10_000_000.0),
        }
    }
}

impl FloatingOrigin {
    pub fn new(recenter_threshold_meters: f64) -> Self {
        Self {
            origin_global: GlobalPosition::ZERO,
            recenter_threshold_meters,
        }
    }

    pub fn should_recenter(&self, camera_global: GlobalPosition) -> bool {
        self.origin_global.distance_to(camera_global) >= self.recenter_threshold_meters
    }

    pub fn recenter_on_camera(&mut self, camera_global: GlobalPosition) {
        self.origin_global = camera_global;
    }

    pub fn update(&mut self, camera_global: GlobalPosition) -> bool {
        if self.should_recenter(camera_global) {
            self.recenter_on_camera(camera_global);
            true
        } else {
            false
        }
    }
}

fn recenter_floating_origin_runtime(
    mut runtime: ResMut<FloatingOriginRuntime>,
    camera_query: Query<&GlobalPositionComponent, With<FreeCamera>>,
    mut objects_query: Query<(&GlobalPositionComponent, &mut Transform)>,
) {
    let Ok(camera_global_position) = camera_query.single() else {
        return;
    };

    let did_recenter = runtime.origin.update(camera_global_position.position);

    if !did_recenter {
        return;
    }

    for (object_global_position, mut transform) in objects_query.iter_mut() {
        transform.translation = to_local_position(
            object_global_position.position,
            runtime.origin.origin_global,
        )
        .position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinates::GlobalPosition;

    #[test]
    fn does_not_recenter_below_threshold() {
        let mut origin = FloatingOrigin::new(10_000.0);
        let camera = GlobalPosition::new_meters(9_999.0, 0.0, 0.0);

        assert!(!origin.update(camera));
    }

    #[test]
    fn recenters_at_threshold() {
        let mut origin = FloatingOrigin::new(10_000.0);
        let camera = GlobalPosition::new_meters(10_000.0, 0.0, 0.0);

        assert!(origin.update(camera));
        assert_eq!(origin.origin_global, camera);
    }
}
