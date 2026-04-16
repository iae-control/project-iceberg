"""
SymPy Symbolic Verification of the PowerLogInv (PLI) Bootstrapping Chain.

PLI(x, y) = ln(x)^(1/y)

This script verifies 5 core derivations symbolically and checks the real-domain
requirement for all intermediate values.
"""

import json
import sys
from datetime import datetime

import sympy as sp

# ---------------------------------------------------------------------------
# Symbols – positive=True implies real=True in SymPy
# ---------------------------------------------------------------------------
x, y = sp.symbols("x y", positive=True, real=True)

results = {
    "metadata": {
        "script": "sympy_pli_chain.py",
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "sympy_version": sp.__version__,
        "python_version": sys.version,
    },
    "derivations": [],
    "real_domain_checks": [],
    "summary": {},
}

PASS = "PASS"
FAIL = "FAIL"

# ============================================================
# Helper
# ============================================================

def verify_equal(expr, target, label=""):
    """Return True if simplify(expr - target) == 0."""
    diff = sp.simplify(expr - target)
    ok = diff == sp.S.Zero
    return ok, diff


def is_real_positive(expr, label=""):
    """
    Check that an expression is real for positive real x, y.
    We do this by:
      1. Verifying sp.im(simplify(expr)) == 0
      2. Verifying a numeric substitution (x=2, y=3) yields a real float.
    Both must pass.
    """
    simplified = sp.simplify(expr)
    imag_part = sp.simplify(sp.im(simplified))
    symbolic_real = imag_part == sp.S.Zero

    # Numeric check with safe positive values
    try:
        val = complex(simplified.subs([(x, 2), (y, 3)]))
        numeric_real = abs(val.imag) < 1e-12
    except Exception as e:
        numeric_real = False

    return symbolic_real and numeric_real


# ============================================================
# PLI definition and helpers
# ============================================================

def PLI(a, b):
    """PLI(a, b) = ln(a)^(1/b)"""
    return sp.log(a) ** (sp.Integer(1) / b)


def Exp(a):
    return sp.exp(a)


def Inv(a):
    return sp.Integer(1) / a


def Log(a):
    return sp.log(a)


def Power(a, b):
    return a ** b


def Divide(a, b):
    return a / b


def Minus(a):
    """Minus(x) = Log(Inv(Exp(x))) = ln(1/exp(x)) = -x"""
    return Log(Inv(Exp(a)))


def Subtract(a, b):
    """Subtract(a, b) = Log(Divide(Exp(a), Exp(b)))"""
    return Log(Divide(Exp(a), Exp(b)))


# ============================================================
# Derivation 1 – ln(x) from PLI
# ============================================================
print("=" * 60)
print("Derivation 1: ln(x) from PLI(x, 1)")
print("=" * 60)

expr1_step1 = PLI(x, sp.Integer(1))
print(f"  PLI(x, 1)  = ln(x)^(1/1) = {expr1_step1}")

expr1_simplified = sp.simplify(expr1_step1)
print(f"  simplified = {expr1_simplified}")

target1 = sp.log(x)
ok1, diff1 = verify_equal(expr1_simplified, target1)
status1 = PASS if ok1 else FAIL
print(f"  Target     : {target1}")
print(f"  Difference : {diff1}")
print(f"  Result     : {status1}")

results["derivations"].append({
    "id": 1,
    "name": "ln(x) from PLI",
    "expression": "PLI(x, 1)",
    "target": "ln(x)",
    "expr_str": str(expr1_step1),
    "simplified_str": str(expr1_simplified),
    "difference": str(diff1),
    "status": status1,
})

# ============================================================
# Derivation 2 – x^y (Power) from PLI
# ============================================================
print()
print("=" * 60)
print("Derivation 2: x^y from PLI(Exp(x), Inv(y))")
print("=" * 60)

# Step 2a: PLI(exp(x), 1/y)
expr2_raw = PLI(Exp(x), Inv(y))
print(f"  PLI(exp(x), 1/y) = ln(exp(x))^(1/(1/y)) = {expr2_raw}")

# Step 2b: ln(exp(x)) simplifies to x
ln_exp_x = sp.simplify(sp.log(sp.exp(x)))
print(f"  ln(exp(x))       = {ln_exp_x}  (simplifies to x)")

