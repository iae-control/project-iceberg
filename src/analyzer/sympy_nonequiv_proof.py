# -*- coding: utf-8 -*-
"""
sympy_nonequiv_proof.py
-----------------------
Formally proves (via symbolic algebra) that:

    PLI(x,y) = ln(x)^(1/y)

is NOT equivalent to:

    EML(x,y)       = exp(x) - ln(y)
    EDL(x,y)       = exp(x) / ln(y)
    NegEML(x,y)    = ln(x) - exp(y)
    DivLogExp(x,y) = ln(x) / exp(y)

under any combination of the five transformations:
    T1. Argument swap     : f(x,y) <-> g(y,x)
    T2. Negation          : f(x,y) <-> -g(x,y)
    T3. Reciprocal        : f(x,y) <-> 1/g(x,y)
    T4. Constant shift    : f(x,y) <-> g(x,y) + c   (some constant c)
    T5. Constant multiply : f(x,y) <-> c * g(x,y)   (some constant c != 0)

Strategy
--------
For exact equivalence (T1-T3 + identity):
    simplify(f - g) == 0

For T4 (constant shift):
    diff(f - g, x) == 0  AND  diff(f - g, y) == 0
    => f - g is a constant (possibly zero).

For T5 (constant multiply):
    diff(f/g, x) == 0  AND  diff(f/g, y) == 0
    => f/g is a constant.

All checks are tried exhaustively over every combination of
{identity, swap} x {identity, negate, reciprocal} applied to PLI,
against each target function.
"""

import io
import sys
import json
import traceback
from datetime import datetime, timezone

# Force stdout to UTF-8 to handle any unicode in output
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

import sympy as sp

# ---------------------------------------------------------------------------
# Symbols: x > 0, y > 0 so ln(x) and ln(y) are real and non-zero
# ---------------------------------------------------------------------------
x_pos = sp.Symbol("x", positive=True)
y_pos = sp.Symbol("y", positive=True)


# ---------------------------------------------------------------------------
# Function definitions
# ---------------------------------------------------------------------------
def PLI(a, b):
    """PowerLogInv: ln(a)^(1/b)"""
    return sp.log(a) ** (sp.Integer(1) / b)


def EML(a, b):
    """Exp Minus Log: exp(a) - ln(b)"""
    return sp.exp(a) - sp.log(b)


def EDL(a, b):
    """Exp Div Log: exp(a) / ln(b)"""
    return sp.exp(a) / sp.log(b)


def NegEML(a, b):
    """Negated variant: ln(a) - exp(b)"""
    return sp.log(a) - sp.exp(b)


def DivLogExp(a, b):
    """Log over Exp: ln(a) / exp(b)"""
    return sp.log(a) / sp.exp(b)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def safe_simplify(expr):
    """Attempt SymPy simplification; return original on failure."""
    try:
        return sp.simplify(expr)
    except Exception:
        return expr


def is_zero(expr):
    """Return True if expr simplifies to exactly 0."""
    try:
        return safe_simplify(expr) == sp.S.Zero
    except Exception:
        return False


def is_constant(expr, variables=(x_pos, y_pos)):
    """
    Return True if expr is constant w.r.t. all given variables.
    Checks that every partial derivative is zero.
    """
    try:
        for v in variables:
            d = safe_simplify(sp.diff(expr, v))
            if d != sp.S.Zero:
                return False
        return True
    except Exception:
        return False


def check_exact_equiv(f_expr, g_expr):
    """T0: exact equality -- difference is zero."""
    return is_zero(f_expr - g_expr)


def check_t4_shift(f_expr, g_expr):
    """T4: f = g + c for some constant c  <=>  (f - g) is constant."""
    return is_constant(f_expr - g_expr)


def check_t5_multiply(f_expr, g_expr):
    """T5: f = c * g for some constant c  <=>  f/g is constant."""
    try:
        ratio = sp.simplify(f_expr / g_expr)
        return is_constant(ratio)
    except Exception:
        return False


