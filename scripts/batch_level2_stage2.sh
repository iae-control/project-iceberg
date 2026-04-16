#!/bin/bash
# Level 2 Stage 2: Rust K_max=7 verification of 635 filtered candidates
VERIFY="D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
RESULTS="D:/CLAUDE/EML_IAE/project-iceberg/results"
LOG="$RESULTS/verify_L2_stage2.log"

exec > >(tee "$LOG") 2>&1

echo "=== Level 2 Stage 2: Rust K_max=7 ==="
echo "Started: $(date)"

# Generate --custom-op specs from filtered candidates
SPECS=$(python -c "
import json
with open('$RESULTS/L2_stage1_filtered.json') as f:
    data = json.load(f)

bop_map = {'Plus':'Plus','Sub':'Sub','Mul':'Mul','Div':'Div','Pow':'Pow','LogBase':'LogBase'}

for item in data['candidates']:
    c = item['candidate']
    const = item['const']
    t = c['type']
    b = bop_map.get(c['binary_op'], c['binary_op'])
    name = c['name']

    if t == 'A':
        # b(u1(u2(x)), u3(y)) => BINOP(U1(U2), U3)
        u1, u2, u3 = c['u1'], c['u2'], c['u3']
        if u2 == 'Id':
            left = u1
        elif u1 == 'Id':
            left = u2
        else:
            left = f'{u1}({u2})'
        right = u3
        spec = f'{name}:{b}({left},{right})'
    elif t == 'B':
        # b(u1(x), u2(u3(y))) => BINOP(U1, U2(U3))
        u1, u2, u3 = c['u1'], c['u2'], c['u3']
        left = u1
        if u3 == 'Id':
            right = u2
        elif u2 == 'Id':
            right = u3
        else:
            right = f'{u2}({u3})'
        spec = f'{name}:{b}({left},{right})'
    elif t == 'C':
        # u0(b(u1(x), u2(y))) => U0(BINOP(U1, U2))
        u0, u1, u2 = c['u0'], c['u1'], c['u2']
        spec = f'{name}:{u0}({b}({u1},{u2}))'
    else:
        continue

    print(f'{spec}|{const}|{name}')
" 2>/dev/null)

TOTAL=$(echo "$SPECS" | wc -l)
echo "Filtered candidates: $TOTAL"

SHEFFER=0
PARTIAL_10=0
COUNT=0
BEST=0
BEST_NAME=""

while IFS='|' read -r SPEC CONST NAME; do
    COUNT=$((COUNT + 1))

    OUTPUT=$("$VERIFY" --constants "$CONST" --functions '' \
        --custom-op "$SPEC" \
        --operations "$NAME" \
        --max-k 7 2>&1)

    RC=$(echo "$OUTPUT" | grep "^Remaining constants:" | tail -1 | grep -o '"[^"]*"' | wc -l)
    RU=$(echo "$OUTPUT" | grep "^Remaining unary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
    RB=$(echo "$OUTPUT" | grep "^Remaining binary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
    REMAINING=$((RC + RU + RB))
    FOUND=$((35 - REMAINING))

    if [ "$FOUND" -ge 10 ]; then
        ELAPSED=$(echo "$OUTPUT" | grep "^Elapsed:" | awk '{print $2}')
        echo "[$COUNT/$TOTAL] HIT: $NAME + $CONST -> $FOUND/35 ($ELAPSED)"
        PARTIAL_10=$((PARTIAL_10 + 1))
        echo "$OUTPUT" > "$RESULTS/partial_hits/L2_${NAME}_${CONST}.log"
    fi

    if [ "$REMAINING" -eq 0 ]; then
        echo "*** SHEFFER: $NAME + $CONST -> 35/35! ***"
        mkdir -p "$RESULTS/hits"
        echo "$OUTPUT" > "$RESULTS/hits/L2_${NAME}_${CONST}.log"
        SHEFFER=$((SHEFFER + 1))
    fi

    if [ "$FOUND" -gt "$BEST" ]; then
        BEST=$FOUND
        BEST_NAME="$NAME+$CONST"
    fi

    if [ $((COUNT % 100)) -eq 0 ]; then
        echo "  Progress: $COUNT/$TOTAL | Sheffer: $SHEFFER | 10+: $PARTIAL_10 | Best: $BEST/35"
    fi
done <<< "$SPECS"

echo ""
echo "============================================="
echo "Level 2 Stage 2 Complete"
echo "Total: $COUNT | Sheffer: $SHEFFER | 10+: $PARTIAL_10"
echo "Best: $BEST_NAME ($BEST/35)"
echo "Finished: $(date)"