# Step 2c: x^(1/(1/y)) = x^y
expr2_simplified = sp.simplify(expr2_raw)
print(f"  Full simplify    = {expr2_simplified}")

target2 = x ** y
ok2, diff2 = verify_equal(expr2_simplified, target2)
status2 = PASS if ok2 else FAIL
print(f"  Target           : {target2}")
print(f"  Difference       : {diff2}")
print(f"  Result           : {status2}")

results["derivations"].append({
    "id": 2,
    "name": "x^y (Power) from PLI",
    "expression": "PLI(Exp(x), Inv(y))",
    "target": "x**y",
    "ln_exp_x_step": str(ln_exp_x),
    "expr_str": str(expr2_raw),
    "simplified_str": str(expr2_simplified),
    "difference": str(diff2),
    "status": status2,
})

# ============================================================
# Derivation 3 – x*y (Times) from Power
# ============================================================
print()
print("=" * 60)
print("Derivation 3: x*y from Log(Power(Exp(x), y))")
print("=" * 60)

# Power(Exp(x), y) = exp(x)^y  (already verified above as x^y when base=exp(x))
# Log(exp(x)^y) = y * ln(exp(x)) = y * x = x*y
expr3_inner = Power(Exp(x), y)
print(f"  Power(Exp(x), y) = exp(x)^y = {expr3_inner}")

expr3_raw = Log(expr3_inner)
print(f"  Log(exp(x)^y)    = {expr3_raw}")

expr3_simplified = sp.simplify(expr3_raw)
print(f"  Simplified       = {expr3_simplified}")

target3 = x * y
ok3, diff3 = verify_equal(expr3_simplified, target3)
status3 = PASS if ok3 else FAIL
print(f"  Target           : {target3}")
print(f"  Difference       : {diff3}")
print(f"  Result           : {status3}")

results["derivations"].append({
    "id": 3,
    "name": "x*y (Times) from Power",
    "expression": "Log(Power(Exp(x), y))",
    "target": "x*y",
    "expr_str": str(expr3_raw),
    "simplified_str": str(expr3_simplified),
    "difference": str(diff3),
    "status": status3,
})

# ============================================================
# Derivation 4 – x-y (Subtract) from Divide
# ============================================================
print()
print("=" * 60)
print("Derivation 4: x-y from Log(Divide(Exp(x), Exp(y)))")
print("=" * 60)

# Divide(Exp(x), Exp(y)) = exp(x)/exp(y) = exp(x-y)
# Log(exp(x-y)) = x - y
expr4_inner = Divide(Exp(x), Exp(y))
print(f"  Divide(exp(x), exp(y)) = {expr4_inner}")

expr4_raw = Log(expr4_inner)
print(f"  Log(exp(x)/exp(y))     = {expr4_raw}")

expr4_simplified = sp.simplify(expr4_raw)
print(f"  Simplified             = {expr4_simplified}")

target4 = x - y
ok4, diff4 = verify_equal(expr4_simplified, target4)
status4 = PASS if ok4 else FAIL
print(f"  Target                 : {target4}")
print(f"  Difference             : {diff4}")
print(f"  Result                 : {status4}")

results["derivations"].append({
    "id": 4,
    "name": "x-y (Subtract) from Divide",
    "expression": "Log(Divide(Exp(x), Exp(y)))",
    "target": "x - y",
    "expr_str": str(expr4_raw),
    "simplified_str": str(expr4_simplified),
    "difference": str(diff4),
    "status": status4,
})

# ============================================================
# Derivation 5 – x+y (Plus) from Subtract and Minus
# ============================================================
print()
print("=" * 60)
print("Derivation 5: x+y from Subtract(x, Minus(y))")
print("=" * 60)

# Minus(y) = Log(Inv(Exp(y))) = ln(1/exp(y)) = -y
minus_y_raw = Log(Inv(Exp(y)))
minus_y_simplified = sp.simplify(minus_y_raw)
print(f"  Minus(y) = ln(1/exp(y)) = {minus_y_raw}")
print(f"           simplified     = {minus_y_simplified}")

