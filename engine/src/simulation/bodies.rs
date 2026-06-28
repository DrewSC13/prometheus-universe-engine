#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyId {
    Sun,
    Mercury,
    Venus,
    Earth,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyClass {
    Star,
    TerrestrialPlanet,
    GasGiant,
    IceGiant,
    NaturalSatellite,
}

#[derive(Debug, Clone, Copy)]
pub struct OrbitDefinition {
    pub parent: BodyId,
    pub semi_major_axis_meters: f64,
    pub period_days: f64,
    pub phase_radians: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct CelestialBodyDefinition {
    pub id: BodyId,
    pub name: &'static str,
    pub class: BodyClass,
    pub radius_meters: f64,
    pub mass_kg: f64,
    pub visual_radius: f32,
    pub color_srgb: [f32; 3],
    pub orbit: Option<OrbitDefinition>,
}

impl CelestialBodyDefinition {
    pub fn has_orbit(self) -> bool {
        self.orbit.is_some()
    }

    pub fn is_root_body(self) -> bool {
        self.orbit.is_none()
    }
}

pub const SOLAR_SYSTEM_BODIES: &[CelestialBodyDefinition] = &[
    CelestialBodyDefinition {
        id: BodyId::Sun,
        name: "Sol",
        class: BodyClass::Star,
        radius_meters: 696_340_000.0,
        mass_kg: 1.9885e30,
        visual_radius: 3.0,
        color_srgb: [1.0, 0.82, 0.18],
        orbit: None,
    },
    CelestialBodyDefinition {
        id: BodyId::Mercury,
        name: "Mercurio",
        class: BodyClass::TerrestrialPlanet,
        radius_meters: 2_439_700.0,
        mass_kg: 3.3011e23,
        visual_radius: 0.30,
        color_srgb: [0.55, 0.52, 0.48],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 57_909_050_000.0,
            period_days: 87.969,
            phase_radians: 0.15,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Venus,
        name: "Venus",
        class: BodyClass::TerrestrialPlanet,
        radius_meters: 6_051_800.0,
        mass_kg: 4.8675e24,
        visual_radius: 0.58,
        color_srgb: [0.95, 0.72, 0.38],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 108_208_000_000.0,
            period_days: 224.701,
            phase_radians: 0.75,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Earth,
        name: "Tierra",
        class: BodyClass::TerrestrialPlanet,
        radius_meters: 6_371_000.0,
        mass_kg: 5.9722e24,
        visual_radius: 0.65,
        color_srgb: [0.08, 0.32, 1.0],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 149_597_870_700.0,
            period_days: 365.256,
            phase_radians: 0.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Moon,
        name: "Luna",
        class: BodyClass::NaturalSatellite,
        radius_meters: 1_737_400.0,
        mass_kg: 7.342e22,
        visual_radius: 0.22,
        color_srgb: [0.68, 0.68, 0.68],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Earth,
            semi_major_axis_meters: 384_400_000.0,
            period_days: 27.321_661,
            phase_radians: 0.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Mars,
        name: "Marte",
        class: BodyClass::TerrestrialPlanet,
        radius_meters: 3_389_500.0,
        mass_kg: 6.4171e23,
        visual_radius: 0.42,
        color_srgb: [0.9, 0.25, 0.12],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 227_939_200_000.0,
            period_days: 686.980,
            phase_radians: 1.4,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Jupiter,
        name: "Júpiter",
        class: BodyClass::GasGiant,
        radius_meters: 69_911_000.0,
        mass_kg: 1.8982e27,
        visual_radius: 1.40,
        color_srgb: [0.85, 0.62, 0.42],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 778_570_000_000.0,
            period_days: 4_332.589,
            phase_radians: 2.2,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Saturn,
        name: "Saturno",
        class: BodyClass::GasGiant,
        radius_meters: 58_232_000.0,
        mass_kg: 5.6834e26,
        visual_radius: 1.20,
        color_srgb: [0.88, 0.78, 0.52],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 1_433_530_000_000.0,
            period_days: 10_759.22,
            phase_radians: 2.9,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Uranus,
        name: "Urano",
        class: BodyClass::IceGiant,
        radius_meters: 25_362_000.0,
        mass_kg: 8.6810e25,
        visual_radius: 0.95,
        color_srgb: [0.45, 0.85, 0.95],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 2_872_460_000_000.0,
            period_days: 30_688.5,
            phase_radians: 3.6,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Neptune,
        name: "Neptuno",
        class: BodyClass::IceGiant,
        radius_meters: 24_622_000.0,
        mass_kg: 1.02413e26,
        visual_radius: 0.95,
        color_srgb: [0.25, 0.40, 1.0],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 4_495_060_000_000.0,
            period_days: 60_182.0,
            phase_radians: 4.3,
        }),
    },
];

pub fn find_body(id: BodyId) -> Option<&'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == id)
}

pub fn root_bodies() -> impl Iterator<Item = &'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES
        .iter()
        .filter(|body| body.is_root_body())
}

pub fn orbiting_bodies() -> impl Iterator<Item = &'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES.iter().filter(|body| body.has_orbit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_contains_phase_1_bodies() {
        assert_eq!(SOLAR_SYSTEM_BODIES.len(), 10);
        assert!(find_body(BodyId::Sun).is_some());
        assert!(find_body(BodyId::Mercury).is_some());
        assert!(find_body(BodyId::Venus).is_some());
        assert!(find_body(BodyId::Earth).is_some());
        assert!(find_body(BodyId::Moon).is_some());
        assert!(find_body(BodyId::Mars).is_some());
        assert!(find_body(BodyId::Jupiter).is_some());
        assert!(find_body(BodyId::Saturn).is_some());
        assert!(find_body(BodyId::Uranus).is_some());
        assert!(find_body(BodyId::Neptune).is_some());
    }

    #[test]
    fn body_ids_are_unique() {
        let mut ids = HashSet::new();

        for body in SOLAR_SYSTEM_BODIES {
            assert!(ids.insert(body.id), "body id repeated: {:?}", body.id);
        }
    }

    #[test]
    fn only_sun_is_root_body_for_now() {
        let roots: Vec<_> = root_bodies().collect();

        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].id, BodyId::Sun);
    }

    #[test]
    fn every_orbiting_body_has_positive_orbit_values() {
        for body in orbiting_bodies() {
            let orbit = body.orbit.expect("orbiting body should have orbit");

            assert!(orbit.semi_major_axis_meters > 0.0);
            assert!(orbit.period_days > 0.0);
        }
    }

    #[test]
    fn every_non_root_orbit_parent_exists() {
        for body in orbiting_bodies() {
            let parent = body.orbit.expect("orbit should exist").parent;

            assert!(
                find_body(parent).is_some(),
                "parent body does not exist for {:?}",
                body.id
            );
        }
    }

    #[test]
    fn every_body_has_positive_physical_values() {
        for body in SOLAR_SYSTEM_BODIES {
            assert!(body.radius_meters > 0.0);
            assert!(body.mass_kg > 0.0);
            assert!(body.visual_radius > 0.0);
        }
    }
}
