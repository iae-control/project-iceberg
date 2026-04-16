#!/bin/bash
# Full Level 1 search: all 2011 candidates × 7 constants
# Uses --custom-op for dynamic operator registration
# Quick screen: max-k=7

VERIFY="D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
RESULTS="D:/CLAUDE/EML_IAE/project-iceberg/results"
CAND_FILE="$RESULTS/candidates_L1.json"

mkdir -p "$RESULTS/verify_L1_log"
mkdir -p "$RESULTS/hits"
mkdir -p "$RESULTS/partial_hits"

CONSTANTS=("1" "E" "Pi" "-1" "2" "0" "I")

# Read candidates from JSON using Python
CANDIDATES=$(python -c "
import json
with open('$CAND_FILE') as f:
    data = json.load(f)
for c in data['candidates']:
    # Map binary op names for --custom-op format
    bop = c['binary_op']
    bop_map = {'Plus':'Plus','Subtract':'Sub','Times':'Mul','Divide':'Div','Power':'Pow','LogBase':'LogBase'}
    bop_mapped = bop_map.get(bop, bop)
    print(f\"{c['name']}:{bop_mapped}({c['unary1']},{c['unary2']})\")
")

TOTAL_CANDS=$(echo "$CANDIDATES" | wc -l)
echo "=== Level 1 Full Search ==="
echo "Candidates: $TOTAL_CANDS"
echo "Constants: ${#CONSTANTS[@]}"
echo "Total runs: $((TOTAL_CANDS * ${#CONSTANTS[@]}))"
echo "Max-K: 7 (quick screen)"
echo "============================================="

TOTAL_HITS=0
SHEFFER_COUNT=0
PARTIAL_10PLUS=0
COUNT=0
BEST_SCORE=0
BEST_OP=""
BEST_CONST=""

# Create summary CSV
echo "operator,constant,found,total,elapsed_ms" > "$RESULTS/verify_L1_results.csv"

while IFS= read -r SPEC; do
    OP_NAME=$(echo "$SPEC" | cut -d: -f1)

    for CONST in "${CONSTANTS[@]}"; do
        COUNT=$((COUNT + 1))

        # Run verification
        OUTPUT=$("$VERIFY" --constants "$CONST" --functions '' \
            --custom-op "$SPEC" \
            --operations "$OP_NAME" \
            --max-k 7 2>&1)

        # Parse remaining counts from last occurrence
        RC=$(echo "$OUTPUT" | grep "^Remaining constants:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RU=$(echo "$OUTPUT" | grep "^Remaining unary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RB=$(echo "$OUTPUT" | grep "^Remaining binary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        REMAINING=$((RC + RU + RB))
        FOUND=$((35 - REMAINING))
        ELAPSED=$(echo "$OUTPUT" | grep "^Elapsed:" | awk '{print $2}')

        echo "$OP_NAME,$CONST,$FOUND,35,$ELAPSED" >> "$RESULTS/verify_L1_results.csv"

        if [ "$FOUND" -ge 10 ]; then
            echo "[$COUNT/$((TOTAL_CANDS * ${#CONSTANTS[@]}))] HIGH: $OP_NAME + const=$CONST -> $FOUND/35 ($ELAPSED)"
            echo "$OUTPUT" > "$RESULTS/partial_hits/${OP_NAME}_${CONST}.log"
            PARTIAL_10PLUS=$((PARTIAL_10PLUS + 1))
        fi

        if [ "$REMAINING" -eq 0 ]; then
            echo "*** SHEFFER: $OP_NAME + const=$CONST -> 35/35 ***"
            echo "$OUTPUT" > "$RESULTS/hits/${OP_NAME}_${CONST}.log"
            SHEFFER_COUNT=$((SHEFFER_COUNT + 1))
        fi

        if [ "$FOUND" -gt "$BEST_SCORE" ]; then
            BEST_SCORE=$FOUND
            BEST_OP=$OP_NAME
            BEST_CONST=$CONST
        fi

        # Progress every 500 runs
        if [ $((COUNT % 500)) -eq 0 ]; then
            echo "  Progress: $COUNT/$((TOTAL_CANDS * ${#CONSTANTS[@]})) | Sheffer: $SHEFFER_COUNT | 10+: $PARTIAL_10PLUS"
        fi
    done
done <<< "$CANDIDATES"

echo ""
echo "============================================="
echo "Level 1 Full Search Complete"
echo "Total runs: $COUNT"
echo "Sheffer candidates: $SHEFFER_COUNT"
echo "Partial hits (10+/35): $PARTIAL_10PLUS"
echo "Best: $BEST_OP + const=$BEST_CONST ($BEST_SCORE/35)"
echo "Results: $RESULTS/verify_L1_results.csv"
