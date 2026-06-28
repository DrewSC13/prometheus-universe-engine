use bevy::math::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialBodyKind {
    Sun,
    Earth,
    Moon,
}

#[derive(Debug, Clone, Copy)]
pub struct CelestialBody {
    pub kind: CelestialBodyKind,
    pub name: &'static str,
    pub radius_meters: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct CircularOrbit {
    pub radius_meters: f64,
    pub period_days: f64,
    pub phase_radians: f64,
}

impl CircularOrbit {
    pub fn position_at_days(self, days_since_epoch: f64) -> DVec3 {
        let angle =
            self.phase_radians + std::f64::consts::TAU * days_since_epoch / self.period_days;

        DVec3::new(
            self.radius_meters * angle.cos(),
            0.0,
            self.radius_meters * angle.sin(),
        )
    }
}

pub const SUN: CelestialBody = CelestialBody {
    kind: CelestialBodyKind::Sun,
    name: "Sol",
    radius_meters: 696_340_000.0,
};

pub const EARTH: CelestialBody = CelestialBody {
    kind: CelestialBodyKind::Earth,
    name: "Tierra",
    radius_meters: 6_371_000.0,
};

pub const MOON: CelestialBody = CelestialBody {
    kind: CelestialBodyKind::Moon,
    name: "Luna",
    radius_meters: 1_737_400.0,
};

pub const EARTH_ORBIT: CircularOrbit = CircularOrbit {
    radius_meters: 149_597_870_700.0,
    period_days: 365.256,
    phase_radians: 0.0,
};

pub const MOON_ORBIT: CircularOrbit = CircularOrbit {
    radius_meters: 384_400_000.0,
    period_days: 27.321_661,
    phase_radians: 0.0,
};

#[derive(Debug, Clone, Copy)]
pub struct SolarEarthMoonState {
    pub sun_position_meters: DVec3,
    pub earth_position_meters: DVec3,
    pub moon_position_meters: DVec3,
}

pub fn solar_earth_moon_state(days_since_j2000: f64) -> SolarEarthMoonState {
    let sun_position_meters = DVec3::ZERO;
    let earth_position_meters = EARTH_ORBIT.position_at_days(days_since_j2000);
    let moon_relative_position_meters = MOON_ORBIT.position_at_days(days_since_j2000);
    let moon_position_meters = earth_position_meters + moon_relative_position_meters;

    SolarEarthMoonState {
        sun_position_meters,
        earth_position_meters,
        moon_position_meters,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sun_starts_at_origin() {
        let state = solar_earth_moon_state(0.0);
        assert_eq!(state.sun_position_meters, DVec3::ZERO);
    }

    #[test]
    fn earth_starts_at_orbit_radius_on_x_axis() {
        let state = solar_earth_moon_state(0.0);

        assert!((state.earth_position_meters.x - EARTH_ORBIT.radius_meters).abs() < 1.0);
        assert_eq!(state.earth_position_meters.y, 0.0);
        assert!(state.earth_position_meters.z.abs() < 1.0);
    }

    #[test]
    fn moon_starts_relative_to_earth() {
        let state = solar_earth_moon_state(0.0);
        let relative = state.moon_position_meters - state.earth_position_meters;

        assert!((relative.x - MOON_ORBIT.radius_meters).abs() < 1.0);
        assert_eq!(relative.y, 0.0);
        assert!(relative.z.abs() < 1.0);
    }

    #[test]
    fn earth_returns_near_start_after_one_orbit() {
        let start = solar_earth_moon_state(0.0);
        let end = solar_earth_moon_state(EARTH_ORBIT.period_days);

        let distance = start
            .earth_position_meters
            .distance(end.earth_position_meters);

        assert!(distance < 1.0);
    }

    #[test]
    fn moon_returns_near_relative_start_after_one_orbit() {
        let start = solar_earth_moon_state(0.0);
        let end = solar_earth_moon_state(MOON_ORBIT.period_days);

        let start_relative = start.moon_position_meters - start.earth_position_meters;
        let end_relative = end.moon_position_meters - end.earth_position_meters;

        let distance = start_relative.distance(end_relative);

        assert!(distance < 1.0);
    }
}
