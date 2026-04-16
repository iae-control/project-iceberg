"""
Test: Reproduce EML, EDL, -EML verification.
These three operators are known Sheffer operators from Odrzywołek (2026).
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from verifier.complex_ops import C
from verifier.engine import verify_operator
from typing import Optional


def eml_op(a: C, b: C) -> Optional[C]:
    """EML[a,b] = exp(a) - ln(b)"""
    lb = b.ln()
    if lb is None:
        return None
    return a.exp().sub(lb)


def edl_op(a: C, b: C) -> Optional[C]:
    """EDL[a,b] = exp(a) / ln(b)"""
    lb = b.ln()
    if lb is None:
        return None
    return a.exp().div(lb)


def neg_eml_op(a: C, b: C) -> Optional[C]:
    """-EML[a,b] = ln(a) - exp(b)"""
    la = a.ln()
    if la is None:
        return None
    return la.sub(b.exp())


def test_eml():
    """EML(x,y) = exp(x) - ln(y) with constant 1"""
    print("\n" + "="*60)
    print("TEST: EML with constant 1")
    print("="*60)
    result = verify_operator(
        op_name="EML",
        op_fn=eml_op,
        op_commutative=False,
        companion_constants=["1"],
        max_k=9,
        verbose=True,
    )
    return result


def test_edl():
    """EDL(x,y) = exp(x) / ln(y) with constant e"""
    print("\n" + "="*60)
    print("TEST: EDL with constant E")
    print("="*60)
    result = verify_operator(
        op_name="EDL",
        op_fn=edl_op,
        op_commutative=False,
        companion_constants=["E"],
        max_k=9,
        verbose=True,
    )
    return result


def test_neg_eml():
    """-EML(x,y) = ln(x) - exp(y) with constant -inf (mapped to constant 0 + special handling)"""
    # Note: The paper uses -inf as companion constant.
    # ln(x)-exp(y) with x=e^(-inf)=0 gives ln(0)-exp(0) which is undefined.
    # We approximate by using constants that allow bootstrapping.
    # Actually -EML is just EML with arguments swapped and negated.
    # -EML(x,y) = ln(x) - exp(y) = -(exp(y) - ln(x)) = -EML(y,x)
    # So -EML with any constant c behaves like -EML(c,c) = ln(c) - exp(c)
    # The paper states -EML uses constant "-inf" which in practice means:
    # Using 0 as the companion constant won't work directly.
    # Let's try with companion 0 first, then explore alternatives.
    print("\n" + "="*60)
    print("TEST: -EML (ln(x)-exp(y)) with constant 0")
    print("="*60)
    result = verify_operator(
        op_name="NegEML",
        op_fn=neg_eml_op,
        op_commutative=False,
        companion_constants=["0"],
        max_k=9,
        verbose=True,
    )
    return result


if __name__ == "__main__":
    import json

    results = {}

    r1 = test_eml()
    results["EML"] = r1.to_dict()

    r2 = test_edl()
    results["EDL"] = r2.to_dict()

    r3 = test_neg_eml()
    results["NegEML"] = r3.to_dict()

    # Save results
    out_path = os.path.join(os.path.dirname(__file__), '..', 'results', 'repro_results.json')
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved to {out_path}")

    # Summary
    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    for name, r in results.items():
        status = "SHEFFER" if r["is_sheffer"] else f"PARTIAL ({r['total_found']}/{r['total_targets']})"
        print(f"  {name}: {status} ({r['elapsed_seconds']}s)")
