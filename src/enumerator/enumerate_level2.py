"""
Level 2 candidate operator enumerator.
Three structural types:
  Type A: b(u1(u2(x)), u3(y))     — left nesting
  Type B: b(u1(x), u2(u3(y)))     — right nesting
  Type C: u0(b(u1(x), u2(y)))     — outer wrapping
"""
import json, os, itertools

UNARY = [
    "Exp", "Log", "Sin", "Cos", "Tan",
    "ArcSin", "ArcCos", "ArcTan",
    "Sinh", "Cosh", "Tanh",
    "ArcSinh", "ArcCosh", "ArcTanh",
    "Inv", "Minus", "Sqr", "Sqrt", "Id", "LogisticSigmoid",
]

BINARY = [
    ("Plus", True), ("Sub", False), ("Mul", True),
    ("Div", False), ("Pow", False), ("LogBase", False),
]

# Inverse pairs: u1(u2(x)) = x => skip
INVERSE_PAIRS = {
    ("Exp","Log"), ("Log","Exp"),
    ("Sin","ArcSin"), ("ArcSin","Sin"),
    ("Cos","ArcCos"), ("ArcCos","Cos"),
    ("Tan","ArcTan"), ("ArcTan","Tan"),
    ("Sinh","ArcSinh"), ("ArcSinh","Sinh"),
    ("Cosh","ArcCosh"), ("ArcCosh","Cosh"),
    ("Tanh","ArcTanh"), ("ArcTanh","Tanh"),
    ("Sqr","Sqrt"), ("Sqrt","Sqr"),
    ("Minus","Minus"), ("Inv","Inv"),
}


def is_identity_collapse(u1, u2):
    """u1(u2(x)) = x"""
    if u1 == "Id" or u2 == "Id":
        return True
    if (u1, u2) in INVERSE_PAIRS:
        return True
    return False


def is_level1_collapse(b, u1, u2, u3):
    """Type A/B collapses to Level 1 when inner unary is Id"""
    # Type A: b(u1(u2(x)), u3(y)) with u2=Id => b(u1(x), u3(y)) = Level 1
    # Type B: b(u1(x), u2(u3(y))) with u3=Id => b(u1(x), u2(y)) = Level 1
    return False  # Already handled by identity collapse


def enumerate_type_a():
    """Type A: b(u1(u2(x)), u3(y)) — left nesting"""
    candidates = []
    for (b, comm), u1, u2, u3 in itertools.product(BINARY, UNARY, UNARY, UNARY):
        if is_identity_collapse(u1, u2):
            continue
        if u3 == "Id" and u1 == "Id":
            continue  # Collapses to b(u2(x), y) = Level 1
        if u2 == "Id":
            continue  # Collapses to b(u1(x), u3(y)) = Level 1
        name = f"A_{b}_{u1}{u2}_{u3}"
        candidates.append({
            "name": name, "type": "A",
            "binary_op": b, "u1": u1, "u2": u2, "u3": u3,
            "formula": f"{b}({u1}({u2}(x)), {u3}(y))",
        })
    return candidates


def enumerate_type_b():
    """Type B: b(u1(x), u2(u3(y))) — right nesting"""
    candidates = []
    for (b, comm), u1, u2, u3 in itertools.product(BINARY, UNARY, UNARY, UNARY):
        if is_identity_collapse(u2, u3):
            continue
        if u1 == "Id" and u3 == "Id":
            continue
        if u3 == "Id":
            continue  # Collapses to Level 1
        name = f"B_{b}_{u1}_{u2}{u3}"
        candidates.append({
            "name": name, "type": "B",
            "binary_op": b, "u1": u1, "u2": u2, "u3": u3,
            "formula": f"{b}({u1}(x), {u2}({u3}(y)))",
        })
    return candidates


def enumerate_type_c():
    """Type C: u0(b(u1(x), u2(y))) — outer wrapping"""
    candidates = []
    for (b, comm), u0, u1, u2 in itertools.product(BINARY, UNARY, UNARY, UNARY):
        if u0 == "Id":
            continue  # Collapses to Level 1
        # For commutative b, only keep u1 <= u2
        if comm and u1 > u2:
            continue
        name = f"C_{u0}_{b}_{u1}_{u2}"
        candidates.append({
            "name": name, "type": "C",
            "binary_op": b, "u0": u0, "u1": u1, "u2": u2,
            "formula": f"{u0}({b}({u1}(x), {u2}(y)))",
        })
    return candidates


def main():
    a = enumerate_type_a()
    b = enumerate_type_b()
    c = enumerate_type_c()

    print(f"Type A (left nesting):  {len(a)} candidates")
    print(f"Type B (right nesting): {len(b)} candidates")
    print(f"Type C (outer wrap):    {len(c)} candidates")
    total = len(a) + len(b) + len(c)
    print(f"Total Level 2:          {total}")

    # Priority: power-based operators first (PLI family cousins)
    all_cands = a + b + c
    power_first = sorted(all_cands, key=lambda c: (
        0 if "Pow" in c["binary_op"] else 1,
        0 if "Log" in c.get("u1","") or "Log" in c.get("u2","") else 1,
        c["name"]
    ))

    # Constants to test
    constants = ["1", "E"]  # Reduced set for speed

    # Estimate time
    runs = total * len(constants)
    # Stage 1 is fast (~10ms each), most fail => ~90% pruned
    stage1_time = runs * 0.01  # seconds
    stage2_runs = int(runs * 0.1)  # ~10% pass Stage 1
    stage2_time = stage2_runs * 0.05  # ~50ms each at K_max=7
    total_est = (stage1_time + stage2_time) / 3600
    print(f"\nWith {len(constants)} constants: {runs} Stage 1 runs")
    print(f"Estimated Stage 1: {stage1_time/60:.0f} min")
    print(f"Estimated Stage 2 (~10% pass): {stage2_runs} runs, {stage2_time/60:.0f} min")
    print(f"Total estimated: {total_est:.1f} hours")

    # Save
    out_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    out_path = os.path.join(out_dir, 'candidates_L2.json')
    with open(out_path, 'w') as f:
        json.dump({
            "type_a_count": len(a),
            "type_b_count": len(b),
            "type_c_count": len(c),
            "total": total,
            "constants": constants,
            "candidates": power_first,
        }, f, indent=2)

    print(f"\nSaved {total} candidates to {out_path}")
    print(f"Priority: power-based operators first")


if __name__ == "__main__":
    main()
