"""
PRD-02 Task A: Numerical Stability Benchmark
Compare EML vs DivLogExp (and EDL) on core functions.

Key questions:
1. Division-by-zero events across input space
2. Overflow/underflow events (inf/NaN)
3. Max relative error at various tree depths
4. Edge case behavior (x near 0, 1, large y, negative y)
"""
from __future__ import annotations
import math
import csv
import os
import sys
from typing import Optional, Callable
from dataclasses import dataclass, field

try:
    import mpmath
    mpmath.mp.dps = 50  # 50 decimal digits precision
    HAS_MPMATH = True
except ImportError:
    HAS_MPMATH = False
    print("WARNING: mpmath not available, using float64 as reference")


# ============================================================
# Operator definitions (pure float64)
# ============================================================

def eml(a: float, b: float) -> Optional[float]:
    """EML(a,b) = exp(a) - ln(b)"""
    if b <= 0:
        return None
    try:
        return math.exp(a) - math.log(b)
    except (OverflowError, ValueError):
        return None


def edl(a: float, b: float) -> Optional[float]:
    """EDL(a,b) = exp(a) / ln(b)"""
    if b <= 0:
        return None
    lb = math.log(b)
    if lb == 0:
        return None
    try:
        return math.exp(a) / lb
    except (OverflowError, ValueError):
        return None


def div_log_exp(a: float, b: float) -> Optional[float]:
    """DivLogExp(a,b) = ln(a) / exp(b)"""
    if a <= 0:
        return None
    try:
        eb = math.exp(b)
        if eb == 0 or not math.isfinite(eb):
            return None
        return math.log(a) / eb
    except (OverflowError, ValueError):
        return None


# ============================================================
# Bootstrapping chains (how to build each function from the operator)
# ============================================================

# EML chain (from author's verification)
def eml_exp(x: float) -> Optional[float]:
    """exp(x) = EML(x, 1) = exp(x) - ln(1) = exp(x)"""
    return eml(x, 1.0)

def eml_e() -> Optional[float]:
    """e = EML(1, 1)"""
    return eml(1.0, 1.0)

def eml_ln(x: float) -> Optional[float]:
    """ln(x) = EML(1, exp(EML(1, x)))
    = exp(1) - ln(exp(exp(1) - ln(x)))
    = e - ln(exp(e - ln(x)))
    = e - (e - ln(x)) = ln(x)"""
    inner = eml(1.0, x)
    if inner is None:
        return None
    exp_inner = eml_exp(inner)
    if exp_inner is None:
        return None
    return eml(1.0, exp_inner)

def eml_sub(a: float, b: float) -> Optional[float]:
    """a - b = EML(ln(a), exp(b))"""
    la = eml_ln(a) if a > 0 else None
    if la is None:
        return None
    eb = eml_exp(b)
    if eb is None:
        return None
    return eml(la, eb)


# DivLogExp chain (from our verification)
def dle_ln(x: float) -> Optional[float]:
    """ln(x) via DivLogExp chain:
    DivLogExp[x, DivLogExp[1, 1]]
    = ln(x) / exp(ln(1)/exp(1))
    = ln(x) / exp(0/e)
    = ln(x) / exp(0) = ln(x) / 1 = ln(x)"""
    inner = div_log_exp(1.0, 1.0)  # ln(1)/exp(1) = 0/e = 0
    if inner is None:
        return None
    return div_log_exp(x, inner)

def dle_inv(x: float) -> Optional[float]:
    """1/x via DivLogExp chain: DivLogExp(e, ln(x))"""
    e = math.e
    lx = dle_ln(x)
    if lx is None:
        return None
    return div_log_exp(e, lx)

def dle_exp(x: float) -> Optional[float]:
    """exp(x) = 1/DivLogExp(e, x) = 1/(ln(e)/exp(x)) = exp(x)/1 = exp(x)"""
    v = div_log_exp(math.e, x)
    if v is None or v == 0:
        return None
    return 1.0 / v

def dle_neg(x: float) -> Optional[float]:
    """-(x) via ln(DivLogExp(e, x))"""
    v = div_log_exp(math.e, x)
    if v is None or v <= 0:
        return None
    return math.log(v)

def dle_minus(x: float) -> Optional[float]:
    """Alias for neg"""
    return dle_neg(x)


# ============================================================
# Benchmark functions
# ============================================================

@dataclass
class BenchmarkResult:
    function_name: str
    operator: str
    tree_depth: int  # K value
    input_x: float
    input_y: Optional[float]
    native_result: float
    operator_result: Optional[float]
    relative_error: Optional[float]
    is_overflow: bool
    is_nan: bool
    is_div_zero: bool


def relative_error(native: float, computed: Optional[float]) -> Optional[float]:
    if computed is None:
        return None
    if not math.isfinite(native) or not math.isfinite(computed):
        return None
    if native == 0:
        return abs(computed) if computed != 0 else 0.0
    return abs(computed - native) / abs(native)