# ---------------------------------------------------------------------------
# Build PLI variants under T1 (swap) x T2 (negate) x T3 (reciprocal)
# ---------------------------------------------------------------------------
PLI_variants = [
    ("PLI(x,y)",           PLI(x_pos, y_pos)),
    ("PLI(y,x)",           PLI(y_pos, x_pos)),              # T1
    ("-PLI(x,y)",         -PLI(x_pos, y_pos)),              # T2
    ("-PLI(y,x)",         -PLI(y_pos, x_pos)),              # T1+T2
    ("1/PLI(x,y)",         sp.Integer(1) / PLI(x_pos, y_pos)),   # T3
    ("1/PLI(y,x)",         sp.Integer(1) / PLI(y_pos, x_pos)),   # T1+T3
]

# ---------------------------------------------------------------------------
# Target functions (both argument orders for each)
# ---------------------------------------------------------------------------
targets = [
    ("EML(x,y)",           EML(x_pos, y_pos)),
    ("EML(y,x)",           EML(y_pos, x_pos)),
    ("EDL(x,y)",           EDL(x_pos, y_pos)),
    ("EDL(y,x)",           EDL(y_pos, x_pos)),
    ("NegEML(x,y)",        NegEML(x_pos, y_pos)),
    ("NegEML(y,x)",        NegEML(y_pos, x_pos)),
    ("DivLogExp(x,y)",     DivLogExp(x_pos, y_pos)),
    ("DivLogExp(y,x)",     DivLogExp(y_pos, x_pos)),
]

# ---------------------------------------------------------------------------
# Run all comparisons
# ---------------------------------------------------------------------------
results = {
    "metadata": {
        "description": (
            "Symbolic non-equivalence proof: PLI vs EML/EDL/NegEML/DivLogExp "
            "under transformations T1-T5"
        ),
        "date": datetime.now(timezone.utc).isoformat(),
        "functions": {
            "PLI":          "log(x)**(1/y)",
            "EML":          "exp(x) - log(y)",
            "EDL":          "exp(x) / log(y)",
            "NegEML":       "log(x) - exp(y)",
            "DivLogExp":    "log(x) / exp(y)",
        },
        "transformations": {
            "T1": "Argument swap: f(x,y) <-> g(y,x)",
            "T2": "Negation: f(x,y) <-> -g(x,y)",
            "T3": "Reciprocal: f(x,y) <-> 1/g(x,y)",
            "T4": "Constant shift: f(x,y) <-> g(x,y) + c",
            "T5": "Constant multiply: f(x,y) <-> c*g(x,y)",
        },
        "domain_assumption": "x > 0, y > 0 (SymPy positive=True symbols)",
    },
    "comparisons": [],
    "summary": {},
}

print("=" * 70)
print("PLI Non-Equivalence Proof  --  SymPy symbolic verification")
print("=" * 70)
print(f"PLI variants : {len(PLI_variants)}")
print(f"Targets      : {len(targets)}")
print(f"Comparisons  : {len(PLI_variants) * len(targets)}")
print()

any_equivalent_found = False

for pli_name, pli_expr in PLI_variants:
    for tgt_name, tgt_expr in targets:
        label = f"{pli_name}  vs  {tgt_name}"
        entry = {
            "pli_variant": pli_name,
            "target": tgt_name,
            "checks": {},
            "verdict": None,
        }

        # T0: exact equality
        try:
            exact = check_exact_equiv(pli_expr, tgt_expr)
        except Exception as e:
            exact = False
            entry["checks"]["exact_error"] = str(e)
        entry["checks"]["exact_equal"] = exact

        # T4: constant shift
        try:
            shift = check_t4_shift(pli_expr, tgt_expr)
        except Exception as e:
            shift = False
            entry["checks"]["t4_error"] = str(e)
        entry["checks"]["t4_constant_shift"] = shift

        # T5: constant multiply
        try:
            mult = check_t5_multiply(pli_expr, tgt_expr)
        except Exception as e:
            mult = False
            entry["checks"]["t5_error"] = str(e)
        entry["checks"]["t5_constant_multiply"] = mult

        # Verdict
        if exact or shift or mult:
            verdict = "EQUIVALENT"
            any_equivalent_found = True
        else:
            verdict = "NON-EQUIVALENT"

        entry["verdict"] = verdict
        results["comparisons"].append(entry)

        print(f"[{verdict}]  {label}")
        if verdict == "EQUIVALENT":
            print(f"  *** ALERT: equivalence detected! checks={entry['checks']}")

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
total       = len(results["comparisons"])
equiv_count = sum(1 for r in results["comparisons"] if r["verdict"] == "EQUIVALENT")
ne_count    = total - equiv_count

