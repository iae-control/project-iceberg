"""
Level 2 Stage 1 proper filter: K_max=3 numerical sieve.
Mirrors the author's VerifyBaseSet approach:
  - Generate all K<=3 RPN programs from {constant, candidate_op}
  - Evaluate each at transcendental test points
  - Compare to 4 target values with epsilon=1e-10
  - Reject if 0/4 targets hit

Expected pass rate: <5% (vs previous 99%).
"""
import json, os, sys, time, cmath, math
from typing import Optional, Callable

GAMMA = 0.5772156649015329
GLAISHER = 1.2824271291006227

# 4 target values (evaluated at x=gamma, y=glaisher)
TARGETS = {
    "exp": cmath.exp(GAMMA),        # 1.78107...
    "ln":  cmath.log(GAMMA),        # -0.54953...
    "add": GAMMA + GLAISHER,        # 1.85964...
    "mul": GAMMA * GLAISHER,        # 0.74044...
}

EPS = 1e-10


def near(a: complex, b: complex) -> bool:
    return abs(a - b) <= EPS * (1.0 + abs(a) + abs(b))


# Unary function implementations (complex-valued)
def apply_u(name: str, z: complex) -> Optional[complex]:
    try:
        if name == "Exp": return cmath.exp(z)
        if name == "Log": return cmath.log(z) if z != 0 else None
        if name == "Sin": return cmath.sin(z)
        if name == "Cos": return cmath.cos(z)
        if name == "Tan": return cmath.tan(z)
        if name == "ArcSin": return cmath.asin(z)
        if name == "ArcCos": return cmath.acos(z)
        if name == "ArcTan": return cmath.atan(z)
        if name == "Sinh": return cmath.sinh(z)
        if name == "Cosh": return cmath.cosh(z)
        if name == "Tanh": return cmath.tanh(z)
        if name == "ArcSinh": return cmath.asinh(z)
        if name == "ArcCosh": return cmath.acosh(z)
        if name == "ArcTanh": return cmath.atanh(z)
        if name == "Inv": return 1/z if z != 0 else None
        if name == "Minus": return -z
        if name == "Sqr": return z*z
        if name == "Sqrt": return cmath.sqrt(z)
        if name == "Id": return z
        if name == "LogisticSigmoid": return 1/(1+cmath.exp(-z))
    except (OverflowError, ValueError, ZeroDivisionError):
        return None
    return None


def apply_b(name: str, a: complex, b: complex) -> Optional[complex]:
    try:
        if name == "Plus": return a + b
        if name == "Sub": return a - b
        if name == "Mul": return a * b
        if name == "Div": return a / b if b != 0 else None
        if name == "Pow": return a ** b if a != 0 else None
        if name == "LogBase":
            la = cmath.log(a)
            return cmath.log(b) / la if la != 0 else None
    except (OverflowError, ValueError, ZeroDivisionError):
        return None
    return None


def make_op_fn(cand: dict) -> Callable[[complex, complex], Optional[complex]]:
    """Build a callable for the Level 2 candidate operator."""
    t = cand["type"]

    if t == "A":
        # b(u1(u2(x)), u3(y))
        u1, u2, u3, bop = cand["u1"], cand["u2"], cand["u3"], cand["binary_op"]
        def fn(x, y):
            v1 = apply_u(u2, x)
            if v1 is None: return None
            v2 = apply_u(u1, v1)
            if v2 is None: return None
            v3 = apply_u(u3, y)
            if v3 is None: return None
            return apply_b(bop, v2, v3)
        return fn

    elif t == "B":
        # b(u1(x), u2(u3(y)))
        u1, u2, u3, bop = cand["u1"], cand["u2"], cand["u3"], cand["binary_op"]
        def fn(x, y):
            v1 = apply_u(u1, x)
            if v1 is None: return None
            v3 = apply_u(u3, y)
            if v3 is None: return None
            v2 = apply_u(u2, v3)
            if v2 is None: return None
            return apply_b(bop, v1, v2)
        return fn

    elif t == "C":
        # u0(b(u1(x), u2(y)))
        u0, u1, u2, bop = cand["u0"], cand["u1"], cand["u2"], cand["binary_op"]
        def fn(x, y):
            v1 = apply_u(u1, x)
            if v1 is None: return None
            v2 = apply_u(u2, y)
            if v2 is None: return None
            bv = apply_b(bop, v1, v2)
            if bv is None: return None
            return apply_u(u0, bv)
        return fn

    return lambda x, y: None