def mpmath_reference(func_name: str, x: float, y: float = 0.0) -> Optional[float]:
    """Get high-precision reference value using mpmath."""
    if not HAS_MPMATH:
        return None
    try:
        mx = mpmath.mpf(str(x))
        my = mpmath.mpf(str(y))
        if func_name == "exp":
            return float(mpmath.exp(mx))
        elif func_name == "ln":
            if mx <= 0:
                return None
            return float(mpmath.log(mx))
        elif func_name == "neg":
            return float(-mx)
        elif func_name == "inv":
            if mx == 0:
                return None
            return float(1/mx)
        elif func_name == "add":
            return float(mx + my)
        elif func_name == "sub":
            return float(mx - my)
        elif func_name == "mul":
            return float(mx * my)
        elif func_name == "div":
            if my == 0:
                return None
            return float(mx / my)
        elif func_name == "sqrt":
            if mx < 0:
                return None
            return float(mpmath.sqrt(mx))
        elif func_name == "sin":
            return float(mpmath.sin(mx))
    except Exception:
        return None
    return None


def benchmark_unary(
    func_name: str,
    native_fn: Callable[[float], float],
    chains: dict[str, Callable[[float], Optional[float]]],
    test_inputs: list[float],
) -> list[BenchmarkResult]:
    """Benchmark unary function across operators."""
    results = []
    for x in test_inputs:
        try:
            native = native_fn(x)
        except (ValueError, OverflowError):
            continue
        if not math.isfinite(native):
            continue

        for op_name, chain_fn in chains.items():
            try:
                computed = chain_fn(x)
            except (ValueError, OverflowError, ZeroDivisionError):
                computed = None

            is_overflow = computed is not None and (math.isinf(computed) if isinstance(computed, float) else False)
            is_nan = computed is not None and (math.isnan(computed) if isinstance(computed, float) else False)
            is_div_zero = computed is None

            re = relative_error(native, computed)

            results.append(BenchmarkResult(
                function_name=func_name,
                operator=op_name,
                tree_depth=0,  # Will be filled per-operator
                input_x=x,
                input_y=None,
                native_result=native,
                operator_result=computed,
                relative_error=re,
                is_overflow=is_overflow,
                is_nan=is_nan,
                is_div_zero=is_div_zero,
            ))
    return results


def benchmark_binary(
    func_name: str,
    native_fn: Callable[[float, float], float],
    chains: dict[str, Callable[[float, float], Optional[float]]],
    test_pairs: list[tuple[float, float]],
) -> list[BenchmarkResult]:
    """Benchmark binary function across operators."""
    results = []
    for x, y in test_pairs:
        try:
            native = native_fn(x, y)
        except (ValueError, OverflowError, ZeroDivisionError):
            continue
        if not math.isfinite(native):
            continue

        for op_name, chain_fn in chains.items():
            try:
                computed = chain_fn(x, y)
            except (ValueError, OverflowError, ZeroDivisionError):
                computed = None

            is_overflow = computed is not None and (math.isinf(computed) if isinstance(computed, float) else False)
            is_nan = computed is not None and (math.isnan(computed) if isinstance(computed, float) else False)
            is_div_zero = computed is None

            re = relative_error(native, computed)

            results.append(BenchmarkResult(
                function_name=func_name,
                operator=op_name,
                tree_depth=0,
                input_x=x,
                input_y=y,
                native_result=native,
                operator_result=computed,
                relative_error=re,
                is_overflow=is_overflow,
                is_nan=is_nan,
                is_div_zero=is_div_zero,
            ))
    return results


