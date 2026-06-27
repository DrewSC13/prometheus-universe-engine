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

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        assert!(scale >= 0.0, "time_scale no puede ser negativo");
        self.time_scale = scale;
    }

    pub fn set_direction(&mut self, direction: TimeDirection) {
        self.direction = direction;
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
    fn advances_one_day_at_normal_scale() {
        let mut time = SimulationTime::j2000();
        time.tick_seconds(SimulationTime::SECONDS_PER_DAY);
        assert_eq!(time.jd_tdb, SimulationTime::J2000_JD_TDB + 1.0);
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
}
