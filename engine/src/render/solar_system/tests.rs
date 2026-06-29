#[test]
fn planetary_detail_body_selection_is_stable() {
    assert!(is_planetary_detail_body(super::BodyId::Earth));
    assert!(is_planetary_detail_body(super::BodyId::Jupiter));
    assert!(is_planetary_detail_body(super::BodyId::Neptune));
    assert!(!is_planetary_detail_body(super::BodyId::Sun));
    assert!(!is_planetary_detail_body(super::BodyId::Moon));
}

#[test]
fn planet_band_visuals_are_limited_to_jupiter_and_saturn() {
    assert!(has_planet_band_visual(super::BodyId::Jupiter));
    assert!(has_planet_band_visual(super::BodyId::Saturn));
    assert!(!has_planet_band_visual(super::BodyId::Earth));
    assert!(!has_planet_band_visual(super::BodyId::Neptune));
}

#[test]
fn planet_surface_feature_counts_are_positive() {
    assert!(planet_surface_feature_count(super::BodyId::Mercury) > 0);
    assert!(planet_surface_feature_count(super::BodyId::Jupiter) > PLANET_SURFACE_FEATURE_COUNT);
}

#[test]
fn planet_surface_feature_scale_stays_positive() {
    let scale = planet_surface_feature_scale(12, 1.0);

    assert!(scale > 0.0);
}

#[test]
fn planet_surface_material_index_stays_in_range() {
    for index in 0..64 {
        assert!(planet_surface_material_index(super::BodyId::Earth, index) < 5);
        assert!(planet_surface_material_index(super::BodyId::Mars, index) < 5);
        assert!(planet_surface_material_index(super::BodyId::Jupiter, index) < 5);
    }
}

#[test]
fn planet_band_y_factors_exist_only_for_jupiter_and_saturn() {
    assert!(planet_band_y_factors(super::BodyId::Jupiter).is_some());
    assert!(planet_band_y_factors(super::BodyId::Saturn).is_some());
    assert!(planet_band_y_factors(super::BodyId::Earth).is_none());
}

#[test]
fn planet_surface_direction_is_normalized() {
    let direction = planet_surface_direction(8, 64, super::BodyId::Earth, 0.4);

    assert!((direction.length() - 1.0).abs() < 0.0001);
}

#[test]
fn planet_band_constants_are_valid() {
    assert!(PLANET_BAND_MARKERS >= 96);
    assert!(PLANET_BAND_MARKER_RADIUS > 0.0);
    assert!(JUPITER_BAND_Y_FACTORS.len() > SATURN_BAND_Y_FACTORS.len());
}

use super::labels::{is_major_body_label, label_vertical_offset};
use super::planet_surface::{
    has_planet_band_visual, is_planetary_detail_body, planet_band_y_factors,
    planet_surface_direction, planet_surface_feature_count, planet_surface_feature_scale,
    planet_surface_material_index, JUPITER_BAND_Y_FACTORS, PLANET_BAND_MARKERS,
    PLANET_BAND_MARKER_RADIUS, PLANET_SURFACE_FEATURE_COUNT, SATURN_BAND_Y_FACTORS,
};
use super::*;
use crate::simulation::bodies::OrbitDefinition;

#[test]
fn label_visibility_mode_cycles_in_expected_order() {
    assert_eq!(
        LabelVisibilityMode::MajorOnly.next(),
        LabelVisibilityMode::All
    );
    assert_eq!(LabelVisibilityMode::All.next(), LabelVisibilityMode::None);
    assert_eq!(
        LabelVisibilityMode::None.next(),
        LabelVisibilityMode::MajorOnly
    );
}

#[test]
fn orbit_visibility_mode_cycles_in_expected_order() {
    assert_eq!(
        OrbitVisibilityMode::All.next(),
        OrbitVisibilityMode::PlanetaryOnly
    );
    assert_eq!(
        OrbitVisibilityMode::PlanetaryOnly.next(),
        OrbitVisibilityMode::None
    );
    assert_eq!(OrbitVisibilityMode::None.next(), OrbitVisibilityMode::All);
}

#[test]
fn major_labels_include_planets_but_not_moon() {
    assert!(is_major_body_label(BodyId::Sun));
    assert!(is_major_body_label(BodyId::Earth));
    assert!(is_major_body_label(BodyId::Jupiter));
    assert!(!is_major_body_label(BodyId::Moon));
}

#[test]
fn planetary_orbits_exclude_moon_orbit() {
    assert!(is_planetary_orbit(BodyId::Earth));
    assert!(is_planetary_orbit(BodyId::Jupiter));
    assert!(!is_planetary_orbit(BodyId::Moon));
}

#[test]
fn satellite_orbit_visual_radius_scales_with_distance() {
    let moon_orbit = OrbitDefinition {
        parent: BodyId::Earth,
        semi_major_axis_meters: 384_400_000.0,
        period_days: 27.0,
        phase_radians: 0.0,
    };

    let titan_orbit = OrbitDefinition {
        parent: BodyId::Saturn,
        semi_major_axis_meters: 1_221_870_000.0,
        period_days: 16.0,
        phase_radians: 0.0,
    };

    assert!(educational_orbit_radius(titan_orbit) > educational_orbit_radius(moon_orbit));
}