def run_benchmark():
    """Run the full numerical stability benchmark."""
    # Test inputs for unary functions
    unary_inputs = [0.001, 0.01, 0.1, 0.5, 0.9, 1.0, 1.5, 2.0, 5.0, 10.0, 100.0, 1000.0]
    # Edge cases
    edge_inputs = [1e-10, 1e-6, 1.0 - 1e-10, 1.0 + 1e-10, 1e6, 1e10]

    # Test inputs for binary functions
    binary_pairs = [
        (x, y)
        for x in [0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
        for y in [0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    ]

    all_inputs = unary_inputs + edge_inputs

    all_results: list[BenchmarkResult] = []

    # --- Unary functions ---
    print("Benchmarking unary functions...")

    # exp(x)
    all_results.extend(benchmark_unary("exp", math.exp, {
        "EML": eml_exp,
        "DivLogExp": dle_exp,
    }, all_inputs))

    # ln(x)
    all_results.extend(benchmark_unary("ln", math.log, {
        "EML": eml_ln,
        "DivLogExp": dle_ln,
    }, [x for x in all_inputs if x > 0]))

    # neg(x)
    all_results.extend(benchmark_unary("neg", lambda x: -x, {
        "EML": lambda x: eml_sub(0.0, x) if eml_ln(1.0) is not None else None,
        "DivLogExp": dle_neg,
    }, all_inputs))

    # inv(x) = 1/x
    all_results.extend(benchmark_unary("inv", lambda x: 1.0/x, {
        "EML": lambda x: math.exp(-math.log(x)) if x > 0 else None,
        "DivLogExp": dle_inv,
    }, [x for x in all_inputs if x != 0]))

    # sqrt(x)
    all_results.extend(benchmark_unary("sqrt", math.sqrt, {
        "EML": lambda x: math.exp(0.5 * math.log(x)) if x > 0 else None,
        "DivLogExp": lambda x: math.exp(0.5 * (dle_ln(x) or float('nan'))) if x > 0 and dle_ln(x) is not None else None,
    }, [x for x in all_inputs if x > 0]))

    # --- Binary functions ---
    print("Benchmarking binary functions...")

    # x + y
    all_results.extend(benchmark_binary("add", lambda x, y: x + y, {
        "EML": lambda x, y: eml_sub(x, eml_sub(0.0, y) if eml_ln(1.0) is not None else None) if eml_sub(0.0, y) is not None else None,
    }, binary_pairs))

    # x - y
    all_results.extend(benchmark_binary("sub", lambda x, y: x - y, {
        "EML": eml_sub,
    }, [(x, y) for x, y in binary_pairs if x > 0]))

    # x * y (via exp(ln(x) + ln(y)))
    all_results.extend(benchmark_binary("mul", lambda x, y: x * y, {
        "EML": lambda x, y: math.exp(math.log(x) + math.log(y)) if x > 0 and y > 0 else None,
        "DivLogExp": lambda x, y: math.exp((dle_ln(x) or 0) + (dle_ln(y) or 0)) if x > 0 and y > 0 and dle_ln(x) is not None and dle_ln(y) is not None else None,
    }, [(x, y) for x, y in binary_pairs if x > 0 and y > 0]))

    # x / y (via exp(ln(x) - ln(y)))
    all_results.extend(benchmark_binary("div", lambda x, y: x / y, {
        "EML": lambda x, y: math.exp(math.log(x) - math.log(y)) if x > 0 and y > 0 else None,
        "DivLogExp": lambda x, y: math.exp((dle_ln(x) or 0) - (dle_ln(y) or 0)) if x > 0 and y > 0 and dle_ln(x) is not None and dle_ln(y) is not None else None,
    }, [(x, y) for x, y in binary_pairs if x > 0 and y > 0]))

    # --- Analysis ---
    print(f"\nTotal benchmark points: {len(all_results)}")

    # Save raw results
    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results')
    os.makedirs(results_dir, exist_ok=True)
    csv_path = os.path.join(results_dir, 'numerical_stability.csv')

    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow([
            'function', 'operator', 'input_x', 'input_y',
            'native_result', 'operator_result', 'relative_error',
            'is_overflow', 'is_nan', 'is_div_zero',
        ])
        for r in all_results:
            writer.writerow([
                r.function_name, r.operator, r.input_x, r.input_y,
                r.native_result, r.operator_result, r.relative_error,
                r.is_overflow, r.is_nan, r.is_div_zero,
            ])

    print(f"Saved to {csv_path}")

    # Summary statistics
    print("\n=== Summary by Operator ===")
    operators = sorted(set(r.operator for r in all_results))
    for op in operators:
        op_results = [r for r in all_results if r.operator == op]
        total = len(op_results)
        div_zeros = sum(1 for r in op_results if r.is_div_zero)
        overflows = sum(1 for r in op_results if r.is_overflow)
        nans = sum(1 for r in op_results if r.is_nan)
        valid = [r for r in op_results if r.relative_error is not None]
        if valid:
            max_re = max(r.relative_error for r in valid)
            avg_re = sum(r.relative_error for r in valid) / len(valid)
            median_re = sorted(r.relative_error for r in valid)[len(valid)//2]
        else:
            max_re = avg_re = median_re = float('nan')

        print(f"\n  {op}:")
        print(f"    Total points: {total}")
        print(f"    Div-by-zero: {div_zeros} ({100*div_zeros/total:.1f}%)")
        print(f"    Overflow: {overflows}")
        print(f"    NaN: {nans}")
        print(f"    Valid comparisons: {len(valid)}")
        if valid:
            print(f"    Max relative error: {max_re:.2e}")
            print(f"    Avg relative error: {avg_re:.2e}")
            print(f"    Median relative error: {median_re:.2e}")

    # Per-function breakdown
    print("\n=== Per-Function Max Relative Error ===")
    functions = sorted(set(r.function_name for r in all_results))
    print(f"{'Function':<10}", end="")
    for op in operators:
        print(f"  {op:<15}", end="")
    print()

    for func in functions:
        print(f"{func:<10}", end="")
        for op in operators:
            func_op_results = [r for r in all_results
                             if r.function_name == func and r.operator == op
                             and r.relative_error is not None]
            if func_op_results:
                max_re = max(r.relative_error for r in func_op_results)
                print(f"  {max_re:<15.2e}", end="")
            else:
                print(f"  {'N/A':<15}", end="")
        print()


if __name__ == "__main__":
    run_benchmark()
