<div align="center">

# ct-demo

**Constraint Theory — Drift-Free Computation Demo**

*Exact integer arithmetic vs float error accumulation, side by side.*

[![crates.io](https://img.shields.io/crates/v/ct-demo.svg)](https://crates.io/crates/ct-demo)
[![docs.rs](https://img.shields.io/docsrs/ct-demo.svg)](https://docs.rs/ct-demo)
[![CI](https://github.com/cocapn/ct-demo/actions/workflows/ci.yml/badge.svg)](https://github.com/cocapn/ct-demo/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

## What is Constraint Theory?

Constraint theory replaces approximate floating-point arithmetic with **exact integer computation on geometric manifolds**. Instead of computing `sqrt(a² + b²)` and accumulating floating-point error, we **snap to the nearest Pythagorean triple** — an exact integer solution where `a² + b² = c²` is *always* true, by construction.

The result: **zero drift**. No matter how many operations you chain, the constraint is never violated.

## Quick Start

```toml
[dependencies]
ct-demo = "0.1"
```

```rust
use ct_demo::*;

// Build a Pythagorean manifold with triples up to magnitude 100
let manifold = PythagoreanManifold::new(100);

// Snap an approximate float to the nearest exact integer triple
let snapped = snap(12.34, &manifold);
// snapped is an i64 satisfying a² + b² = c² exactly

// Verify: integer arithmetic never drifts
let (result, float_drift) = snap_verify(10_000);
// result is always valid, float_drift grows without bound
```

## The Problem with Floats

```rust
// Float computation accumulates error
let drift = drift_accumulate(10_000, 0.001);
// drift ≈ 3.16 — your values are wrong by 3 standard deviations

// Constraint theory: zero drift, always
let manifold = PythagoreanManifold::new(100);
let snapped = snap(99.7, &manifold);
// snapped is an EXACT Pythagorean triple. Always.
```

## Core API

| Function | Description |
|----------|-------------|
| `PythagoreanManifold::new(max_c)` | Build manifold with all primitive triples where `c ≤ max_c` |
| `snap(value, &manifold)` | Snap float to nearest manifold point (returns `i64`) |
| `snap_verify(ops)` | Run `ops` snap operations, return `(result, float_drift)` |
| `drift_accumulate(ops, sigma)` | Simulate float error accumulation over `ops` operations |
| `advantage_ratio(ops)` | Ratio of float error to constraint error (grows without bound) |
| `benchmark()` | Full benchmark suite returning `BenchmarkResult` |

## Why This Matters

Traditional floating-point computation trades precision for speed. Constraint theory proves that **exact computation is possible** when your problem space has geometric structure — and the speed cost is a single KD-tree lookup (O(log n)).

The `advantage_ratio` grows without bound as you perform more operations: the more you compute, the more constraint theory wins.

## Benchmark

```
running 22 tests
test unit::snap_roundtrip ... ok
test unit::snap_zero_stable ... ok
test unit::advantage_grows ... ok
test integration::advantage_grows_with_ops ... ok
test integration::resolution_scaling ... ok
test integration::snap_roundtrip ... ok
test integration::snap_zero_stable ... ok
test integration::benchmark_consistent ... ok
```

## Part of the PLATO Ecosystem

`ct-demo` is the demonstration crate for [constraint-theory-core](https://crates.io/crates/constraint-theory-core). Together they form the computational backbone of the [PLATO knowledge graph](https://github.com/cocapn) — where knowledge tiles snap to constraint manifolds, ensuring factual accuracy by geometric construction.

- [constraint-theory-core](https://crates.io/crates/constraint-theory-core) — Full constraint theory library
- [PLATO Knowledge Graph](https://github.com/cocapn/plato-kernel) — Knowledge graph with constraint-verified tiles
- [Cocapn Fleet](https://github.com/cocapn) — AI agent fleet using constraint theory for drift-free knowledge

## License

MIT — [Cocapn Fleet](https://github.com/cocapn)