# Subtract(x, Minus(y)) = Log(Divide(Exp(x), Exp(Minus(y))))
# Minus(y) = -y, so Exp(Minus(y)) = exp(-y)
# Divide(exp(x), exp(-y)) = exp(x) / exp(-y) = exp(x+y)
# Log(exp(x+y)) = x + y

# Build symbolically: substitute Minus(y) as the second argument of Subtract
minus_y_expr = minus_y_simplified   # = -y
expr5_raw = Subtract(x, minus_y_expr)
print(f"  Subtract(x, -y) = Log(Divide(exp(x), exp(-y))) = {expr5_raw}")

expr5_simplified = sp.simplify(expr5_raw)
print(f"  Simplified       = {expr5_simplified}")

target5 = x + y
ok5, diff5 = verify_equal(expr5_simplified, target5)
status5 = PASS if ok5 else FAIL
print(f"  Target           : {target5}")
print(f"  Difference       : {diff5}")
print(f"  Result           : {status5}")

results["derivations"].append({
    "id": 5,
    "name": "x+y (Plus) from Subtract and Minus",
    "expression": "Subtract(x, Minus(y))",
    "target": "x + y",
    "minus_y_raw": str(minus_y_raw),
    "minus_y_simplified": str(minus_y_simplified),
    "expr_str": str(expr5_raw),
    "simplified_str": str(expr5_simplified),
    "difference": str(diff5),
    "status": status5,
})

# ============================================================
# Real-domain checks
# ============================================================
print()
print("=" * 60)
print("Real-domain checks for positive real inputs")
print("=" * 60)

real_checks = [
    ("ln(exp(x))",          sp.log(sp.exp(x))),
    ("x^y",                 x ** y),
    ("exp(x)^y",            sp.exp(x) ** y),
    ("exp(x)/exp(y)",       sp.exp(x) / sp.exp(y)),
    ("ln(exp(x)/exp(y))",   sp.log(sp.exp(x) / sp.exp(y))),
    ("ln(1/exp(y))",        sp.log(1 / sp.exp(y))),
]

for label, expr in real_checks:
    ok = is_real_positive(expr)
    status = PASS if ok else FAIL
    simplified = sp.simplify(expr)
    imag = sp.simplify(sp.im(simplified))
    print(f"  {label:<28}  imaginary part = {str(imag):<6}  -> {status}")
    results["real_domain_checks"].append({
        "expression": label,
        "simplified": str(simplified),
        "imaginary_part": str(imag),
        "status": status,
    })

# ============================================================
# Summary
# ============================================================
print()
print("=" * 60)
print("SUMMARY")
print("=" * 60)

deriv_statuses   = [d["status"] for d in results["derivations"]]
real_statuses    = [r["status"] for r in results["real_domain_checks"]]
all_pass_deriv   = all(s == PASS for s in deriv_statuses)
all_pass_real    = all(s == PASS for s in real_statuses)
overall          = PASS if (all_pass_deriv and all_pass_real) else FAIL

results["summary"] = {
    "derivations_pass": sum(s == PASS for s in deriv_statuses),
    "derivations_total": len(deriv_statuses),
    "real_checks_pass": sum(s == PASS for s in real_statuses),
    "real_checks_total": len(real_statuses),
    "all_derivations_pass": all_pass_deriv,
    "all_real_checks_pass": all_pass_real,
    "overall": overall,
}

for d in results["derivations"]:
    print(f"  Derivation {d['id']} ({d['name']}): {d['status']}")
for r in results["real_domain_checks"]:
    print(f"  Real-domain [{r['expression']}]: {r['status']}")

print()
print(f"  Derivations : {results['summary']['derivations_pass']}/{results['summary']['derivations_total']} PASS")
print(f"  Real checks : {results['summary']['real_checks_pass']}/{results['summary']['real_checks_total']} PASS")
print(f"  OVERALL     : {overall}")

# ============================================================
# Write JSON results
# ============================================================
output_path = (
    "D:/CLAUDE/EML_IAE/project-iceberg/results/analysis/"
    "symbolic_proofs/pli_chain_verification.json"
)

with open(output_path, "w", encoding="utf-8") as fh:
    json.dump(results, fh, indent=2, ensure_ascii=False)

print()
print(f"Results written to: {output_path}")
