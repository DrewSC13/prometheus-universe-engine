#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyId {
    Sun,

    Mercury,
    Venus,
    Earth,
    Moon,
    Mars,

    Jupiter,
    Io,
    Europa,
    Ganymede,
    Callisto,

    Saturn,
    Titan,
    Enceladus,
    Rhea,

    Uranus,
    Titania,
    Oberon,

    Neptune,
    Triton,
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
    pub physical_radius_meters: f64,
    pub mass_kg: f64,
    pub visual_radius: f32,
    pub color_srgb: [f32; 3],
    pub orbit: Option<OrbitDefinition>,
}

pub const SOLAR_SYSTEM_BODIES: [CelestialBodyDefinition; 21] = [
    CelestialBodyDefinition {
        id: BodyId::Sun,
        name: "Sol",
        class: BodyClass::Star,
        physical_radius_meters: 696_340_000.0,
        mass_kg: 1.9885e30,
        visual_radius: 3.5,
        color_srgb: [1.0, 0.82, 0.28],
        orbit: None,
    },
    CelestialBodyDefinition {
        id: BodyId::Mercury,
        name: "Mercurio",
        class: BodyClass::TerrestrialPlanet,
        physical_radius_meters: 2_439_700.0,
        mass_kg: 3.3011e23,
        visual_radius: 0.22,
        color_srgb: [0.62, 0.59, 0.53],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 57_909_227_000.0,
            period_days: 87.969,
            phase_radians: 0.1,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Venus,
        name: "Venus",
        class: BodyClass::TerrestrialPlanet,
        physical_radius_meters: 6_051_800.0,
        mass_kg: 4.8675e24,
        visual_radius: 0.5,
        color_srgb: [0.90, 0.73, 0.48],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 108_209_475_000.0,
            period_days: 224.701,
            phase_radians: 1.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Earth,
        name: "Tierra",
        class: BodyClass::TerrestrialPlanet,
        physical_radius_meters: 6_371_000.0,
        mass_kg: 5.97237e24,
        visual_radius: 0.55,
        color_srgb: [0.18, 0.42, 0.92],
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
        physical_radius_meters: 1_737_400.0,
        mass_kg: 7.342e22,
        visual_radius: 0.16,
        color_srgb: [0.78, 0.78, 0.74],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Earth,
            semi_major_axis_meters: 384_400_000.0,
            period_days: 27.321661,
            phase_radians: 0.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Mars,
        name: "Marte",
        class: BodyClass::TerrestrialPlanet,
        physical_radius_meters: 3_389_500.0,
        mass_kg: 6.4171e23,
        visual_radius: 0.38,
        color_srgb: [0.92, 0.28, 0.16],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 227_943_824_000.0,
            period_days: 686.980,
            phase_radians: 2.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Jupiter,
        name: "Jupiter",
        class: BodyClass::GasGiant,
        physical_radius_meters: 69_911_000.0,
        mass_kg: 1.8982e27,
        visual_radius: 1.15,
        color_srgb: [0.83, 0.63, 0.43],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 778_340_821_000.0,
            period_days: 4_332.589,
            phase_radians: 0.5,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Io,
        name: "Io",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 1_821_600.0,
        mass_kg: 8.9319e22,
        visual_radius: 0.14,
        color_srgb: [0.95, 0.82, 0.36],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Jupiter,
            semi_major_axis_meters: 421_700_000.0,
            period_days: 1.769,
            phase_radians: 0.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Europa,
        name: "Europa",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 1_560_800.0,
        mass_kg: 4.7998e22,
        visual_radius: 0.13,
        color_srgb: [0.82, 0.78, 0.68],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Jupiter,
            semi_major_axis_meters: 671_100_000.0,
            period_days: 3.551,
            phase_radians: 1.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Ganymede,
        name: "Ganimedes",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 2_634_100.0,
        mass_kg: 1.4819e23,
        visual_radius: 0.18,
        color_srgb: [0.70, 0.64, 0.56],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Jupiter,
            semi_major_axis_meters: 1_070_400_000.0,
            period_days: 7.155,
            phase_radians: 2.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Callisto,
        name: "Calisto",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 2_410_300.0,
        mass_kg: 1.0759e23,
        visual_radius: 0.17,
        color_srgb: [0.48, 0.45, 0.42],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Jupiter,
            semi_major_axis_meters: 1_882_700_000.0,
            period_days: 16.689,
            phase_radians: 3.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Saturn,
        name: "Saturno",
        class: BodyClass::GasGiant,
        physical_radius_meters: 58_232_000.0,
        mass_kg: 5.6834e26,
        visual_radius: 1.05,
        color_srgb: [0.86, 0.76, 0.52],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 1_426_666_422_000.0,
            period_days: 10_759.22,
            phase_radians: 1.2,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Titan,
        name: "Titan",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 2_574_700.0,
        mass_kg: 1.3452e23,
        visual_radius: 0.18,
        color_srgb: [0.91, 0.62, 0.28],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Saturn,
            semi_major_axis_meters: 1_221_870_000.0,
            period_days: 15.945,
            phase_radians: 0.4,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Enceladus,
        name: "Encelado",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 252_100.0,
        mass_kg: 1.08022e20,
        visual_radius: 0.09,
        color_srgb: [0.92, 0.96, 1.0],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Saturn,
            semi_major_axis_meters: 238_020_000.0,
            period_days: 1.370,
            phase_radians: 2.4,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Rhea,
        name: "Rea",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 763_800.0,
        mass_kg: 2.3065e21,
        visual_radius: 0.11,
        color_srgb: [0.72, 0.72, 0.70],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Saturn,
            semi_major_axis_meters: 527_040_000.0,
            period_days: 4.518,
            phase_radians: 1.4,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Uranus,
        name: "Urano",
        class: BodyClass::IceGiant,
        physical_radius_meters: 25_362_000.0,
        mass_kg: 8.6810e25,
        visual_radius: 0.86,
        color_srgb: [0.45, 0.92, 0.96],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 2_870_658_186_000.0,
            period_days: 30_685.4,
            phase_radians: 2.2,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Titania,
        name: "Titania",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 788_900.0,
        mass_kg: 3.527e21,
        visual_radius: 0.11,
        color_srgb: [0.64, 0.62, 0.60],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Uranus,
            semi_major_axis_meters: 435_910_000.0,
            period_days: 8.706,
            phase_radians: 0.7,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Oberon,
        name: "Oberon",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 761_400.0,
        mass_kg: 3.014e21,
        visual_radius: 0.11,
        color_srgb: [0.58, 0.56, 0.54],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Uranus,
            semi_major_axis_meters: 583_520_000.0,
            period_days: 13.463,
            phase_radians: 1.7,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Neptune,
        name: "Neptuno",
        class: BodyClass::IceGiant,
        physical_radius_meters: 24_622_000.0,
        mass_kg: 1.02413e26,
        visual_radius: 0.84,
        color_srgb: [0.22, 0.32, 0.95],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Sun,
            semi_major_axis_meters: 4_498_396_441_000.0,
            period_days: 60_189.0,
            phase_radians: 3.0,
        }),
    },
    CelestialBodyDefinition {
        id: BodyId::Triton,
        name: "Triton",
        class: BodyClass::NaturalSatellite,
        physical_radius_meters: 1_353_400.0,
        mass_kg: 2.14e22,
        visual_radius: 0.13,
        color_srgb: [0.76, 0.80, 0.86],
        orbit: Some(OrbitDefinition {
            parent: BodyId::Neptune,
            semi_major_axis_meters: 354_759_000.0,
            period_days: 5.877,
            phase_radians: 2.7,
        }),
    },
];

