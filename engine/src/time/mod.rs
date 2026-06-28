use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Epoch {
    J2000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationTime {
    pub jd_tdb: f64,
    pub epoch: Epoch,
    pub time_scale: f64,
    pub paused: bool,
    pub direction: TimeDirection,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct SimulationClock(pub SimulationTime);

pub struct SimulationTimeControlsPlugin;

impl Plugin for SimulationTimeControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_time_controls);
    }
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self(SimulationTime::j2000())
    }
}

impl SimulationTime {
    pub const J2000_JD_TDB: f64 = 2_451_545.0;
    pub const SECONDS_PER_DAY: f64 = 86_400.0;

    pub fn j2000() -> Self {
        Self {
            jd_tdb: Self::J2000_JD_TDB,
            epoch: Epoch::J2000,
            time_scale: 1.0,
            paused: false,
            direction: TimeDirection::Forward,
        }
    }

    pub fn days_since_j2000(self) -> f64 {
        self.jd_tdb - Self::J2000_JD_TDB
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        assert!(scale >= 0.0, "time_scale no puede ser negativo");
        self.time_scale = scale;
    }

    pub fn set_direction(&mut self, direction: TimeDirection) {
        self.direction = direction;
    }

    pub fn toggle_direction(&mut self) {
        self.direction = match self.direction {
            TimeDirection::Forward => TimeDirection::Backward,
            TimeDirection::Backward => TimeDirection::Forward,
        };
    }

    pub fn tick_seconds(&mut self, real_delta_seconds: f64) {
        if self.paused {
            return;
        }

        let sign = match self.direction {
            TimeDirection::Forward => 1.0,
            TimeDirection::Backward => -1.0,
        };

        let simulation_seconds = real_delta_seconds * self.time_scale * sign;
        self.jd_tdb += simulation_seconds / Self::SECONDS_PER_DAY;
    }
}

fn keyboard_time_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut simulation_clock: ResMut<SimulationClock>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        simulation_clock.0.toggle_pause();
        info!("SimulationTime paused: {}", simulation_clock.0.paused);
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        simulation_clock.0.set_time_scale(1.0);
        info!("SimulationTime scale: x1");
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        simulation_clock.0.set_time_scale(100.0);
        info!("SimulationTime scale: x100");
    }

    if keyboard.just_pressed(KeyCode::Digit3) {
        simulation_clock.0.set_time_scale(1_000.0);
        info!("SimulationTime scale: x1000");
    }

    if keyboard.just_pressed(KeyCode::Digit4) {
        simulation_clock.0.set_time_scale(10_000.0);
        info!("SimulationTime scale: x10000");
    }

    if keyboard.just_pressed(KeyCode::Digit5) {
        simulation_clock.0.set_time_scale(50_000.0);
        info!("SimulationTime scale: x50000");
    }

    if keyboard.just_pressed(KeyCode::Digit6) {
        simulation_clock.0.set_time_scale(1_000_000.0);
        info!("SimulationTime scale: x1000000");
    }

    if keyboard.just_pressed(KeyCode::KeyB) {
        simulation_clock.0.toggle_direction();
        info!(
            "SimulationTime direction: {:?}",
            simulation_clock.0.direction
        );
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        simulation_clock.0 = SimulationTime::j2000();
        simulation_clock.0.set_time_scale(50_000.0);
        info!("SimulationTime reset to J2000 with x50000 scale");
    }
}

pub fn utc_to_tdb(jd_utc: f64) -> f64 {
    jd_utc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_j2000() {
        let time = SimulationTime::j2000();
        assert_eq!(time.jd_tdb, SimulationTime::J2000_JD_TDB);
        assert_eq!(time.epoch, Epoch::J2000);
    }

    #[test]
    fn reports_zero_days_since_j2000_at_start() {
        let time = SimulationTime::j2000();
        assert_eq!(time.days_since_j2000(), 0.0);
    }

    #[test]
    fn advances_one_day_at_normal_scale() {
        let mut time = SimulationTime::j2000();
        time.tick_seconds(SimulationTime::SECONDS_PER_DAY);
        assert_eq!(time.jd_tdb, SimulationTime::J2000_JD_TDB + 1.0);
        assert_eq!(time.days_since_j2000(), 1.0);
    }

    #[test]
    fn can_go_backward() {
        let mut time = SimulationTime::j2000();
        time.set_direction(TimeDirection::Backward);
        time.tick_seconds(SimulationTime::SECONDS_PER_DAY);
        assert_eq!(time.jd_tdb, SimulationTime::J2000_JD_TDB - 1.0);
    }

    #[test]
    fn paused_time_does_not_advance() {
        let mut time = SimulationTime::j2000();
        time.pause();
        time.tick_seconds(SimulationTime::SECONDS_PER_DAY);
        assert_eq!(time.jd_tdb, SimulationTime::J2000_JD_TDB);
    }

    #[test]
    fn toggle_pause_changes_pause_state() {
        let mut time = SimulationTime::j2000();
        assert!(!time.paused);

        time.toggle_pause();
        assert!(time.paused);

        time.toggle_pause();
        assert!(!time.paused);
    }

    #[test]
    fn toggle_direction_changes_direction() {
        let mut time = SimulationTime::j2000();
        assert_eq!(time.direction, TimeDirection::Forward);

        time.toggle_direction();
        assert_eq!(time.direction, TimeDirection::Backward);

        time.toggle_direction();
        assert_eq!(time.direction, TimeDirection::Forward);
    }
}
