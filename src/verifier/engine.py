"""
Sheffer operator verification engine.
Implements the bootstrapping search from Odrzywołek (2026).

Algorithm:
1. Start with known constants + operations
2. Enumerate all expressions up to complexity K
3. Check if any expression matches a target function value
4. If match found, add target to known set, restart from K=1
5. Repeat until all 36 targets found or K_max exceeded
"""
from __future__ import annotations
import math
import time
import json
from dataclasses import dataclass, field
from typing import Callable, Optional
from .complex_ops import C, qkey, near, value_ok, EULER_GAMMA, GLAISHER, CATALAN, KHINCHIN


# --- Function/operator registry types ---

@dataclass
class UnaryFn:
    name: str
    f: Callable[[C], Optional[C]]

@dataclass
class BinaryFn:
    name: str
    f: Callable[[C, C], Optional[C]]
    commutative: bool = False

@dataclass
class ConstDef:
    name: str
    value: C


# --- Standard catalogs ---

def logistic_sigmoid(z: C) -> Optional[C]:
    return C.real(1.0).div(C.real(1.0).add(z.neg().exp()))


def standard_unary_catalog() -> dict[str, UnaryFn]:
    items = [
        ("Half", lambda x: x.mul(C.real(0.5))),
        ("Minus", lambda x: x.neg()),
        ("Log", lambda x: x.ln()),
        ("Exp", lambda x: x.exp()),
        ("Inv", lambda x: C.real(1.0).div(x)),
        ("Sqrt", lambda x: x.sqrt()),
        ("Sqr", lambda x: x.mul(x)),
        ("Cosh", lambda x: x.cosh()),
        ("Cos", lambda x: x.cos()),
        ("Sinh", lambda x: x.sinh()),
        ("Sin", lambda x: x.sin()),
        ("Tanh", lambda x: x.tanh()),
        ("Tan", lambda x: x.tan()),
        ("ArcSinh", lambda x: x.asinh()),
        ("ArcTanh", lambda x: x.atanh()),
        ("ArcSin", lambda x: x.asin()),
        ("ArcCos", lambda x: x.acos()),
        ("ArcTan", lambda x: x.atan()),
        ("ArcCosh", lambda x: x.acosh()),
        ("LogisticSigmoid", logistic_sigmoid),
    ]
    return {name: UnaryFn(name, f) for name, f in items}


def standard_binary_catalog() -> dict[str, BinaryFn]:
    items = [
        ("Plus", lambda a, b: a.add(b), True),
        ("Times", lambda a, b: a.mul(b), True),
        ("Subtract", lambda a, b: a.sub(b), False),
        ("Divide", lambda a, b: a.div(b), False),
        ("Power", lambda a, b: a.pow(b), False),
        ("Log", lambda a, b: _log_base(a, b), False),
        ("Avg", lambda a, b: a.add(b).mul(C.real(0.5)), True),
        ("Hypot", lambda a, b: a.mul(a).add(b.mul(b)).sqrt(), True),
        ("EML", lambda a, b: _eml_op(a, b), False),
    ]
    return {name: BinaryFn(name, f, comm) for name, f, comm in items}


def _eml_op(a: C, b: C) -> Optional[C]:
    """EML[a,b] = exp(a) - ln(b)"""
    lb = b.ln()
    if lb is None:
        return None
    return a.exp().sub(lb)


def _log_base(base: C, x: C) -> Optional[C]:
    """log_base(x) = ln(x)/ln(base)"""
    lx = x.ln()
    lb = base.ln()
    if lx is None or lb is None:
        return None
    return lx.div(lb)


def standard_constant_catalog() -> dict[str, ConstDef]:
    return {
        "0": ConstDef("0", C.real(0.0)),
        "EulerGamma": ConstDef("EulerGamma", C.real(EULER_GAMMA)),
        "Glaisher": ConstDef("Glaisher", C.real(GLAISHER)),
        "Catalan": ConstDef("Catalan", C.real(CATALAN)),
        "Khinchin": ConstDef("Khinchin", C.real(KHINCHIN)),
        "Pi": ConstDef("Pi", C.real(math.pi)),
        "E": ConstDef("E", C.real(math.e)),
        "I": ConstDef("I", C.i()),
        "1": ConstDef("1", C.real(1.0)),
        "-1": ConstDef("-1", C.real(-1.0)),
        "2": ConstDef("2", C.real(2.0)),
    }


