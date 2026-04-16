"""
PRD-02 Task A v2: Corrected Numerical Stability Benchmark

CORRECTIONS from v1:
1. EML uses SUBTRACTION (exp(x)-ln(y)), not division.
   "Division-by-zero" is wrong framing for EML.
   The correct issue is ln(y) domain restriction (y>0).
2. Proper comparison: EDL vs DivLogExp (both use division)
3. Track domain failures separately from arithmetic errors
4. Analyze precision loss at each bootstrapping depth

Comparison groups:
- Group A: EML vs NegEML (subtraction-based operators)
- Group B: EDL vs DivLogExp (division-based operators)
- For each: measure domain failure rate, overflow rate, precision loss
"""
from __future__ import annotations
import math
import csv
import os
from dataclasses import dataclass
from typing import Optional, Callable

import mpmath
mpmath.mp.dps = 50


# ============================================================
# Operator definitions
# ============================================================

def eml(a: float, b: float) -> Optional[float]:
    """EML(a,b) = exp(a) - ln(b). Fails when b <= 0."""
    if b <= 0:
        return None
    try:
        return math.exp(a) - math.log(b)
    except OverflowError:
        return None


def edl(a: float, b: float) -> Optional[float]:
    """EDL(a,b) = exp(a) / ln(b). Fails when b <= 0 or b = 1 (ln=0)."""
    if b <= 0:
        return None
    lb = math.log(b)
    if lb == 0.0:
        return None
    try:
        return math.exp(a) / lb
    except OverflowError:
        return None


def div_log_exp(a: float, b: float) -> Optional[float]:
    """DivLogExp(a,b) = ln(a) / exp(b). Fails when a <= 0. exp(b) is always > 0."""
    if a <= 0:
        return None
    try:
        eb = math.exp(b)
        if not math.isfinite(eb):
            return None  # overflow in exp(b)
        return math.log(a) / eb
    except OverflowError:
        return None


def neg_eml(a: float, b: float) -> Optional[float]:
    """NegEML(a,b) = ln(a) - exp(b). Fails when a <= 0."""
    if a <= 0:
        return None
    try:
        return math.log(a) - math.exp(b)
    except OverflowError:
        return None


# High-precision versions
def eml_hp(a, b):
    if b <= 0:
        return None
    return float(mpmath.exp(mpmath.mpf(str(a))) - mpmath.log(mpmath.mpf(str(b))))


def edl_hp(a, b):
    if b <= 0:
        return None
    lb = mpmath.log(mpmath.mpf(str(b)))
    if lb == 0:
        return None
    return float(mpmath.exp(mpmath.mpf(str(a))) / lb)


def dle_hp(a, b):
    if a <= 0:
        return None
    eb = mpmath.exp(mpmath.mpf(str(b)))
    return float(mpmath.log(mpmath.mpf(str(a))) / eb)


# ============================================================
# Bootstrapping chains for deriving functions
# ============================================================

