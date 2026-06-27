use crate::coordinates::GlobalPosition;

#[derive(Debug, Clone, Copy)]
pub struct FloatingOrigin {
    pub origin_global: GlobalPosition,
    pub recenter_threshold_meters: f64,
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