# --- Target sets ---

DEFAULT_TARGET_CONSTANTS = ["Glaisher", "EulerGamma", "Pi", "E", "1", "-1", "2"]
DEFAULT_TARGET_UNARY = [
    "Half", "Minus", "Log", "Exp", "Inv", "Sqrt", "Sqr",
    "Cosh", "Cos", "Sinh", "Sin", "Tanh", "Tan",
    "ArcSinh", "ArcTanh", "ArcSin", "ArcCos", "ArcTan", "ArcCosh",
    "LogisticSigmoid",
]
DEFAULT_TARGET_BINARY = [
    "Plus", "Times", "Subtract", "Divide", "Power", "Log", "Avg", "Hypot",
]


# --- Core search algorithm ---

@dataclass
class SearchResult:
    found: bool
    witness_expr: Optional[str] = None
    witness_k: Optional[int] = None


def can_represent(
    target: C,
    constants: list[C],
    unary: list[UnaryFn],
    binary: list[BinaryFn],
    max_k: int,
    eps: float = 16 * 2.220446049250313e-16,
    real_only: bool = False,
) -> bool:
    """Fast check: can target be represented using given primitives within K steps?"""
    levels: list[list[C]] = [[] for _ in range(max_k + 1)]
    seen: set[tuple[int, int]] = set()

    for c in constants:
        if not value_ok(c, real_only, eps):
            continue
        key = qkey(c)
        if key not in seen:
            seen.add(key)
            levels[1].append(c)
            if near(c, target, eps):
                return True

    for k in range(2, max_k + 1):
        nxt: list[C] = []

        # Unary applications
        for u in unary:
            for x in levels[k - 1]:
                try:
                    y = u.f(x)
                except (OverflowError, ValueError, ZeroDivisionError):
                    continue
                if y is not None and value_ok(y, real_only, eps):
                    key = qkey(y)
                    if key not in seen:
                        seen.add(key)
                        if near(y, target, eps):
                            return True
                        nxt.append(y)

        # Binary applications
        for b in binary:
            for left_k in range(1, k - 1):
                right_k = k - 1 - left_k
                for a in levels[left_k]:
                    for bb in levels[right_k]:
                        if b.commutative and left_k == right_k and qkey(a) > qkey(bb):
                            continue
                        try:
                            y = b.f(a, bb)
                        except (OverflowError, ValueError, ZeroDivisionError):
                            continue
                        if y is not None and value_ok(y, real_only, eps):
                            key = qkey(y)
                            if key not in seen:
                                seen.add(key)
                                if near(y, target, eps):
                                    return True
                                nxt.append(y)

        levels[k] = nxt

    return False


def find_representation(
    target: C,
    constants: list[tuple[str, C]],
    unary: list[tuple[str, UnaryFn]],
    binary: list[tuple[str, BinaryFn]],
    max_k: int,
    eps: float = 16 * 2.220446049250313e-16,
    real_only: bool = False,
) -> Optional[tuple[str, int]]:
    """Find a witness expression for target. Returns (expr_str, k) or None."""
    levels: list[list[tuple[C, str]]] = [[] for _ in range(max_k + 1)]
    seen: set[tuple[int, int]] = set()

    for name, c in constants:
        if not value_ok(c, real_only, eps):
            continue
        key = qkey(c)
        if key not in seen:
            seen.add(key)
            levels[1].append((c, name))
            if near(c, target, eps):
                return (name, 1)

    for k in range(2, max_k + 1):
        nxt: list[tuple[C, str]] = []

        for u_name, u in unary:
            for x, x_expr in levels[k - 1]:
                try:
                    y = u.f(x)
                except (OverflowError, ValueError, ZeroDivisionError):
                    continue
                if y is not None and value_ok(y, real_only, eps):
                    expr = f"{u_name}[{x_expr}]"
                    if near(y, target, eps):
                        return (expr, k)
                    key = qkey(y)
                    if key not in seen:
                        seen.add(key)
                        nxt.append((y, expr))

        for b_name, b in binary:
            for left_k in range(1, k - 1):
                right_k = k - 1 - left_k
                for a, a_expr in levels[left_k]:
                    for bb, bb_expr in levels[right_k]:
                        if b.commutative and left_k == right_k and qkey(a) > qkey(bb):
                            continue
                        try:
                            y = b.f(a, bb)
                        except (OverflowError, ValueError, ZeroDivisionError):
                            continue
                        if y is not None and value_ok(y, real_only, eps):
                            expr = f"{b_name}[{a_expr}, {bb_expr}]"
                            if near(y, target, eps):
                                return (expr, k)
                            key = qkey(y)
                            if key not in seen:
                                seen.add(key)
                                nxt.append((y, expr))

        levels[k] = nxt

    return None