pub fn find_body(id: BodyId) -> Option<&'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES.iter().find(|body| body.id == id)
}

pub fn root_bodies() -> impl Iterator<Item = &'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES
        .iter()
        .filter(|body| body.orbit.is_none())
}

pub fn orbiting_bodies() -> impl Iterator<Item = &'static CelestialBodyDefinition> {
    SOLAR_SYSTEM_BODIES
        .iter()
        .filter(|body| body.orbit.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn catalog_contains_phase_1_bodies_and_major_moons() {
        let ids = SOLAR_SYSTEM_BODIES
            .iter()
            .map(|body| body.id)
            .collect::<HashSet<_>>();

        for expected_id in [
            BodyId::Sun,
            BodyId::Mercury,
            BodyId::Venus,
            BodyId::Earth,
            BodyId::Moon,
            BodyId::Mars,
            BodyId::Jupiter,
            BodyId::Io,
            BodyId::Europa,
            BodyId::Ganymede,
            BodyId::Callisto,
            BodyId::Saturn,
            BodyId::Titan,
            BodyId::Enceladus,
            BodyId::Rhea,
            BodyId::Uranus,
            BodyId::Titania,
            BodyId::Oberon,
            BodyId::Neptune,
            BodyId::Triton,
        ] {
            assert!(ids.contains(&expected_id));
        }
    }

    #[test]
    fn body_ids_are_unique() {
        let mut ids = HashSet::new();

        for body in SOLAR_SYSTEM_BODIES {
            assert!(ids.insert(body.id));
        }
    }

    #[test]
    fn only_sun_is_root_body_for_now() {
        let roots = root_bodies().collect::<Vec<_>>();

        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].id, BodyId::Sun);
    }

    #[test]
    fn every_non_root_orbit_parent_exists() {
        for body in orbiting_bodies() {
            let orbit = body.orbit.unwrap();

            assert!(
                find_body(orbit.parent).is_some(),
                "missing parent for {:?}",
                body.id
            );
        }
    }

    #[test]
    fn every_body_has_positive_physical_values() {
        for body in SOLAR_SYSTEM_BODIES {
            assert!(body.physical_radius_meters > 0.0);
            assert!(body.mass_kg > 0.0);
            assert!(body.visual_radius > 0.0);
        }
    }

    #[test]
    fn every_orbiting_body_has_positive_orbit_values() {
        for body in orbiting_bodies() {
            let orbit = body.orbit.unwrap();

            assert!(orbit.semi_major_axis_meters > 0.0);
            assert!(orbit.period_days > 0.0);
        }
    }

    #[test]
    fn jupiter_has_galilean_moons() {
        let jovian_moons = orbiting_bodies()
            .filter(|body| body.orbit.unwrap().parent == BodyId::Jupiter)
            .map(|body| body.id)
            .collect::<HashSet<_>>();

        assert!(jovian_moons.contains(&BodyId::Io));
        assert!(jovian_moons.contains(&BodyId::Europa));
        assert!(jovian_moons.contains(&BodyId::Ganymede));
        assert!(jovian_moons.contains(&BodyId::Callisto));
    }

    #[test]
    fn saturn_has_major_moons() {
        let saturn_moons = orbiting_bodies()
            .filter(|body| body.orbit.unwrap().parent == BodyId::Saturn)
            .map(|body| body.id)
            .collect::<HashSet<_>>();

        assert!(saturn_moons.contains(&BodyId::Titan));
        assert!(saturn_moons.contains(&BodyId::Enceladus));
        assert!(saturn_moons.contains(&BodyId::Rhea));
    }
}
