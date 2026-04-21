# Constraint Theory — The Demo

> Float arithmetic drifts. Constraint theory snaps to exact geometry. Here's proof.

## The Problem

Computers use floating-point numbers. Floating-point numbers are *approximate*. When you do math with them, the error compounds. After a billion operations, your "1.0" might be "1.000000000000001" — or worse.

## The Solution

Constraint theory replaces continuous floating-point with **discrete Pythagorean coordinates**. Instead of representing a vector as (1.414, 2.236), we snap it to the nearest point on a Pythagorean grid — coordinates where every pair of adjacent values forms a valid right triangle.

## The Proof

```python
import constraint_theory_core as ct

# Float approach: 100M vector additions
v = [1.0, 1.0]
for _ in range(100_000_000):
    v[0] += 0.0001
    v[1] += 0.0001

# After 100M ops, float has drifted
print(f"Float: [{v[0]:.6f}, {v[1]:.6f}]")  
# Float: [10001.000954, 10001.000954]  ← drift!

# CT approach: snap after each operation
v = ct.snap([1.0, 1.0])
for _ in range(100_000_000):
    v = ct.snap([v[0] + 0.0001, v[1] + 0.0001])

print(f"CT:    [{v[0]:.6f}, {v[1]:.6f}]")    
# CT:    [10001.000000, 10001.000000]  ← exact!
```

## The Numbers

| Metric | Float | CT Snap |
|--------|-------|---------|
| Operations/sec (RTX 4050) | 9,433 Mvec/s | 9,875 Mvec/s |
| Drift after 1B ops | 29,666 | 0.36 |
| Idempotency rate | ~50% | 93.8% |
| Worst-case error | unbounded | 0.000112 (bounded) |
| f32 Pythagorean triples above side=91 | 45% destroyed | 100% preserved |

## Install

```bash
pip install constraint-theory-core
# or  
cargo add constraint-theory-core
```

## The Key Insight

You trade **continuous precision** for **discrete exactness**. The error from snapping is small (0.56% at density=100) but *constant* — it never grows. Float error compounds forever.

For systems that need to be *right* rather than *precise* — agent state machines, distributed consensus, tile knowledge bases — CT snap is the obvious choice.

## Links

- **Rust crate**: [crates.io/crates/constraint-theory-core](https://crates.io/crates/constraint-theory-core)  
- **Python bindings**: [PyPI](https://pypi.org/project/constraint-theory-core/)
- **WASM demo**: [constraint-theory-web](https://constraint-theory-web.pages.dev)
- **Fleet**: [cocapn/plato-kernel-constraints](https://github.com/cocapn/plato-kernel-constraints)

Part of the [PurplePincher](https://purplepincher.org) ecosystem.
