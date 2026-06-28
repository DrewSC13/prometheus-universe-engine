use bevy::math::DVec3;

use crate::simulation::bodies::{
    find_body, BodyId, CelestialBodyDefinition, OrbitDefinition, SOLAR_SYSTEM_BODIES,
};

#[derive(Debug, Clone, Copy)]
pub struct BodyRuntimeState {
    pub id: BodyId,
    pub physical_position_meters: DVec3,
}

pub fn circular_orbit_position(orbit: OrbitDefinition, days_since_epoch: f64) -> DVec3 {
    let angle = orbit.phase_radians + std::f64::consts::TAU * days_since_epoch / orbit.period_days;

    DVec3::new(
        orbit.semi_major_axis_meters * angle.cos(),
        0.0,
        orbit.semi_major_axis_meters * angle.sin(),
    )
}

pub fn body_position_meters(id: BodyId, days_since_epoch: f64) -> Option<DVec3> {
    body_position_meters_with_depth(id, days_since_epoch, 0)
}

fn body_position_meters_with_depth(
    id: BodyId,
    days_since_epoch: f64,
    depth: usize,
) -> Option<DVec3> {
    if depth > SOLAR_SYSTEM_BODIES.len() {
        return None;
    }

    let body = find_body(id)?;

    match body.orbit {
        Some(orbit) => {
            let parent_position =
                body_position_meters_with_depth(orbit.parent, days_since_epoch, depth + 1)?;

            Some(parent_position + circular_orbit_position(orbit, days_since_epoch))
        }
        None => Some(DVec3::ZERO),
    }
}

pub fn solar_system_runtime_state(days_since_epoch: f64) -> Vec<BodyRuntimeState> {
    SOLAR_SYSTEM_BODIES
        .iter()
        .filter_map(|body| {
            body_position_meters(body.id, days_since_epoch).map(|physical_position_meters| {
                BodyRuntimeState {
                    id: body.id,
                    physical_position_meters,
                }
            })
        })
        .collect()
}

pub fn body_definition(id: BodyId) -> Option<&'static CelestialBodyDefinition> {
    find_body(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::bodies::{BodyId, SOLAR_SYSTEM_BODIES};

    #[test]
    fn runtime_state_contains_every_catalog_body() {
        let state = solar_system_runtime_state(0.0);

        assert_eq!(state.len(), SOLAR_SYSTEM_BODIES.len());
    }

    #[test]
    fn sun_runtime_position_is_origin() {
        let sun_position = body_position_meters(BodyId::Sun, 0.0).unwrap();

        assert_eq!(sun_position, DVec3::ZERO);
    }

    #[test]
    fn earth_is_farther_than_venus_at_epoch() {
        let earth_position = body_position_meters(BodyId::Earth, 0.0).unwrap();
        let venus_position = body_position_meters(BodyId::Venus, 0.0).unwrap();

        assert!(earth_position.length() > venus_position.length());
    }

    #[test]
    fn moon_orbits_relative_to_earth() {
        let earth_position = body_position_meters(BodyId::Earth, 0.0).unwrap();
        let moon_position = body_position_meters(BodyId::Moon, 0.0).unwrap();

        let moon_relative_distance = moon_position.distance(earth_position);

        assert!((moon_relative_distance - 384_400_000.0).abs() < 1.0);
    }
}