def sieve_k3(op_fn: Callable, const_val: complex) -> int:
    """
    Generate all K<=3 RPN programs from {const, op} and check
    how many of the 4 targets are hit.

    K=1: just the constant
    K=3: op(a, b) where a,b in {const, gamma_probe, glaisher_probe}
         But we don't have gamma/glaisher as "symbols" in the RPN.
         The author's approach uses gamma and glaisher as EVALUATION POINTS,
         not as program tokens.

    Correct approach: the RPN program has tokens {const, op}.
    K=1: const (just the number)
    K=3: op(const, const) — one application
    We evaluate each program at (x=gamma, y=glaisher).

    But wait — the operator is binary: op(x,y).
    At K=1, we have the constant (c) and the anchors (gamma, glaisher).
    At K=3, we form op(a, b) for all pairs a,b from K=1 values.

    K=1 pool: {c, gamma, glaisher}  (3 values)
    K=3 pool: {op(a,b) for a,b in K=1} + unary applications
              But in the Sheffer context, the ONLY operation is the candidate op.
              So K=3 = {op(a,b) : a,b in K=1_pool}
    """
    gamma = complex(GAMMA)
    glaisher = complex(GLAISHER)

    # K=1 values: constant + evaluation anchors
    pool_1 = set()
    for v in [const_val, gamma, glaisher]:
        if v is not None and cmath.isfinite(v):
            pool_1.add(v)

    # Check K=1 values against targets
    hits = set()
    for v in pool_1:
        for tname, tval in TARGETS.items():
            if near(v, tval):
                hits.add(tname)

    if len(hits) >= 4:
        return len(hits)

    # K=3 values: op(a, b) for all pairs from K=1
    pool_3 = set()
    for a in pool_1:
        for b in pool_1:
            try:
                v = op_fn(a, b)
            except:
                continue
            if v is not None and cmath.isfinite(v):
                pool_3.add(v)
                for tname, tval in TARGETS.items():
                    if near(v, tval):
                        hits.add(tname)

    return len(hits)


def main():
    results_dir = os.path.join(os.path.dirname(__file__), "..", "..", "results")
    cand_file = os.path.join(results_dir, "candidates_L2.json")

    with open(cand_file) as f:
        data = json.load(f)

    candidates = data["candidates"]
    constants_map = {"1": 1.0, "E": math.e}
    constants = data.get("constants", ["1", "E"])

    total = len(candidates) * len(constants)
    print(f"Level 2 Stage 1 proper filter: K_max=3, 4-target sieve")
    print(f"Candidates: {len(candidates)}, Constants: {constants}")
    print(f"Total checks: {total}")
    print(f"Targets: exp(γ)={TARGETS['exp']:.4f}, ln(γ)={TARGETS['ln']:.4f}, "
          f"γ+A={TARGETS['add']:.4f}, γ×A={TARGETS['mul']:.4f}")
    print()

    t0 = time.time()
    passed = []
    count = 0
    hist = {0: 0, 1: 0, 2: 0, 3: 0, 4: 0}

    for c in candidates:
        op_fn = make_op_fn(c)
        for const_name in constants:
            count += 1
            const_val = complex(constants_map[const_name])
            nhits = sieve_k3(op_fn, const_val)
            hist[min(nhits, 4)] = hist.get(min(nhits, 4), 0) + 1

            if nhits >= 1:  # At least 1 target hit at K=3
                passed.append({
                    "name": c["name"],
                    "const": const_name,
                    "hits_k3": nhits,
                    "candidate": c,
                })

            if count % 50000 == 0:
                elapsed = time.time() - t0
                rate = count / elapsed
                eta = (total - count) / rate
                print(f"  {count}/{total} ({100*count/total:.0f}%) "
                      f"passed={len(passed)} ({100*len(passed)/count:.1f}%) "
                      f"ETA={eta:.0f}s", flush=True)

    elapsed = time.time() - t0
    print(f"\nStage 1 complete: {elapsed:.0f}s")
    print(f"Total: {count}, Passed: {len(passed)} ({100*len(passed)/count:.1f}%)")
    print(f"Hit distribution: {hist}")

    # Save
    out_path = os.path.join(results_dir, "L2_stage1_filtered.json")
    with open(out_path, "w") as f:
        json.dump({
            "total_checked": count,
            "total_passed": len(passed),
            "pass_rate": len(passed) / count if count > 0 else 0,
            "hit_distribution": hist,
            "candidates": passed,
        }, f, indent=2)

    print(f"Saved to {out_path}")

    # Top hits
    by_hits = sorted(passed, key=lambda x: -x["hits_k3"])
    print(f"\nTop 20 by K=3 hits:")
    for p in by_hits[:20]:
        print(f"  {p['hits_k3']}/4  {p['name']} + {p['const']}")


if __name__ == "__main__":
    main()
