"""
4-operator stability benchmark: EML, EDL, DivLogExp, PowerLogInv
Same 729-point grid for all four. mpmath 50-digit reference.
"""
import math, csv, os, cmath
from typing import Optional
import mpmath
mpmath.mp.dps = 50


def eml(a: float, b: float) -> Optional[float]:
    if b <= 0: return None
    try: return math.exp(a) - math.log(b)
    except OverflowError: return None

def edl(a: float, b: float) -> Optional[float]:
    if b <= 0: return None
    lb = math.log(b)
    if lb == 0.0: return None
    try: return math.exp(a) / lb
    except OverflowError: return None

def dle(a: float, b: float) -> Optional[float]:
    if a <= 0: return None
    try:
        eb = math.exp(b)
        if not math.isfinite(eb): return None
        return math.log(a) / eb
    except OverflowError: return None

def pli(a: float, b: float) -> Optional[float]:
    """PLI(a,b) = ln(a)^(1/b). Domain: a>0, b!=0.
    ln(a) can be negative (when 0<a<1). Negative base ^ non-integer exponent -> complex.
    Count these as domain failures."""
    if a <= 0 or b == 0: return None
    la = math.log(a)
    inv_b = 1.0 / b
    if la < 0:
        # Negative base. Check if 1/b is an integer.
        if inv_b != int(inv_b):
            return None  # Would produce complex -> domain failure
        # Integer exponent of negative base is real
        try:
            return la ** int(inv_b)
        except (OverflowError, ValueError):
            return None
    if la == 0:
        return 0.0 if inv_b > 0 else None
    try:
        result = la ** inv_b
        if not math.isfinite(result): return None
        return result
    except (OverflowError, ValueError):
        return None


# High-precision versions
def eml_hp(a, b):
    if b <= 0: return None
    try: return float(mpmath.exp(mpmath.mpf(str(a))) - mpmath.log(mpmath.mpf(str(b))))
    except: return None

def edl_hp(a, b):
    if b <= 0: return None
    lb = mpmath.log(mpmath.mpf(str(b)))
    if lb == 0: return None
    try: return float(mpmath.exp(mpmath.mpf(str(a))) / lb)
    except: return None

def dle_hp(a, b):
    if a <= 0: return None
    try: return float(mpmath.log(mpmath.mpf(str(a))) / mpmath.exp(mpmath.mpf(str(b))))
    except: return None

def pli_hp(a, b):
    if a <= 0 or b == 0: return None
    la = mpmath.log(mpmath.mpf(str(a)))
    inv_b = mpmath.mpf(1) / mpmath.mpf(str(b))
    if la < 0:
        if float(inv_b) != int(float(inv_b)):
            return None
        try: return float(la ** int(float(inv_b)))
        except: return None
    if la == 0:
        return 0.0 if float(inv_b) > 0 else None
    try:
        result = float(la ** inv_b)
        if not math.isfinite(result): return None
        return result
    except: return None


def rel_err(ref: float, val: float) -> Optional[float]:
    if not math.isfinite(ref) or not math.isfinite(val): return None
    if ref == 0: return abs(val)
    return abs(val - ref) / abs(ref)


def main():
    vals = [-100, -10, -5, -2, -1, -0.5, -0.1, -0.01,
            0.01, 0.1, 0.5, 0.9, 0.99, 1.0, 1.01, 1.5, 2.0, 5.0, 10.0, 100.0]
    edge = [1e-15, 1e-10, 1e-6, 1-1e-10, 1+1e-10, 1e6, 1e10]
    all_vals = vals + edge
    pairs = [(a, b) for a in all_vals for b in all_vals]

    ops = [
        ("EML", eml, eml_hp),
        ("EDL", edl, edl_hp),
        ("DivLogExp", dle, dle_hp),
        ("PowerLogInv", pli, pli_hp),
    ]

    summary = {}
    for op_name, op_fn, op_hp_fn in ops:
        total = ok = domain_fail = div_by_zero = overflow = nan_count = complex_fail = 0
        errors = []

        for a, b in pairs:
            total += 1
            val = op_fn(a, b)
            if val is None:
                if op_name == "EML":
                    domain_fail += 1 if b <= 0 else 0
                    overflow += 1 if b > 0 else 0
                elif op_name == "EDL":
                    if b <= 0: domain_fail += 1
                    elif b > 0 and abs(math.log(b)) < 1e-300: div_by_zero += 1
                    else: overflow += 1
                elif op_name == "DivLogExp":
                    if a <= 0: domain_fail += 1
                    else: overflow += 1
                elif op_name == "PowerLogInv":
                    if a <= 0 or b == 0:
                        domain_fail += 1
                    elif a > 0 and math.log(a) < 0:
                        complex_fail += 1  # negative base ^ non-integer exp
                    else:
                        overflow += 1
                continue
            if math.isnan(val): nan_count += 1; continue
            if math.isinf(val): overflow += 1; continue

            hp_val = op_hp_fn(a, b)
            if hp_val is not None and math.isfinite(hp_val):
                ok += 1
                re = rel_err(hp_val, val)
                if re is not None:
                    errors.append(re)

        errors.sort()
        summary[op_name] = {
            "total": total, "ok": ok,
            "domain_fail": domain_fail, "div_by_zero": div_by_zero,
            "complex_fail": complex_fail, "overflow": overflow, "nan": nan_count,
            "failed_total": total - ok,
            "max_rel_err": max(errors) if errors else None,
            "median_rel_err": errors[len(errors)//2] if errors else None,
            "mean_rel_err": sum(errors)/len(errors) if errors else None,
            "p99_rel_err": errors[int(len(errors)*0.99)] if errors else None,
        }

    eps = 2.220446049250313e-16
    print(f"Input grid: {len(all_vals)} values, {len(pairs)} pairs")
    print(f"Machine epsilon: {eps:.2e}\n")

    header = f"{'Metric':<25}"
    for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
        header += f" {op:<15}"
    print(header)
    print("-" * 90)

    for metric in ["total", "ok", "domain_fail", "div_by_zero", "complex_fail", "overflow", "nan", "failed_total"]:
        row = f"{metric:<25}"
        for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
            row += f" {summary[op][metric]!s:<15}"
        print(row)

    print(f"\n{'Precision':<25}", end="")
    for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
        print(f" {op:<15}", end="")
    print()
    print("-" * 90)

    for metric in ["max_rel_err", "median_rel_err", "mean_rel_err", "p99_rel_err"]:
        row = f"{metric:<25}"
        for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
            v = summary[op][metric]
            if v is not None:
                digits = math.log10(v / eps) if v > 0 else 0
                row += f" {v:<9.2e}({digits:+.1f}d)"
            else:
                row += f" {'N/A':<15}"
        print(row)

    print(f"\nDigits lost = log10(rel_err / eps). 0=perfect, +8=8 digits lost.\n")

    # Failure analysis
    for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
        s = summary[op]
        pct = 100 * s["failed_total"] / s["total"]
        print(f"{op}: {s['failed_total']}/{s['total']} failures ({pct:.1f}%)")
        print(f"  domain={s['domain_fail']} div0={s['div_by_zero']} complex={s['complex_fail']} overflow={s['overflow']} nan={s['nan']}")

    # Save CSV
    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    csv_path = os.path.join(results_dir, 'stability_four_ops.csv')
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['operator', 'metric', 'value'])
        for op in ["EML", "EDL", "DivLogExp", "PowerLogInv"]:
            for k, v in summary[op].items():
                writer.writerow([op, k, v])
    print(f"\nSaved: {csv_path}")

    # Return summary for downstream use
    return summary


if __name__ == "__main__":
    main()