# --- Multi-point witness validation ---

SAMPLE_PAIRS = [
    (EULER_GAMMA, GLAISHER),
    (-EULER_GAMMA, GLAISHER),
    (CATALAN, KHINCHIN),
    (-CATALAN, KHINCHIN),
    (GLAISHER, EULER_GAMMA),
    (-GLAISHER, EULER_GAMMA),
    (KHINCHIN, CATALAN),
    (-KHINCHIN, CATALAN),
]


# --- Bootstrapping verifier ---

@dataclass
class VerifyResult:
    operator_name: str
    companion_constant: str
    found_constants: list[str] = field(default_factory=list)
    found_unary: list[str] = field(default_factory=list)
    found_binary: list[str] = field(default_factory=list)
    remaining_constants: list[str] = field(default_factory=list)
    remaining_unary: list[str] = field(default_factory=list)
    remaining_binary: list[str] = field(default_factory=list)
    witnesses: dict[str, tuple[str, int]] = field(default_factory=dict)
    total_found: int = 0
    total_targets: int = 0
    elapsed_seconds: float = 0.0
    is_sheffer: bool = False

    def to_dict(self) -> dict:
        return {
            "operator_name": self.operator_name,
            "companion_constant": self.companion_constant,
            "is_sheffer": self.is_sheffer,
            "total_found": self.total_found,
            "total_targets": self.total_targets,
            "found_constants": self.found_constants,
            "found_unary": self.found_unary,
            "found_binary": self.found_binary,
            "remaining_constants": self.remaining_constants,
            "remaining_unary": self.remaining_unary,
            "remaining_binary": self.remaining_binary,
            "witnesses": {k: {"expr": v[0], "k": v[1]} for k, v in self.witnesses.items()},
            "elapsed_seconds": round(self.elapsed_seconds, 3),
        }


