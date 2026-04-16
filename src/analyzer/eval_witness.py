"""
Evaluate witness expressions from the Rust verification engine.
Parses Mathematica-style expressions like "EML[Log[EulerGamma], Exp[Glaisher]]"
and evaluates them numerically to measure precision.
"""
from __future__ import annotations
import math
import re
from typing import Optional


# Operator implementations
def eml_op(a: float, b: float) -> Optional[float]:
    if b <= 0:
        return None
    try:
        return math.exp(a) - math.log(b)
    except OverflowError:
        return None


def edl_op(a: float, b: float) -> Optional[float]:
    if b <= 0:
        return None
    lb = math.log(b)
    if lb == 0:
        return None
    try:
        return math.exp(a) / lb
    except OverflowError:
        return None


def dle_op(a: float, b: float) -> Optional[float]:
    if a <= 0:
        return None
    try:
        eb = math.exp(b)
        if not math.isfinite(eb):
            return None
        return math.log(a) / eb
    except OverflowError:
        return None


# Standard function implementations
FUNCTIONS = {
    "Exp": lambda x: math.exp(x),
    "Log": lambda x: math.log(x) if x > 0 else None,
    "Minus": lambda x: -x,
    "Inv": lambda x: 1.0/x if x != 0 else None,
    "Half": lambda x: x/2,
    "Sqrt": lambda x: math.sqrt(x) if x >= 0 else None,
    "Sqr": lambda x: x*x,
    "Sin": lambda x: math.sin(x),
    "Cos": lambda x: math.cos(x),
    "Tan": lambda x: math.tan(x),
    "Sinh": lambda x: math.sinh(x),
    "Cosh": lambda x: math.cosh(x),
    "Tanh": lambda x: math.tanh(x),
    "ArcSin": lambda x: math.asin(x) if -1 <= x <= 1 else None,
    "ArcCos": lambda x: math.acos(x) if -1 <= x <= 1 else None,
    "ArcTan": lambda x: math.atan(x),
    "ArcSinh": lambda x: math.asinh(x),
    "ArcCosh": lambda x: math.acosh(x) if x >= 1 else None,
    "ArcTanh": lambda x: math.atanh(x) if -1 < x < 1 else None,
    "LogisticSigmoid": lambda x: 1.0 / (1.0 + math.exp(-x)),
}

BINARY_OPS = {
    "Plus": lambda a, b: a + b,
    "Subtract": lambda a, b: a - b,
    "Times": lambda a, b: a * b,
    "Divide": lambda a, b: a / b if b != 0 else None,
    "Power": lambda a, b: math.pow(a, b) if a > 0 else None,
    "Log": lambda a, b: math.log(b) / math.log(a) if a > 0 and a != 1 and b > 0 else None,
    "Avg": lambda a, b: (a + b) / 2,
    "Hypot": lambda a, b: math.hypot(a, b),
    "EML": eml_op,
    "EDL": edl_op,
    "DivLogExp": dle_op,
}

CONSTANTS = {
    "1": 1.0,
    "-1": -1.0,
    "2": 2.0,
    "0": 0.0,
    "E": math.e,
    "Pi": math.pi,
    "I": None,  # Can't handle imaginary in float64
    "EulerGamma": 0.5772156649015329,
    "Glaisher": 1.2824271291006227,
}


def parse_expr(s: str) -> tuple:
    """Parse Mathematica-style expression to AST.
    Returns ('atom', name) or ('call', name, [args])"""
    s = s.strip()
    i = 0
    n = len(s)

    def skip_ws():
        nonlocal i
        while i < n and s[i].isspace():
            i += 1

    def parse_ident():
        nonlocal i
        skip_ws()
        j = i
        while i < n and s[i] not in '[], \t':
            i += 1
        if i == j:
            return None
        return s[j:i]

    def parse_rec():
        nonlocal i
        name = parse_ident()
        if name is None:
            return None
        skip_ws()
        if i < n and s[i] == '[':
            i += 1
            args = []
            while True:
                skip_ws()
                if i < n and s[i] == ']':
                    i += 1
                    break
                a = parse_rec()
                if a is None:
                    return None
                args.append(a)
                skip_ws()
                if i < n and s[i] == ',':
                    i += 1
                    continue
                if i < n and s[i] == ']':
                    i += 1
                    break
                return None
            return ('call', name, args)
        return ('atom', name)

    result = parse_rec()
    skip_ws()
    return result if i == n else None


def eval_expr(ast, env: dict[str, float]) -> Optional[float]:
    """Evaluate a parsed expression with given variable bindings."""
    if ast is None:
        return None
    if ast[0] == 'atom':
        name = ast[1]
        if name in env:
            return env[name]
        if name in CONSTANTS:
            return CONSTANTS[name]
        # Try parsing as number
        try:
            return float(name)
        except ValueError:
            return None
    elif ast[0] == 'call':
        _, name, args = ast
        vals = [eval_expr(a, env) for a in args]
        if any(v is None for v in vals):
            return None
        if len(vals) == 1:
            if name in FUNCTIONS:
                try:
                    return FUNCTIONS[name](vals[0])
                except (ValueError, OverflowError, ZeroDivisionError):
                    return None
            return None
        elif len(vals) == 2:
            if name in BINARY_OPS:
                try:
                    return BINARY_OPS[name](vals[0], vals[1])
                except (ValueError, OverflowError, ZeroDivisionError):
                    return None
            return None
    return None


def evaluate_witness(witness_expr: str, x: float = None, y: float = None) -> Optional[float]:
    """Evaluate a witness expression with EulerGamma=x (or default) and Glaisher=y."""
    env = {}
    env["EulerGamma"] = x if x is not None else 0.5772156649015329
    env["Glaisher"] = y if y is not None else 1.2824271291006227
    ast = parse_expr(witness_expr)
    return eval_expr(ast, env)


# Test
if __name__ == "__main__":
    # Test with known EML witnesses
    tests = [
        ("EML[1, 1]", math.e),
        ("EML[EulerGamma, 1]", math.exp(0.5772156649015329)),
        ("EML[1, Exp[EML[1, EulerGamma]]]", math.log(0.5772156649015329)),
        ("EML[Log[EulerGamma], Exp[Glaisher]]", 0.5772156649015329 - 1.2824271291006227),
        ("Subtract[1, -1]", 2.0),
    ]

    print("Witness expression evaluation tests:")
    for expr, expected in tests:
        result = evaluate_witness(expr)
        if result is not None and expected != 0:
            rel_err = abs(result - expected) / abs(expected)
        elif result is not None:
            rel_err = abs(result)
        else:
            rel_err = float('inf')
        status = "PASS" if rel_err < 1e-10 else "FAIL"
        print(f"  {status}: {expr}")
        print(f"    expected={expected}, got={result}, rel_err={rel_err:.2e}")
