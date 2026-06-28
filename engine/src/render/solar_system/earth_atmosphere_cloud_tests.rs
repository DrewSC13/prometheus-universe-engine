use super::*;

#[test]
fn earth_atmosphere_arrays_match_layer_count() {
    assert_eq!(
        EARTH_ATMOSPHERE_RADIUS_FACTORS.len(),
        EARTH_ATMOSPHERE_LAYER_COUNT
    );
    assert_eq!(
        EARTH_ATMOSPHERE_ALPHA_VALUES.len(),
        EARTH_ATMOSPHERE_LAYER_COUNT
    );
}

#[test]
fn earth_atmosphere_radius_expands_outward() {
    for pair in EARTH_ATMOSPHERE_RADIUS_FACTORS.windows(2) {
        assert!(pair[0] < pair[1]);
    }
}

#[test]
fn earth_atmosphere_alpha_fades_outward() {
    for pair in EARTH_ATMOSPHERE_ALPHA_VALUES.windows(2) {
        assert!(pair[0] > pair[1]);
    }
}

#[test]
fn earth_cloud_constants_are_valid() {
    assert!(EARTH_CLOUD_FEATURE_COUNT >= 96);
    assert!(EARTH_CLOUD_RADIUS_FACTOR > 1.0);
    assert!(EARTH_CLOUD_MIN_SCALE > 0.0);
    assert!(EARTH_CLOUD_MAX_SCALE > EARTH_CLOUD_MIN_SCALE);
}

#[test]
fn earth_cloud_direction_is_normalized() {
    let direction = earth_cloud_direction(8, 64, 0.4);

    assert!((direction.length() - 1.0).abs() < 0.0001);
}

#[test]
fn earth_cloud_feature_scale_stays_positive() {
    let scale = earth_cloud_feature_scale(12, 0.65);

    assert!(scale > 0.0);
}