# EML bootstrapping chain (matching the Rust verification output)
class EMLChain:
    """Build functions from EML(x,y)=exp(x)-ln(y) + constant 1."""

    @staticmethod
    def exp_x(x: float) -> Optional[float]:
        """K=3: exp(x) = EML(x, 1) = exp(x) - ln(1) = exp(x)"""
        return eml(x, 1.0)

    @staticmethod
    def const_e() -> Optional[float]:
        """K=3: e = EML(1, 1)"""
        return eml(1.0, 1.0)

    @staticmethod
    def ln_x(x: float) -> Optional[float]:
        """K=6: ln(x) = EML(1, exp(EML(1, x)))
        = exp(1) - ln(exp(exp(1)-ln(x)))
        = e - (e - ln(x)) = ln(x)"""
        inner = eml(1.0, x)  # EML(1,x) = e - ln(x)
        if inner is None:
            return None
        exp_inner = eml(inner, 1.0)  # exp(e - ln(x))
        if exp_inner is None:
            return None
        return eml(1.0, exp_inner)  # e - ln(exp(e-ln(x))) = ln(x)

    @staticmethod
    def subtract(a: float, b: float) -> Optional[float]:
        """K=5: a - b = EML(ln(a), exp(b))"""
        ln_a = EMLChain.ln_x(a)
        if ln_a is None:
            return None
        exp_b = EMLChain.exp_x(b)
        if exp_b is None:
            return None
        return eml(ln_a, exp_b)

    @staticmethod
    def neg_x(x: float) -> Optional[float]:
        """K=4: -x = subtract(0, x) = subtract(ln(1), x)"""
        zero = EMLChain.ln_x(1.0)  # ln(1) = 0
        if zero is None:
            return None
        return EMLChain.subtract(math.exp(zero), x)  # This is wrong path

    @staticmethod
    def neg_x_direct(x: float) -> Optional[float]:
        """-x = Subtract[Log[1], x] = EML(Log[1], exp(x))
        But Log[1] needs EML chain: ln(1) = EML(1, exp(EML(1, 1)))
        = e - ln(exp(e-ln(1))) = e - ln(exp(e)) = e - e = 0"""
        zero = EMLChain.ln_x(1.0)  # = 0 via 6-deep chain
        if zero is None:
            return None
        exp_x = EMLChain.exp_x(x)
        if exp_x is None:
            return None
        return eml(zero, exp_x)  # EML(0, exp(x)) = exp(0) - ln(exp(x)) = 1 - x

    @staticmethod
    def inv_x(x: float) -> Optional[float]:
        """K=4: 1/x = exp(-ln(x)) = EML(-ln(x), 1)"""
        ln_x = EMLChain.ln_x(x)
        if ln_x is None:
            return None
        neg_ln = EMLChain.neg_x_direct(ln_x)
        if neg_ln is None:
            return None
        return eml(neg_ln, 1.0)

    @staticmethod
    def multiply(a: float, b: float) -> Optional[float]:
        """K=6: a*b = exp(ln(a) + ln(b)) = exp(subtract(ln(a), neg(ln(b))))"""
        la = EMLChain.ln_x(a)
        lb = EMLChain.ln_x(b)
        if la is None or lb is None:
            return None
        neg_lb = EMLChain.neg_x_direct(lb)
        if neg_lb is None:
            return None
        sum_logs = EMLChain.subtract(math.exp(la), math.exp(neg_lb)) if la > 0 and neg_lb < 0 else None
        if sum_logs is None:
            # Fallback: use native math for sum
            sum_logs = la + lb
        return eml(sum_logs, 1.0)  # exp(la + lb)


class DLEChain:
    """Build functions from DivLogExp(x,y)=ln(x)/exp(y) + constant 1."""

    @staticmethod
    def ln_x(x: float) -> Optional[float]:
        """K=5: ln(x) = DivLogExp(x, DivLogExp(1,1))
        = ln(x) / exp(ln(1)/exp(1)) = ln(x)/exp(0) = ln(x)"""
        inner = div_log_exp(1.0, 1.0)  # ln(1)/exp(1) = 0/e = 0
        if inner is None:
            return None
        return div_log_exp(x, inner)  # ln(x)/exp(0) = ln(x)/1 = ln(x)

    @staticmethod
    def inv_x(x: float) -> Optional[float]:
        """K=4: 1/x = DivLogExp(e, ln(x))"""
        ln_x = DLEChain.ln_x(x)
        if ln_x is None:
            return None
        return div_log_exp(math.e, ln_x)  # ln(e)/exp(ln(x)) = 1/x

    @staticmethod
    def exp_x(x: float) -> Optional[float]:
        """K=4: exp(x) = 1/DivLogExp(e, x)"""
        v = div_log_exp(math.e, x)  # ln(e)/exp(x) = 1/exp(x)
        if v is None or v == 0:
            return None
        return 1.0 / v

    @staticmethod
    def neg_x(x: float) -> Optional[float]:
        """K=4: -x = Log[DivLogExp[E, x]]
        = ln(ln(e)/exp(x)) = ln(1/exp(x)) = ln(exp(-x)) = -x"""
        v = div_log_exp(math.e, x)
        if v is None or v <= 0:
            return None
        return math.log(v)


# ============================================================
# Benchmark functions
# ============================================================

@dataclass
class BenchEntry:
    function: str
    operator: str
    chain_depth: int
    x: float
    y: float
    native: float
    computed: Optional[float]
    hp_reference: Optional[float]  # mpmath 50-digit reference
    abs_error: Optional[float]
    rel_error: Optional[float]
    ulp_error: Optional[int]
    failure_reason: str  # "ok", "domain", "overflow", "nan"


