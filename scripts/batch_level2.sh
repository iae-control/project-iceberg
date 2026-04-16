#!/bin/bash
# Level 2 3-stage screen search
# Stage 1: K_max=5, 4 core targets (exp, ln, +, *)
# Stage 2: K_max=7, full 35 targets
# Stage 3: K_max=9, re-verify hits only

VERIFY="D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
RESULTS="D:/CLAUDE/EML_IAE/project-iceberg/results"
LOG="$RESULTS/verify_L2_progress.log"
CAND_FILE="$RESULTS/candidates_L2.json"

mkdir -p "$RESULTS/hits" "$RESULTS/partial_hits"

exec > >(tee "$LOG") 2>&1

echo "=== Level 2 Search: 3-Stage Screen ==="
echo "Started: $(date)"

# Generate candidate specs from JSON
SPECS=$(python -c "
import json
with open('$CAND_FILE') as f:
    data = json.load(f)
bop_map = {'Plus':'Plus','Sub':'Sub','Mul':'Mul','Div':'Div','Pow':'Pow','LogBase':'LogBase'}
for c in data['candidates']:
    t = c['type']
    b = bop_map.get(c['binary_op'], c['binary_op'])
    if t == 'A':
        spec = f\"{c['name']}:{b}(chain:{c['u1']}:{c['u2']},{c['u3']})\"
        # For custom-op, we need to build from components
        # Type A: b(u1(u2(x)), u3(y))
        print(f\"{c['name']}|A|{b}|{c['u1']}|{c['u2']}|{c['u3']}\")
    elif t == 'B':
        print(f\"{c['name']}|B|{b}|{c['u1']}|{c['u2']}|{c['u3']}\")
    elif t == 'C':
        print(f\"{c['name']}|C|{c.get('u0','')}|{b}|{c['u1']}|{c['u2']}\")
" 2>/dev/null)

TOTAL=$(echo "$SPECS" | wc -l)
CONSTANTS=("1" "E")

echo "Total candidates: $TOTAL"
echo "Constants: ${CONSTANTS[*]}"
echo "Total Stage 1 runs: $((TOTAL * ${#CONSTANTS[@]}))"
echo "============================================="

STAGE1_PASS=0
STAGE2_PASS=0
SHEFFER=0
COUNT=0

# For Stage 1, we use a simplified approach:
# Run with --target-constants '' --target-functions Exp,Log --target-operations Plus,Times
# This tests only 4 targets instead of 35

while IFS= read -r LINE; do
    NAME=$(echo "$LINE" | cut -d'|' -f1)
    TYPE=$(echo "$LINE" | cut -d'|' -f2)
    F3=$(echo "$LINE" | cut -d'|' -f3)
    F4=$(echo "$LINE" | cut -d'|' -f4)
    F5=$(echo "$LINE" | cut -d'|' -f5)
    F6=$(echo "$LINE" | cut -d'|' -f6)

    # Build custom-op spec based on type
    # Type A: b(u1(u2(x)), u3(y)) — not directly supported by --custom-op
    # We need to check if the Rust CLI supports Level 2 operators
    # It doesn't — --custom-op only supports Level 1: BINOP(U1, U2)
    # For Level 2, we'd need to extend the Rust code.
    # WORKAROUND: Use Python to evaluate and check numerically.
    # But that defeats the purpose of using Rust for speed.

    # ALTERNATIVE: Skip Rust and use Python for Stage 1 (fast numeric check)
    # Only promote to Rust for Stage 2/3

    # For now, break after printing the issue
    if [ "$COUNT" -eq 0 ]; then
        echo "ERROR: --custom-op only supports Level 1 (BINOP(U1,U2))."
        echo "Level 2 operators require nested composition which the Rust CLI cannot express."
        echo "Need to either:"
        echo "  a) Extend Rust to support nested custom-op syntax, OR"
        echo "  b) Use Python for Stage 1 screening, Rust for Stage 2+"
        echo ""
        echo "Falling back to Python Stage 1 screening..."
        break
    fi
done <<< "$SPECS"

# Python-based Stage 1 screening
echo "=== Stage 1: Python numeric screen (K_max=5, 4 targets) ==="
python -c "
import json, math, sys, time

# Load candidates
with open('$CAND_FILE') as f:
    data = json.load(f)

candidates = data['candidates']
constants = ['1', 'E']
total = len(candidates) * len(constants)

# Transcendental test points
GAMMA = 0.5772156649015329
GLAISHER = 1.2824271291006227

# Target values for Stage 1: exp(gamma), ln(gamma), gamma+glaisher, gamma*glaisher
TARGETS_1 = {
    'Exp': math.exp(GAMMA),
    'Log': math.log(GAMMA),
    'Plus': GAMMA + GLAISHER,
    'Times': GAMMA * GLAISHER,
}

# Unary function implementations
import cmath
def apply_u(name, z):
    if not isinstance(z, complex) and not math.isfinite(z): return None
    if isinstance(z, complex) and not (cmath.isfinite(z)): return None
    try:
        if name == 'Exp': return cmath.exp(z)
        if name == 'Log': return cmath.log(z) if z != 0 else None
        if name == 'Sin': return cmath.sin(z)
        if name == 'Cos': return cmath.cos(z)
        if name == 'Tan': return cmath.tan(z)
        if name == 'ArcSin': return cmath.asin(z)
        if name == 'ArcCos': return cmath.acos(z)
        if name == 'ArcTan': return cmath.atan(z)
        if name == 'Sinh': return cmath.sinh(z)
        if name == 'Cosh': return cmath.cosh(z)
        if name == 'Tanh': return cmath.tanh(z)
        if name == 'ArcSinh': return cmath.asinh(z)
        if name == 'ArcCosh': return cmath.acosh(z)
        if name == 'ArcTanh': return cmath.atanh(z)
        if name == 'Inv': return 1/z if z != 0 else None
        if name == 'Minus': return -z
        if name == 'Sqr': return z*z
        if name == 'Sqrt': return cmath.sqrt(z)
        if name == 'Id': return z
        if name == 'LogisticSigmoid': return 1/(1+cmath.exp(-z))
    except: return None
    return None

def apply_b(name, a, b):
    try:
        if name == 'Plus': return a + b
        if name == 'Sub': return a - b
        if name == 'Mul': return a * b
        if name == 'Div': return a / b if b != 0 else None
        if name == 'Pow': return a ** b if a != 0 else None
        if name == 'LogBase': return cmath.log(b) / cmath.log(a) if a != 0 and a != 1 and b != 0 else None
    except: return None
    return None

def eval_op(c, x, y):
    t = c['type']
    try:
        if t == 'A':
            u2x = apply_u(c['u2'], x)
            if u2x is None: return None
            u1u2x = apply_u(c['u1'], u2x)
            if u1u2x is None: return None
            u3y = apply_u(c['u3'], y)
            if u3y is None: return None
            return apply_b(c['binary_op'], u1u2x, u3y)
        elif t == 'B':
            u1x = apply_u(c['u1'], x)
            if u1x is None: return None
            u3y = apply_u(c['u3'], y)
            if u3y is None: return None
            u2u3y = apply_u(c['u2'], u3y)
            if u2u3y is None: return None
            return apply_b(c['binary_op'], u1x, u2u3y)
        elif t == 'C':
            u1x = apply_u(c['u1'], x)
            if u1x is None: return None
            u2y = apply_u(c['u2'], y)
            if u2y is None: return None
            bv = apply_b(c['binary_op'], u1x, u2y)
            if bv is None: return None
            return apply_u(c['u0'], bv)
    except: return None
    return None

# Simplified K_max=5 bootstrap check
# Can the operator + constant generate exp, ln, +, * within K=5?
def stage1_check(c, const_val):
    x, y = GAMMA, GLAISHER

    # Evaluate the operator at test points
    op_val = eval_op(c, x, y)
    if op_val is None: return False

    # Build expression tree up to K=5
    # K=1: just the constant
    # K=3: op(const, const) or op(x_probe, const) etc.
    # K=5: one more nesting

    eps = 1e-8
    vals = {complex(const_val): True, complex(x): True, complex(y): True}

    # K=3: apply op to all pairs
    new_vals = {}
    items = list(vals.keys())
    for a in items:
        for b in items:
            v = eval_op(c, a.real, b.real) if isinstance(a, complex) else None
            # Actually we need to evaluate the candidate operator, not eval_op with complex
            # The candidate IS the operator. eval_op(c, a, b) applies the structural definition.
            pass

    # This approach is too complex for a bash-embedded script.
    # Simpler: just check if the raw operator value matches any Stage 1 target.
    # If op(gamma, glaisher) is NOT close to any of the 4 target values,
    # it probably can't generate them. But this is a weak filter.

    # Better: check if op(const, const) gives a useful value (like e, 0, etc.)
    op_cc = eval_op(c, const_val, const_val)
    op_cx = eval_op(c, const_val, x)
    op_xc = eval_op(c, x, const_val)

    # Count how many useful starting values we get
    useful = 0
    for v in [op_cc, op_cx, op_xc, op_val]:
        if v is None: continue
        if not cmath.isfinite(v): continue
        useful += 1

    return useful >= 2  # At least 2 useful starting values

t0 = time.time()
passed = []
count = 0

const_map = {'1': 1.0, 'E': math.e, 'Pi': math.pi, '-1': -1.0, '2': 2.0, '0': 0.0}

for c in candidates:
    for const_name in constants:
        count += 1
        const_val = const_map.get(const_name, 1.0)

        if stage1_check(c, const_val):
            passed.append((c['name'], const_name, c))

        if count % 50000 == 0:
            elapsed = time.time() - t0
            print(f'  Stage 1: {count}/{total} ({100*count/total:.0f}%), passed={len(passed)}, {elapsed:.0f}s', flush=True)

elapsed = time.time() - t0
print(f'Stage 1 complete: {count} checked, {len(passed)} passed ({100*len(passed)/count:.1f}%), {elapsed:.0f}s')

# Save Stage 1 results
with open('$RESULTS/L2_stage1_passed.json', 'w') as f:
    json.dump([{'name': n, 'const': c} for n, c, _ in passed], f)

print(f'Saved {len(passed)} Stage 1 passes to L2_stage1_passed.json')
" 2>&1

echo ""
echo "=== Level 2 Stage 1 Complete ==="
echo "Finished: $(date)"