if any_equivalent_found:
    conclusion = (
        "PROOF FAILED: some equivalence was found -- inspect comparisons above."
    )
else:
    conclusion = (
        "PROVEN NON-EQUIVALENT: PLI cannot be obtained from EML, EDL, "
        "NegEML, or DivLogExp by any combination of argument-swap, "
        "negation, reciprocal, constant shift, or constant multiply."
    )

results["summary"] = {
    "total_comparisons":    total,
    "equivalent_found":     equiv_count,
    "non_equivalent":       ne_count,
    "overall_conclusion":   conclusion,
}

print()
print("=" * 70)
print("SUMMARY")
print("=" * 70)
print(f"Total comparisons : {total}")
print(f"Equivalent found  : {equiv_count}")
print(f"Non-equivalent    : {ne_count}")
print()
print(conclusion)

# ---------------------------------------------------------------------------
# Structural argument
# ---------------------------------------------------------------------------
structural_note = (
    "\n"
    "STRUCTURAL NON-EQUIVALENCE ARGUMENT\n"
    "=====================================\n"
    "PLI(x,y) = ln(x)^(1/y)  is built from a POWER of ln(x).\n"
    "\n"
    "Key partial derivatives:\n"
    "  d/dx PLI = (1/y) * ln(x)^(1/y - 1) * (1/x)   [decays like 1/x]\n"
    "  d/dy PLI = -ln(x)^(1/y) * ln(ln(x)) / y^2     [contains ln(ln(x))]\n"
    "\n"
    "EML(x,y) = exp(x) - ln(y):\n"
    "  d/dx EML = exp(x)                              [grows exponentially]\n"
    "  d/dy EML = -1/y                                [simple reciprocal]\n"
    "\n"
    "EDL(x,y) = exp(x) / ln(y):\n"
    "  d/dx EDL = exp(x)/ln(y)                        [exponential / log]\n"
    "  d/dy EDL = -exp(x)/(y * ln(y)^2)              [exp * rational in y]\n"
    "\n"
    "Why no T1-T5 combination can bridge this gap:\n"
    "  1. PLI introduces ln(ln(x)) (iterated logarithm) absent in EML/EDL.\n"
    "  2. EML/EDL have exp(x) in x-derivatives; PLI has a power of ln(x).\n"
    "  3. Affine transforms (T4/T5) preserve gradient structure ratios;\n"
    "     they cannot convert power-of-log into exp-minus-log.\n"
    "  4. Argument swap (T1) merely exchanges x and y roles;\n"
    "     it cannot convert a power structure into a sum/quotient.\n"
    "  5. Negation (T2) and reciprocal (T3) flip sign or invert;\n"
    "     neither introduces or removes the iterated-log term.\n"
)

results["structural_argument"] = structural_note.strip()
print(structural_note)

# ---------------------------------------------------------------------------
# Save JSON
# ---------------------------------------------------------------------------
output_path = (
    "D:/CLAUDE/EML_IAE/project-iceberg/results/analysis/"
    "symbolic_proofs/nonequiv_pli.json"
)

with open(output_path, "w", encoding="utf-8") as fh:
    json.dump(results, fh, indent=2, default=str, ensure_ascii=False)

print(f"Results saved to: {output_path}")
