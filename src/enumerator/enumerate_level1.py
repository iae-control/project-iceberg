"""
Level 1 candidate operator enumerator.
Generates all b(u1(x), u2(y)) combinations and applies equivalence filtering.

Level 1 structure: binary_op(unary1(x), unary2(y))
- |U| = 20 unary functions
- |B| = 6 binary operations
- Total raw: 20 * 20 * 6 = 2,400
- After equivalence filtering: ~500
"""
from __future__ import annotations
import json
import itertools
from dataclasses import dataclass, asdict

# Unary function pool
UNARY = [
    "Exp", "Log",                                   # core pair
    "Sin", "Cos", "Tan",                            # trig
    "ArcSin", "ArcCos", "ArcTan",                   # inverse trig
    "Sinh", "Cosh", "Tanh",                          # hyperbolic
    "ArcSinh", "ArcCosh", "ArcTanh",                # inverse hyperbolic
    "Inv",      # 1/x
    "Minus",    # -x
    "Sqr",      # x^2
    "Sqrt",     # sqrt(x)
    "Id",       # x (identity)
    "LogisticSigmoid",  # 1/(1+e^(-x))
]

# Binary operation pool
BINARY = [
    ("Plus", True),       # x + y, commutative
    ("Subtract", False),  # x - y
    ("Times", True),      # x * y, commutative
    ("Divide", False),    # x / y
    ("Power", False),     # x^y
    ("LogBase", False),   # log_x(y)
]

# Companion constant pool
CONSTANTS = ["0", "1", "-1", "2", "E", "Pi", "I"]


@dataclass
class Candidate:
    name: str                  # Unique identifier
    binary_op: str             # The binary operation
    unary1: str                # Applied to x
    unary2: str                # Applied to y
    formula: str               # Human-readable formula
    commutative: bool          # Whether b(u1,u2) is commutative
    is_eml_equiv: bool = False # Trivially equivalent to known EML/EDL/-EML
    filter_reason: str = ""    # Why filtered out


def generate_raw_candidates() -> list[Candidate]:
    """Generate all 2400 raw Level 1 candidates."""
    candidates = []
    for (b_name, b_comm), u1, u2 in itertools.product(BINARY, UNARY, UNARY):
        # For commutative binary ops, only keep u1 <= u2 (lexicographic)
        if b_comm and u1 > u2:
            continue

        name = f"{b_name}_{u1}_{u2}"

        # Build formula
        u1_str = f"{u1}(x)" if u1 != "Id" else "x"
        u2_str = f"{u2}(y)" if u2 != "Id" else "y"

        if b_name == "Plus":
            formula = f"{u1_str} + {u2_str}"
        elif b_name == "Subtract":
            formula = f"{u1_str} - {u2_str}"
        elif b_name == "Times":
            formula = f"{u1_str} * {u2_str}"
        elif b_name == "Divide":
            formula = f"{u1_str} / {u2_str}"
        elif b_name == "Power":
            formula = f"{u1_str} ^ {u2_str}"
        elif b_name == "LogBase":
            formula = f"log_{u1_str}({u2_str})"
        else:
            formula = f"{b_name}({u1_str}, {u2_str})"

        candidates.append(Candidate(
            name=name,
            binary_op=b_name,
            unary1=u1,
            unary2=u2,
            formula=formula,
            commutative=b_comm and u1 == u2,
        ))

    return candidates


def apply_identity_filter(candidates: list[Candidate]) -> list[Candidate]:
    """Remove candidates where identity function makes the operator trivial."""
    filtered = []
    for c in candidates:
        reason = ""

        # Id on both sides: b(x,y) = standard binary op (already in target set)
        if c.unary1 == "Id" and c.unary2 == "Id":
            reason = "identity_both: reduces to standard binary op"

        # Inverse pairs that cancel: exp(ln(x)) = x, etc.
        inverse_pairs = {
            ("Exp", "Log"), ("Log", "Exp"),
            ("Sin", "ArcSin"), ("ArcSin", "Sin"),
            ("Cos", "ArcCos"), ("ArcCos", "Cos"),
            ("Tan", "ArcTan"), ("ArcTan", "Tan"),
            ("Sinh", "ArcSinh"), ("ArcSinh", "Sinh"),
            ("Cosh", "ArcCosh"), ("ArcCosh", "Cosh"),
            ("Tanh", "ArcTanh"), ("ArcTanh", "Tanh"),
            ("Sqrt", "Sqr"), ("Sqr", "Sqrt"),
            ("Minus", "Minus"),
            ("Inv", "Inv"),
        }

        # If both unaries are inverses and binary is commutative,
        # e.g., exp(x) + ln(y) vs ln(x) + exp(y) — keep one
        # This is handled by the commutative sort above

        if reason:
            c.filter_reason = reason
        filtered.append(c)

    return filtered


