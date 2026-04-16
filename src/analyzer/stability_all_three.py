"""
Corrected stability benchmark: EML vs EDL vs DivLogExp
All three operators measured on the same input grid.
DEVIL requirement: EML must be included in stability measurement.
"""
import math
import csv
import os
from typing import Optional

import mpmath
mpmath.mp.dps = 50


def eml(a: float, b: float) -> Optional[float]:
    """EML(a,b) = exp(a) - ln(b). Domain: any a, b > 0."""
    if b <= 0:
        return None
    try:
        return math.exp(a) - math.log(b)
    except OverflowError:
        return None


def edl(a: float, b: float) -> Optional[float]:
    """EDL(a,b) = exp(a) / ln(b). Domain: any a, b > 0 and b != 1."""
    if b <= 0:
        return None
    lb = math.log(b)
    if lb == 0.0:
        return None
    try:
        return math.exp(a) / lb
    except OverflowError:
        return None


def dle(a: float, b: float) -> Optional[float]:
    """DivLogExp(a,b) = ln(a) / exp(b). Domain: a > 0, any b (but exp(b) overflows for large b)."""
    if a <= 0:
        return None
    try:
        eb = math.exp(b)
        if not math.isfinite(eb):
            return None
        return math.log(a) / eb
    except OverflowError:
        return None


# High-precision references
def eml_hp(a, b):
    if b <= 0: return None
    try:
        return float(mpmath.exp(mpmath.mpf(str(a))) - mpmath.log(mpmath.mpf(str(b))))
    except: return None

def edl_hp(a, b):
    if b <= 0: return None
    lb = mpmath.log(mpmath.mpf(str(b)))
    if lb == 0: return None
    try:
        return float(mpmath.exp(mpmath.mpf(str(a))) / lb)
    except: return None

def dle_hp(a, b):
    if a <= 0: return None
    try:
        return float(mpmath.log(mpmath.mpf(str(a))) / mpmath.exp(mpmath.mpf(str(b))))
    except: return None


def rel_err(ref: float, val: float) -> Optional[float]:
    if not math.isfinite(ref) or not math.isfinite(val):
        return None
    if ref == 0:
        return abs(val)
    return abs(val - ref) / abs(ref)


def main():
    # Input grid: cover positive, negative, near-zero, near-1, large values
    vals = [-100, -10, -5, -2, -1, -0.5, -0.1, -0.01,
            0.01, 0.1, 0.5, 0.9, 0.99, 1.0, 1.01, 1.5, 2.0, 5.0, 10.0, 100.0]
    edge = [1e-15, 1e-10, 1e-6, 1-1e-10, 1+1e-10, 1e6, 1e10]
    all_vals = vals + edge

    pairs = [(a, b) for a in all_vals for b in all_vals]

    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    os.makedirs(results_dir, exist_ok=True)

    ops = [
        ("EML", eml, eml_hp),
        ("EDL", edl, edl_hp),
        ("DivLogExp", dle, dle_hp),
    ]

    summary = {}
    for op_name, op_fn, op_hp_fn in ops:
        total = 0
        ok = 0
        domain_fail = 0
        div_by_zero = 0
        overflow = 0
        nan_count = 0
        errors = []

        for a, b in pairs:
            total += 1
            val = op_fn(a, b)
            if val is None:
                # Classify failure
                if op_name == "EML" and b <= 0:
                    domain_fail += 1  # ln(b) undefined
                elif op_name == "EDL":
                    if b <= 0:
                        domain_fail += 1
                    elif b > 0 and abs(math.log(b)) < 1e-300:
                        div_by_zero += 1
                    else:
                        overflow += 1
                elif op_name == "DivLogExp":
                    if a <= 0:
                        domain_fail += 1
                    else:
                        overflow += 1  # exp(b) overflowed
                continue
            if math.isnan(val):
                nan_count += 1
                continue
            if math.isinf(val):
                overflow += 1
                continue

            hp_val = op_hp_fn(a, b)
            if hp_val is not None and math.isfinite(hp_val):
                ok += 1
                re = rel_err(hp_val, val)
                if re is not None:
                    errors.append(re)

        errors.sort()
        summary[op_name] = {
            "total": total,
            "ok": ok,
            "domain_fail": domain_fail,
            "div_by_zero": div_by_zero,
            "overflow": overflow,
            "nan": nan_count,
            "failed_total": domain_fail + div_by_zero + overflow + nan_count,
            "max_rel_err": max(errors) if errors else None,
            "median_rel_err": errors[len(errors)//2] if errors else None,
            "mean_rel_err": sum(errors)/len(errors) if errors else None,
            "p99_rel_err": errors[int(len(errors)*0.99)] if errors else None,
        }

    # Print results
    eps = 2.220446049250313e-16
    print(f"Input grid: {len(all_vals)} values, {len(pairs)} pairs")
    print(f"Machine epsilon: {eps:.2e}")
    print()
    print(f"{'Metric':<25} {'EML':<18} {'EDL':<18} {'DivLogExp':<18}")
    print("-" * 79)
    for metric in ["total", "ok", "domain_fail", "div_by_zero", "overflow", "nan", "failed_total"]:
        row = f"{metric:<25}"
        for op in ["EML", "EDL", "DivLogExp"]:
            row += f" {summary[op][metric]!s:<17}"
        print(row)

    print()
    print(f"{'Precision metric':<25} {'EML':<18} {'EDL':<18} {'DivLogExp':<18}")
    print("-" * 79)
    for metric in ["max_rel_err", "median_rel_err", "mean_rel_err", "p99_rel_err"]:
        row = f"{metric:<25}"
        for op in ["EML", "EDL", "DivLogExp"]:
            v = summary[op][metric]
            if v is not None:
                # Show digits lost relative to machine epsilon
                digits_lost = math.log10(v / eps) if v > 0 else 0
                row += f" {v:<10.2e} ({digits_lost:+.1f}d)"
            else:
                row += f" {'N/A':<18}"
        print(row)

    print()
    print("Digits lost = log10(rel_err / machine_epsilon). 0 = perfect, +8 = 8 digits lost.")

    # Failure breakdown
    print()
    print("=== Failure Analysis ===")
    for op in ["EML", "EDL", "DivLogExp"]:
        s = summary[op]
        pct_fail = 100 * s["failed_total"] / s["total"]
        print(f"\n{op}: {s['failed_total']}/{s['total']} failures ({pct_fail:.1f}%)")
        print(f"  Domain (arg out of range): {s['domain_fail']}")
        print(f"  Division by zero:          {s['div_by_zero']}")
        print(f"  Overflow:                  {s['overflow']}")
        print(f"  NaN:                       {s['nan']}")

        if op == "EML":
            print(f"  Note: EML fails when b<=0 (ln(b) undefined). No division involved.")
        elif op == "EDL":
            print(f"  Note: EDL fails when b<=0 (ln undefined) OR b=1 (ln(b)=0, div by zero).")
        elif op == "DivLogExp":
            print(f"  Note: DivLogExp fails when a<=0 (ln undefined) OR exp(b) overflows.")
            print(f"  exp(b)>0 for all finite b, so division by zero is impossible by definition.")

    # Save CSV
    csv_path = os.path.join(results_dir, 'stability_all_three.csv')
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['operator', 'metric', 'value'])
        for op in ["EML", "EDL", "DivLogExp"]:
            for k, v in summary[op].items():
                writer.writerow([op, k, v])
    print(f"\nSaved to {csv_path}")


if __name__ == "__main__":
    main()
