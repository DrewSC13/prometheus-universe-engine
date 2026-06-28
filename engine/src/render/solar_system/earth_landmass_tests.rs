use super::*;

    #[test]
    fn earth_landmass_cluster_arrays_match_count() {
        assert_eq!(
            EARTH_LANDMASS_CLUSTER_SEEDS.len(),
            EARTH_LANDMASS_CLUSTER_COUNT
        );
        assert_eq!(
            EARTH_LANDMASS_CLUSTER_THRESHOLDS.len(),
            EARTH_LANDMASS_CLUSTER_COUNT
        );
    }

    #[test]
    fn earth_landmass_constants_are_valid() {
        assert!(EARTH_LANDMASS_SAMPLE_COUNT >= 256);
        assert!(EARTH_LANDMASS_RADIUS_FACTOR > 1.0);
        assert!(EARTH_LANDMASS_MIN_SCALE > 0.0);
        assert!(EARTH_LANDMASS_MAX_SCALE > EARTH_LANDMASS_MIN_SCALE);
    }

    #[test]
    fn earth_landmass_direction_is_normalized() {
        let direction = earth_landmass_direction(12, 128, 0.25);

        assert!((direction.length() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn earth_landmass_cluster_strength_stays_in_range() {
        let direction = earth_landmass_direction(32, EARTH_LANDMASS_SAMPLE_COUNT, 0.0);
        let strength = earth_landmass_cluster_strength(direction);

        assert!((0.0..=1.0).contains(&strength));
    }

    #[test]
    fn earth_landmass_feature_scale_stays_positive() {
        let scale = earth_landmass_feature_scale(18, 0.65, 0.6);

        assert!(scale > 0.0);
    }

    #[test]
    fn earth_landmass_visible_sample_count_is_reasonable() {
        let count = earth_landmass_visible_sample_count();

        assert!(count >= 32);
        assert!(count < EARTH_LANDMASS_SAMPLE_COUNT);
    }