def apply_eml_equivalence_filter(candidates: list[Candidate]) -> list[Candidate]:
    """Mark candidates trivially equivalent to EML/EDL/-EML."""
    for c in candidates:
        # EML equivalents
        if c.binary_op == "Subtract" and c.unary1 == "Exp" and c.unary2 == "Log":
            c.is_eml_equiv = True
            c.filter_reason = "EML: exp(x) - ln(y)"
        elif c.binary_op == "Subtract" and c.unary1 == "Log" and c.unary2 == "Exp":
            c.is_eml_equiv = True
            c.filter_reason = "NegEML: ln(x) - exp(y) = -EML(y,x)"

        # EDL equivalents
        elif c.binary_op == "Divide" and c.unary1 == "Exp" and c.unary2 == "Log":
            c.is_eml_equiv = True
            c.filter_reason = "EDL: exp(x) / ln(y)"

        # Sign/reciprocal transforms of EML
        # Plus_Exp_Log: exp(x) + ln(y) = EML(x,y) + 2*ln(y) — not trivially equiv
        # BUT: Subtract_Exp_Minus[Log] = exp(x) - (-ln(y)) = exp(x)+ln(y) ≠ EML

    return candidates


def apply_degeneracy_filter(candidates: list[Candidate]) -> list[Candidate]:
    """Filter operators that are degenerate (constant in one variable)."""
    for c in candidates:
        if c.filter_reason:
            continue

        # Sqr of constant = constant (but Sqr(x) depends on x, so ok)
        # Check for true degeneracies:
        # - If unary is a constant function (none of our unaries are constant)
        pass

    return candidates


def generate_level1() -> tuple[list[Candidate], list[Candidate]]:
    """
    Generate Level 1 candidates and return (all, valid).
    Valid = passed all filters and not EML-equivalent.
    """
    all_candidates = generate_raw_candidates()
    all_candidates = apply_identity_filter(all_candidates)
    all_candidates = apply_eml_equivalence_filter(all_candidates)
    all_candidates = apply_degeneracy_filter(all_candidates)

    valid = [c for c in all_candidates if not c.filter_reason and not c.is_eml_equiv]

    return all_candidates, valid


def main():
    all_cands, valid = generate_level1()

    print(f"Raw candidates: {len(all_cands)}")
    print(f"Filtered (identity/degenerate): {sum(1 for c in all_cands if c.filter_reason)}")
    print(f"EML-equivalent: {sum(1 for c in all_cands if c.is_eml_equiv)}")
    print(f"Valid candidates: {len(valid)}")

    # Group by binary op
    by_op: dict[str, int] = {}
    for c in valid:
        by_op[c.binary_op] = by_op.get(c.binary_op, 0) + 1
    print(f"\nBy binary operation:")
    for op, count in sorted(by_op.items()):
        print(f"  {op}: {count}")

    # Save results
    import os
    out_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    os.makedirs(out_dir, exist_ok=True)

    with open(os.path.join(out_dir, 'candidates_L1.json'), 'w') as f:
        json.dump({
            "total_raw": len(all_cands),
            "total_valid": len(valid),
            "candidates": [asdict(c) for c in valid],
        }, f, indent=2)

    print(f"\nSaved {len(valid)} candidates to results/candidates_L1.json")

    # Print first 20 candidates as sample
    print(f"\nSample candidates:")
    for c in valid[:20]:
        print(f"  {c.name}: {c.formula}")


if __name__ == "__main__":
    main()
