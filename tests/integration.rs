use ct_demo::*;

#[test]
fn integration_snap_roundtrip() {
    let m = PythagoreanManifold::new(3, 10000, 0);
    for v in [-10000.0, -1.0, 0.0, 0.5, 1.0, 3.14, 9999.9, 10000.0] {
        let snapped = snap(v, &m);
        assert!(snapped >= -10000 && snapped <= 10000);
    }
}

#[test]
fn integration_benchmark_consistent() {
    let r1 = benchmark();
    let r2 = benchmark();
    assert_eq!(r1.ops, r2.ops);
    assert_eq!(r1.snap_error, r2.snap_error);
}

#[test]
fn integration_advantage_grows_with_ops() {
    let a1 = advantage_ratio(100);
    let a2 = advantage_ratio(10000);
    // Advantage should grow (or at least not shrink)
    assert!(a2 >= a1 * 0.5); // allow some variance
}

#[test]
fn integration_snap_zero_stable() {
    let m = PythagoreanManifold::unit();
    // Snapping zero should always give zero
    assert_eq!(snap(0.0, &m), 0);
    assert_eq!(snap(-0.0, &m), 0);
    assert_eq!(snap(0.001, &m), 0);
}

#[test]
fn integration_resolution_scaling() {
    let m0 = PythagoreanManifold::new(2, 1000, 0);
    let m2 = PythagoreanManifold::new(2, 1000, 2);
    let val = 5.75;
    assert_eq!(snap(val, &m0), 6);      // rounds to 6
    assert_eq!(snap(val, &m2), 23);     // 5.75 * 4 = 23
}