#[test]
fn saturn_has_ring_visual() {
    assert!(has_ring_visual(BodyId::Saturn));
    assert!(!has_ring_visual(BodyId::Earth));
    assert!(!has_ring_visual(BodyId::Jupiter));
}

#[test]
fn saturn_ring_constants_are_valid() {
    assert!(SATURN_RING_MARKERS >= 64);
    assert!(SATURN_RING_INNER_RADIUS > 0.0);
    assert!(SATURN_RING_OUTER_RADIUS > SATURN_RING_INNER_RADIUS);
    assert!(SATURN_RING_MARKER_RADIUS > 0.0);
}

#[test]
fn starfield_constants_are_valid() {
    assert!(STARFIELD_STAR_COUNT >= 900);
    assert!(STARFIELD_RADIUS > 300.0);
    assert!(STARFIELD_MAX_SCALE > STARFIELD_MIN_SCALE);
    assert!(STARFIELD_MIN_SCALE > 0.0);
}

#[test]
fn starfield_positions_stay_on_shell() {
    let position = starfield_position(0);
    let distance = position.length();

    assert!((distance - STARFIELD_RADIUS).abs() < 0.01);
}

#[test]
fn starfield_scale_stays_in_range() {
    let scale = starfield_scale(42);

    assert!(scale >= STARFIELD_MIN_SCALE);
    assert!(scale <= STARFIELD_MAX_SCALE);
}

#[test]
fn starfield_material_index_stays_in_range() {
    for index in 0..128 {
        assert!(starfield_material_index(index) < 3);
    }
}

#[test]
fn solar_surface_constants_are_valid() {
    assert!(SOLAR_SURFACE_FEATURE_COUNT >= 128);
    assert!(SOLAR_SURFACE_RADIUS_FACTOR > 1.0);
    assert!(SOLAR_SURFACE_MAX_SCALE > SOLAR_SURFACE_MIN_SCALE);
}

#[test]
fn solar_corona_constants_are_valid() {
    assert!(SOLAR_CORONA_MARKERS_PER_SHELL >= 128);
    assert!(SOLAR_CORONA_INNER_RADIUS_FACTOR > 1.0);
    assert!(SOLAR_CORONA_OUTER_RADIUS_FACTOR > SOLAR_CORONA_INNER_RADIUS_FACTOR);
    assert!(SOLAR_CORONA_INNER_SCALE > SOLAR_CORONA_OUTER_SCALE);
}

#[test]
fn solar_surface_direction_is_normalized() {
    let direction = solar_surface_direction(12, 0.25);

    assert!((direction.length() - 1.0).abs() < 0.0001);
}

#[test]
fn solar_corona_direction_is_normalized() {
    let direction = solar_corona_direction(12, 1, 0.25);

    assert!((direction.length() - 1.0).abs() < 0.0001);
}

#[test]
fn solar_surface_feature_scale_stays_in_range() {
    let scale = solar_surface_feature_scale(17);

    assert!(scale >= SOLAR_SURFACE_MIN_SCALE);
    assert!(scale <= SOLAR_SURFACE_MAX_SCALE);
}

#[test]
fn solar_surface_material_index_stays_in_range() {
    for index in 0..64 {
        assert!(solar_surface_material_index(index) < 3);
    }
}

#[test]
fn star_label_is_larger_than_earth_label() {
    let sun = SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Sun)
        .unwrap();

    let earth = SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Earth)
        .unwrap();

    assert!(label_font_size(sun) > label_font_size(earth));
}

#[test]
fn star_label_has_larger_vertical_offset_than_moon_label() {
    let sun = SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Sun)
        .unwrap();

    let moon = SOLAR_SYSTEM_BODIES
        .iter()
        .find(|body| body.id == BodyId::Moon)
        .unwrap();

    assert!(label_vertical_offset(sun) > label_vertical_offset(moon));
}

#[test]
fn selected_body_indicator_scale_exceeds_body_radius() {
    let body_radius = 1.0;

    assert!(super::selected_body_indicator_scale(body_radius) > body_radius);
}

#[test]
fn selected_body_indicator_scale_keeps_small_bodies_visible() {
    assert!(super::selected_body_indicator_scale(0.1) >= 0.40);
}

#[test]
fn solar_body_visual_position_exposes_sun_position() {
    assert_eq!(
        super::solar_body_visual_position(super::BodyId::Sun, 0.0),
        Some(super::Vec3::ZERO)
    );
}

#[test]
fn selected_body_indicator_scale_is_tighter_for_large_bodies() {
    let sun_radius = 3.5;
    let scale = super::selected_body_indicator_scale(sun_radius);

    assert!(scale < sun_radius * 1.20);
    assert!(scale > sun_radius);
}

#[test]
fn selected_body_indicator_pulse_multiplier_stays_subtle() {
    for days in [0.0, 0.5, 1.0, 1.5, 2.0] {
        let multiplier = super::selected_body_indicator_pulse_multiplier(days);

        assert!((0.96..=1.04).contains(&multiplier));
    }
}

#[test]
fn selected_body_indicator_pulsed_scale_preserves_small_body_visibility() {
    let base_scale = super::selected_body_indicator_scale(0.1);
    let pulsed_scale = super::selected_body_indicator_pulsed_scale(0.1, 1.5);

    assert!(pulsed_scale >= base_scale * 0.96);
}