def verify_operator(
    op_name: str,
    op_fn: Callable[[C, C], Optional[C]],
    op_commutative: bool,
    companion_constants: list[str],
    max_k: int = 9,
    eps: float = 16 * 2.220446049250313e-16,
    target_constants: Optional[list[str]] = None,
    target_unary: Optional[list[str]] = None,
    target_binary: Optional[list[str]] = None,
    verbose: bool = True,
) -> VerifyResult:
    """
    Run the bootstrapping verification for a candidate operator.

    This mirrors the author's VerifyBaseSet algorithm:
    - Start with {companion_constants} as known constants and {op_name} as known binary op
    - Also always include EulerGamma and Glaisher as evaluation anchors
    - Iteratively search for expressions matching target functions
    - When found, promote to known set and restart
    """
    t0 = time.time()

    all_consts = standard_constant_catalog()
    all_unary = standard_unary_catalog()
    all_binary = standard_binary_catalog()

    # Register the candidate operator
    all_binary[op_name] = BinaryFn(op_name, op_fn, op_commutative)

    # Known sets: start with companion constants + the operator
    known_consts = ["EulerGamma", "Glaisher"] + companion_constants
    known_consts = sorted(set(known_consts))
    known_unary: list[str] = []
    known_binary: list[str] = [op_name]

    # Target sets
    tc = sorted(set(target_constants or DEFAULT_TARGET_CONSTANTS))
    tu = sorted(set(target_unary or DEFAULT_TARGET_UNARY))
    tb = sorted(set(target_binary or DEFAULT_TARGET_BINARY))

    # Todo = target - known
    todo_c = [c for c in tc if c not in known_consts]
    todo_u = [u for u in tu if u not in known_unary]
    todo_b = [b for b in tb if b not in known_binary]

    total_targets = len(tc) + len(tu) + len(tb)
    result = VerifyResult(
        operator_name=op_name,
        companion_constant=",".join(companion_constants),
        total_targets=total_targets,
    )

    if verbose:
        print(f"=== Verifying operator: {op_name} ===")
        print(f"Companion constants: {companion_constants}")
        print(f"Targets: {len(tc)} consts, {len(tu)} unary, {len(tb)} binary = {total_targets}")
        print(f"Todo: {len(todo_c)} consts, {len(todo_u)} unary, {len(todo_b)} binary")

    k = 1
    while k <= max_k and (todo_c or todo_u or todo_b):
        if verbose:
            print(f"  K={k}: searching... (todo: {len(todo_c)}c/{len(todo_u)}u/{len(todo_b)}b)")

        # Build current known sets for search
        named_consts = [(n, all_consts[n].value) for n in known_consts if n in all_consts]
        named_unary = [(n, all_unary[n]) for n in known_unary if n in all_unary]
        named_binary = [(n, all_binary[n]) for n in known_binary if n in all_binary]

        found_something = False

        # Check constants
        for idx, c_name in enumerate(todo_c):
            if c_name not in all_consts:
                continue
            target_val = all_consts[c_name].value
            w = find_representation(target_val, named_consts, named_unary, named_binary, k, eps)
            if w is not None:
                todo_c.pop(idx)
                known_consts.append(c_name)
                known_consts = sorted(set(known_consts))
                result.found_constants.append(c_name)
                result.witnesses[f"const:{c_name}"] = w
                if verbose:
                    print(f"    FOUND constant: {c_name} = {w[0]} (K={w[1]})")
                found_something = True
                k = 1
                break

        if found_something:
            continue

        # Check unary functions (evaluate at EulerGamma)
        for idx, u_name in enumerate(todo_u):
            if u_name not in all_unary:
                continue
            uf = all_unary[u_name]
            try:
                target_val = uf.f(C.real(EULER_GAMMA))
            except (OverflowError, ValueError, ZeroDivisionError):
                continue
            if target_val is None:
                continue
            w = find_representation(target_val, named_consts, named_unary, named_binary, k, eps)
            if w is not None:
                todo_u.pop(idx)
                known_unary.append(u_name)
                known_unary = sorted(set(known_unary))
                result.found_unary.append(u_name)
                result.witnesses[f"unary:{u_name}"] = w
                if verbose:
                    print(f"    FOUND unary: {u_name} = {w[0]} (K={w[1]})")
                found_something = True
                k = 1
                break

        if found_something:
            continue

        # Check binary operations (evaluate at EulerGamma, Glaisher)
        for idx, b_name in enumerate(todo_b):
            if b_name not in all_binary:
                continue
            bf = all_binary[b_name]
            try:
                target_val = bf.f(C.real(EULER_GAMMA), C.real(GLAISHER))
            except (OverflowError, ValueError, ZeroDivisionError):
                continue
            if target_val is None:
                continue
            w = find_representation(target_val, named_consts, named_unary, named_binary, k, eps)
            if w is not None:
                todo_b.pop(idx)
                known_binary.append(b_name)
                known_binary = sorted(set(known_binary))
                result.found_binary.append(b_name)
                result.witnesses[f"binary:{b_name}"] = w
                if verbose:
                    print(f"    FOUND binary: {b_name} = {w[0]} (K={w[1]})")
                found_something = True
                k = 1
                break

        if found_something:
            continue

        k += 1

    result.remaining_constants = todo_c
    result.remaining_unary = todo_u
    result.remaining_binary = todo_b
    result.total_found = total_targets - len(todo_c) - len(todo_u) - len(todo_b)
    result.is_sheffer = not todo_c and not todo_u and not todo_b
    result.elapsed_seconds = time.time() - t0

    if verbose:
        print(f"\n=== Result: {'SHEFFER' if result.is_sheffer else 'NOT SHEFFER'} ===")
        print(f"Found {result.total_found}/{total_targets} targets in {result.elapsed_seconds:.1f}s")
        if todo_c:
            print(f"  Missing constants: {todo_c}")
        if todo_u:
            print(f"  Missing unary: {todo_u}")
        if todo_b:
            print(f"  Missing binary: {todo_b}")

    return result
