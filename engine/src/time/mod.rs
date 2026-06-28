use bevy::prelude::*;

pub const DEFAULT_TIME_SCALE: f64 = 50_000.0;

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

pub fn time_scale_preset_for_digit(digit: u8) -> Option<f64> {
    match digit {
        1 => Some(1_000.0),
        2 => Some(10_000.0),
        3 => Some(50_000.0),
        4 => Some(250_000.0),
        5 => Some(1_000_000.0),
        6 => Some(5_000_000.0),
        7 => Some(25_000_000.0),
        8 => Some(100_000_000.0),
        9 => Some(500_000_000.0),
        0 => Some(1_000_000_000.0),
        _ => None,
    }
}

pub fn format_time_scale(scale: f64) -> String {
    format!("{}x", format_integer_with_separators(scale.round() as i64))
}

fn format_integer_with_separators(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut formatted = String::new();

    for (index, character) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            formatted.push('.');
        }

        formatted.push(character);
    }

    let grouped = formatted.chars().rev().collect::<String>();
    format!("{sign}{grouped}")
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

    for (key, digit) in [
        (KeyCode::Digit1, 1),
        (KeyCode::Digit2, 2),
        (KeyCode::Digit3, 3),
        (KeyCode::Digit4, 4),
        (KeyCode::Digit5, 5),
        (KeyCode::Digit6, 6),
        (KeyCode::Digit7, 7),
        (KeyCode::Digit8, 8),
        (KeyCode::Digit9, 9),
        (KeyCode::Digit0, 0),
    ] {
        if keyboard.just_pressed(key) {
            if let Some(scale) = time_scale_preset_for_digit(digit) {
                simulation_clock.0.set_time_scale(scale);
                info!("Time scale: {}", format_time_scale(scale));
            }
        }
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
        simulation_clock.0.set_time_scale(DEFAULT_TIME_SCALE);
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
    #[test]
    fn time_scale_presets_are_spaced_for_visible_orbital_motion() {
        assert_eq!(time_scale_preset_for_digit(1), Some(1_000.0));
        assert_eq!(time_scale_preset_for_digit(3), Some(DEFAULT_TIME_SCALE));
        assert_eq!(time_scale_preset_for_digit(0), Some(1_000_000_000.0));
        assert_eq!(time_scale_preset_for_digit(10), None);
    }

    #[test]
    fn time_scale_format_uses_grouped_spanish_style_numbers() {
        assert_eq!(format_time_scale(50_000.0), "50.000x");
        assert_eq!(format_time_scale(1_000_000.0), "1.000.000x");
    }
}