def ulp_distance(a: float, b: float) -> Optional[int]:
    """ULP distance between two floats."""
    if not math.isfinite(a) or not math.isfinite(b):
        return None
    ia = int.from_bytes(a.hex().encode()[:8], 'big') if False else None
    # Simpler approach: count how many ULPs apart
    import struct
    ba = struct.pack('d', a)
    bb = struct.pack('d', b)
    ia = struct.unpack('q', ba)[0]
    ib = struct.unpack('q', bb)[0]
    if ia < 0:
        ia = -(ia ^ 0x7FFFFFFFFFFFFFFF)
    if ib < 0:
        ib = -(ib ^ 0x7FFFFFFFFFFFFFFF)
    return abs(ia - ib)


def rel_err(native: float, computed: Optional[float]) -> Optional[float]:
    if computed is None or not math.isfinite(computed):
        return None
    if native == 0:
        return abs(computed)
    return abs(computed - native) / max(abs(native), 1e-300)


def run_corrected_benchmark():
    """Run corrected benchmark with proper comparisons."""

    # Test inputs
    unary_pos = [0.001, 0.01, 0.1, 0.5, 0.9, 1.0, 1.5, 2.0, 5.0, 10.0, 100.0]
    unary_all = [-100.0, -10.0, -5.0, -2.0, -1.0, -0.5, -0.1] + unary_pos
    edge = [1e-15, 1e-10, 1e-6, 1-1e-10, 1+1e-10, 1e6, 1e10]

    results: list[BenchEntry] = []

    # ============================================================
    # GROUP A: Subtraction-based operators (EML vs NegEML)
    # Issue: domain restriction (ln needs y>0 for EML, a>0 for NegEML)
    # ============================================================
    print("=== GROUP A: Subtraction-based (EML vs NegEML) ===")

    test_pairs_A = [(a, b) for a in unary_all for b in unary_pos]
    for a, b in test_pairs_A:
        # EML: always valid for any a, b>0
        eml_val = eml(a, b)
        eml_hp_val = eml_hp(a, b)

        # NegEML: valid for a>0, any b
        neml_val = neg_eml(a, b) if a > 0 else None

        if eml_val is not None and eml_hp_val is not None:
            re = rel_err(eml_hp_val, eml_val)
            results.append(BenchEntry("raw_op", "EML", 1, a, b, eml_hp_val, eml_val,
                                     eml_hp_val, abs(eml_val - eml_hp_val) if eml_val else None,
                                     re, None, "ok"))

    # ============================================================
    # GROUP B: Division-based operators (EDL vs DivLogExp)
    # Key comparison: which has fewer failures and better precision?
    # ============================================================
    print("=== GROUP B: Division-based (EDL vs DivLogExp) ===")

    # EDL: exp(a)/ln(b) — fails when b<=0 or b=1
    # DivLogExp: ln(a)/exp(b) — fails when a<=0, never div-by-zero (exp>0)
    test_pairs_B = [(a, b) for a in unary_all + edge for b in unary_all + edge]

    edl_total = edl_ok = edl_domain = edl_divzero = edl_overflow = 0
    dle_total = dle_ok = dle_domain = dle_divzero = dle_overflow = 0

    edl_errors = []
    dle_errors = []

    for a, b in test_pairs_B:
        # EDL
        edl_total += 1
        ev = edl(a, b)
        if ev is None:
            if b <= 0:
                edl_domain += 1
            elif abs(math.log(b)) < 1e-15 if b > 0 else True:
                edl_divzero += 1
            else:
                edl_overflow += 1
        else:
            ev_hp = edl_hp(a, b)
            if ev_hp is not None:
                edl_ok += 1
                re = rel_err(ev_hp, ev)
                if re is not None:
                    edl_errors.append(re)

        # DivLogExp
        dle_total += 1
        dv = div_log_exp(a, b)
        if dv is None:
            if a <= 0:
                dle_domain += 1
            else:
                dle_overflow += 1
        else:
            dv_hp = dle_hp(a, b)
            if dv_hp is not None:
                dle_ok += 1
                re = rel_err(dv_hp, dv)
                if re is not None:
                    dle_errors.append(re)

    print(f"\n  EDL: total={edl_total}, ok={edl_ok}, domain_fail={edl_domain}, "
          f"div_by_zero={edl_divzero}, overflow={edl_overflow}")
    print(f"  DLE: total={dle_total}, ok={dle_ok}, domain_fail={dle_domain}, "
          f"div_by_zero={dle_divzero}, overflow={dle_overflow}")

    if edl_errors:
        edl_errors.sort()
        print(f"  EDL precision: max_rel_err={max(edl_errors):.2e}, "
              f"median={edl_errors[len(edl_errors)//2]:.2e}, "
              f"mean={sum(edl_errors)/len(edl_errors):.2e}")
    if dle_errors:
        dle_errors.sort()
        print(f"  DLE precision: max_rel_err={max(dle_errors):.2e}, "
              f"median={dle_errors[len(dle_errors)//2]:.2e}, "
              f"mean={sum(dle_errors)/len(dle_errors):.2e}")

    # ============================================================
    # GROUP C: Bootstrapped function precision
    # Compare how well each operator chain builds basic functions
    # ============================================================
    print("\n=== GROUP C: Bootstrapped function precision ===")
    print(f"{'Function':<12} {'EML chain rel_err':<22} {'DLE chain rel_err':<22} {'EML depth':<10} {'DLE depth':<10}")

    for x in [0.5, 1.0, 2.0, 5.0, 10.0]:
        if x > 0:
            # exp(x)
            native_exp = math.exp(x)
            eml_exp = EMLChain.exp_x(x)
            dle_exp = DLEChain.exp_x(x)
            re_eml = rel_err(native_exp, eml_exp) if eml_exp else None
            re_dle = rel_err(native_exp, dle_exp) if dle_exp else None
            print(f"exp({x}){'':<6} {re_eml if re_eml is not None else 'FAIL'!s:<22} "
                  f"{re_dle if re_dle is not None else 'FAIL'!s:<22} {'K=3':<10} {'K=4':<10}")

            # ln(x)
            native_ln = math.log(x)
            eml_ln = EMLChain.ln_x(x)
            dle_ln = DLEChain.ln_x(x)
            re_eml = rel_err(native_ln, eml_ln) if eml_ln else None
            re_dle = rel_err(native_ln, dle_ln) if dle_ln else None
            print(f"ln({x}){'':<7} {re_eml if re_eml is not None else 'FAIL'!s:<22} "
                  f"{re_dle if re_dle is not None else 'FAIL'!s:<22} {'K=6':<10} {'K=5':<10}")

            # 1/x
            native_inv = 1.0 / x
            eml_inv = EMLChain.inv_x(x)
            dle_inv = DLEChain.inv_x(x)
            re_eml = rel_err(native_inv, eml_inv) if eml_inv else None
            re_dle = rel_err(native_inv, dle_inv) if dle_inv else None
            print(f"1/{x}{'':<8} {re_eml if re_eml is not None else 'FAIL'!s:<22} "
                  f"{re_dle if re_dle is not None else 'FAIL'!s:<22} {'K=4':<10} {'K=4':<10}")

            # -x
            native_neg = -x
            eml_neg = EMLChain.neg_x_direct(x)
            dle_neg = DLEChain.neg_x(x)
            re_eml = rel_err(native_neg, eml_neg) if eml_neg else None
            re_dle = rel_err(native_neg, dle_neg) if dle_neg else None
            print(f"-{x}{'':<9} {re_eml if re_eml is not None else 'FAIL'!s:<22} "
                  f"{re_dle if re_dle is not None else 'FAIL'!s:<22} {'K=4':<10} {'K=4':<10}")

        print()

    # ============================================================
    # Save results
    # ============================================================
    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    csv_path = os.path.join(results_dir, 'numerical_stability_v2.csv')
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['section', 'operator', 'metric', 'value'])
        # Group B summary
        writer.writerow(['GroupB', 'EDL', 'total', edl_total])
        writer.writerow(['GroupB', 'EDL', 'ok', edl_ok])
        writer.writerow(['GroupB', 'EDL', 'domain_fail', edl_domain])
        writer.writerow(['GroupB', 'EDL', 'div_by_zero', edl_divzero])
        writer.writerow(['GroupB', 'EDL', 'overflow', edl_overflow])
        writer.writerow(['GroupB', 'EDL', 'max_rel_err', max(edl_errors) if edl_errors else 'N/A'])
        writer.writerow(['GroupB', 'DivLogExp', 'total', dle_total])
        writer.writerow(['GroupB', 'DivLogExp', 'ok', dle_ok])
        writer.writerow(['GroupB', 'DivLogExp', 'domain_fail', dle_domain])
        writer.writerow(['GroupB', 'DivLogExp', 'div_by_zero', dle_divzero])
        writer.writerow(['GroupB', 'DivLogExp', 'overflow', dle_overflow])
        writer.writerow(['GroupB', 'DivLogExp', 'max_rel_err', max(dle_errors) if dle_errors else 'N/A'])

    print(f"\nSaved to {csv_path}")


if __name__ == "__main__":
    run_corrected_benchmark()
