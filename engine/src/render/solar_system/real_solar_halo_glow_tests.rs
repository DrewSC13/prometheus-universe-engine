use super::*;

#[test]
fn real_solar_halo_layer_arrays_match_layer_count() {
    assert_eq!(
        REAL_SOLAR_HALO_RADIUS_FACTORS.len(),
        REAL_SOLAR_HALO_LAYER_COUNT
    );
    assert_eq!(
        REAL_SOLAR_HALO_ALPHA_VALUES.len(),
        REAL_SOLAR_HALO_LAYER_COUNT
    );
}

#[test]
fn real_solar_halo_radius_factors_expand_outward() {
    for pair in REAL_SOLAR_HALO_RADIUS_FACTORS.windows(2) {
        assert!(pair[0] < pair[1]);
    }
}

#[test]
fn real_solar_halo_alpha_fades_outward() {
    for pair in REAL_SOLAR_HALO_ALPHA_VALUES.windows(2) {
        assert!(pair[0] > pair[1]);
    }
}

#[test]
fn real_solar_light_constants_are_valid() {
    assert!(REAL_SOLAR_LIGHT_INTENSITY > 0.0);
    assert!(REAL_SOLAR_LIGHT_RANGE > 0.0);
}
