//! Comparison example: float vs constraint theory arithmetic
use ct_demo::{benchmark, advantage_ratio, drift_accumulate, PythagoreanManifold, snap};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     Constraint Theory vs Float: Drift Comparison           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Snap demo
    let m = PythagoreanManifold::unit();
    let values = [3.14159, 2.71828, 1.61803, 0.0, -3.5];
    println!("Snap Demo (round to nearest integer):");
    println!("  {:>10} → {}", "Value", "Snapped");
    for v in &values {
        println!("  {:>10.5} → {}", v, snap(*v, &m));
    }
    println!();

    // Drift comparison at different scales
    let scales: &[usize] = &[100, 1_000, 10_000, 100_000, 1_000_000];
    println!("Drift Accumulation (σ = 2.2e-16):");
    println!("  {:>12} {:>15} {:>12} {:>12}", "Ops", "Float Drift", "CT Drift", "Ratio");
    println!("  {:>12} {:>15} {:>12} {:>12}", "----", "-----------", "--------", "-----");
    for &ops in scales {
        let fd = drift_accumulate(ops, f64::EPSILON);
        let ct = 0.0;
        let ratio = if ct < 1e-30 { f64::INFINITY } else { fd / ct };
        if ratio.is_infinite() {
            println!("  {:>12} {:>15.6} {:>12.1} {:>12}", ops, fd, ct, "∞");
        } else {
            println!("  {:>12} {:>15.6} {:>12.1} {:>12.1}×", ops, fd, ct, ratio);
        }
    }
    println!();

    // Full benchmark
    println!("{}", benchmark());
}
