use super::*;

#[test]
fn saturn_ring_mesh_constants_are_valid() {
    assert!(SATURN_RING_MESH_SEGMENTS >= 96);
    assert!(SATURN_RING_MESH_Y_SCALE > 0.0);
    assert!(SATURN_RING_MESH_Y_SCALE <= 1.0);
    assert_eq!(SATURN_RING_MESH_BANDS.len(), 3);
}

#[test]
fn saturn_ring_mesh_bands_are_ordered() {
    for (inner, outer) in SATURN_RING_MESH_BANDS {
        assert!(inner > SATURN_RING_INNER_RADIUS);
        assert!(outer > inner);
        assert!(outer <= SATURN_RING_OUTER_RADIUS);
    }
}

#[test]
fn saturn_ring_mesh_uses_same_projection_as_ring_markers() {
    assert_eq!(SATURN_RING_MESH_Y_SCALE, 0.42);
}
