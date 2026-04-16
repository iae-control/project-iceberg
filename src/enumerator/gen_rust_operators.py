"""
Generate Rust source code for all Level 1 candidate operators.
Produces a Rust function body that can be inserted into binary_catalog().
"""
import json
import os

# Mapping from unary function name to Rust evaluation code
UNARY_RUST = {
    "Exp":    "({x}).exp()",
    "Log":    "({x}).ln()?",
    "Sin":    "({x}).sin()",
    "Cos":    "({x}).cos()",
    "Tan":    "({x}).tan()?",
    "ArcSin": "({x}).asin()?",
    "ArcCos": "({x}).acos()?",
    "ArcTan": "({x}).atan()?",
    "Sinh":   "({x}).sinh()",
    "Cosh":   "({x}).cosh()",
    "Tanh":   "({x}).tanh()?",
    "ArcSinh": "({x}).asinh()?",
    "ArcCosh": "({x}).acosh()?",
    "ArcTanh": "({x}).atanh()?",
    "Inv":    "C::real(1.0).div({x})?",
    "Minus":  "({x}).neg()",
    "Sqr":    "({x}).mul({x})",
    "Sqrt":   "({x}).sqrt()",
    "Id":     "{x}",
    "LogisticSigmoid": "logistic_sigmoid({x})?",
}

# Which unary functions return Option<C> (need ?)
UNARY_OPTION = {
    "Log", "Tan", "ArcSin", "ArcCos", "ArcTan",
    "Tanh", "ArcSinh", "ArcCosh", "ArcTanh",
    "Inv", "LogisticSigmoid",
}

# Mapping binary operation to Rust code combining u1(a) and u2(b)
BINARY_RUST = {
    "Plus":     "Some({left}.add({right}))",
    "Subtract": "Some({left}.sub({right}))",
    "Times":    "Some({left}.mul({right}))",
    "Divide":   "{left}.div({right})",
    "Power":    "{left}.pow({right})",
    "LogBase":  "{right}.ln()?.div({left}.ln()?)",  # log_base(x,y) = ln(y)/ln(x)
}

BINARY_COMMUTATIVE = {"Plus", "Times"}


def gen_operator_rust(binary_op: str, u1: str, u2: str) -> str:
    """Generate a single (name, Binary{...}) entry."""
    name = f"{binary_op}_{u1}_{u2}"
    comm = binary_op in BINARY_COMMUTATIVE and u1 == u2

    # Generate the lambda body
    left_code = UNARY_RUST[u1].replace("{x}", "a")
    right_code = UNARY_RUST[u2].replace("{x}", "b")

    # Some unary functions that produce Option need special handling
    # We need to assign them to temp variables if they're used in the binary op
    needs_let_left = u1 in UNARY_OPTION
    needs_let_right = u2 in UNARY_OPTION

    body_lines = []

    if needs_let_left:
        body_lines.append(f"let left = {left_code};")
        left_var = "left"
    else:
        left_var = left_code

    if needs_let_right:
        body_lines.append(f"let right = {right_code};")
        right_var = "right"
    else:
        right_var = right_code

    # Binary combination
    binary_code = BINARY_RUST[binary_op].replace("{left}", left_var).replace("{right}", right_var)

    if body_lines:
        body = " ".join(body_lines) + f" {binary_code}"
    else:
        body = binary_code

    return f'''        (
            "{name}",
            Binary {{
                f: |a, b| {{ {body} }},
                commutative: {"true" if comm else "false"},
            }},
        ),'''


def gen_all_operators():
    """Generate Rust code for all valid Level 1 operators."""
    # Load candidate list
    cand_file = os.path.join(os.path.dirname(__file__), '..', '..', 'results', 'candidates_L1.json')
    with open(cand_file) as f:
        data = json.load(f)

    candidates = data["candidates"]
    print(f"Generating Rust code for {len(candidates)} candidates...")

    lines = []
    lines.append("        // === AUTO-GENERATED Level 1 candidate operators ===")
    lines.append(f"        // Total: {len(candidates)} operators")

    errors = []
    for c in candidates:
        try:
            rust_code = gen_operator_rust(c["binary_op"], c["unary1"], c["unary2"])
            lines.append(rust_code)
        except Exception as e:
            errors.append(f"  Error generating {c['name']}: {e}")

    if errors:
        print("Errors:")
        for e in errors:
            print(e)

    rust_code = "\n".join(lines)

    # Save to file
    out_file = os.path.join(os.path.dirname(__file__), '..', '..', 'src', 'rust_engine', 'level1_operators.rs')
    os.makedirs(os.path.dirname(out_file), exist_ok=True)
    with open(out_file, 'w') as f:
        f.write(rust_code)

    print(f"Generated {len(candidates)} operators -> {out_file}")
    print(f"Total lines: {len(lines)}")
    return rust_code


if __name__ == "__main__":
    gen_all_operators()
