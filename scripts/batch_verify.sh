#!/bin/bash
# Batch verification of Level 1 candidate operators
# Each candidate is tested with multiple companion constants
# Quick screen: max-k=7

VERIFY="D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
RESULTS="D:/CLAUDE/EML_IAE/project-iceberg/results"
mkdir -p "$RESULTS/verify_log"

CONSTANTS=("1" "E" "Pi" "-1" "2" "0")

# List of candidate operators registered in binary_catalog
CANDIDATES=(
    "Plus_Exp_Exp"
    "Times_Exp_Exp"
    "Sub_Exp_Exp"
    "Div_Exp_Exp"
    "Pow_Exp_Exp"
    "Plus_Exp_Log"
    "Times_Exp_Log"
    "Pow_Exp_Log"
    "LogBase_Exp_Log"
    "Plus_Log_Exp"
    "Times_Log_Exp"
    "Div_Log_Exp"
    "Pow_Log_Exp"
    "Plus_Log_Log"
    "Sub_Log_Log"
    "Times_Log_Log"
    "Div_Log_Log"
    "Pow_Log_Log"
    "Sub_Exp_Sin"
    "Div_Exp_Sin"
    "Sub_Exp_Cos"
    "Sub_Exp_Inv"
    "Div_Exp_Inv"
    "Sub_Exp_Sqrt"
    "Sub_Exp_Neg"
    "Sub_Log_Sin"
    "Sub_Log_Cos"
    "Sub_Exp_Sqr"
    "Pow_Exp_Id"
    "Pow_Id_Exp"
    "Pow_Log_Id"
    "Pow_Id_Log"
    "Sub_Exp_Sinh"
    "Sub_Exp_Cosh"
    "Div_Exp_Sinh"
    "Sub_Exp_Tanh"
)

echo "Starting batch verification of ${#CANDIDATES[@]} candidates with ${#CONSTANTS[@]} constants each"
echo "Total runs: $((${#CANDIDATES[@]} * ${#CONSTANTS[@]}))"
echo "============================================="

TOTAL_HITS=0
BEST_SCORE=0
BEST_OP=""
BEST_CONST=""

for OP in "${CANDIDATES[@]}"; do
    for CONST in "${CONSTANTS[@]}"; do
        # Run with max-k=7 for quick screening
        OUTPUT=$("$VERIFY" --constants "$CONST" --functions '' --operations "$OP" --max-k 7 2>&1)

        # Count remaining items
        RC=$(echo "$OUTPUT" | grep "^Remaining constants:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RU=$(echo "$OUTPUT" | grep "^Remaining unary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RB=$(echo "$OUTPUT" | grep "^Remaining binary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        REMAINING=$((RC + RU + RB))
        FOUND=$((35 - REMAINING))

        if [ "$FOUND" -gt 0 ]; then
            ELAPSED=$(echo "$OUTPUT" | grep "^Elapsed:" | awk '{print $2}')
            echo "HIT: $OP + const=$CONST -> found=$FOUND/35 remaining=$REMAINING ($Elapsed)"

            # Save detailed log
            echo "$OUTPUT" > "$RESULTS/verify_log/${OP}_${CONST}.log"

            TOTAL_HITS=$((TOTAL_HITS + 1))
            if [ "$FOUND" -gt "$BEST_SCORE" ]; then
                BEST_SCORE=$FOUND
                BEST_OP=$OP
                BEST_CONST=$CONST
            fi

            # If sheffer (found all 35), save to hits/
            if [ "$REMAINING" -eq 0 ]; then
                echo "*** SHEFFER CANDIDATE FOUND: $OP + const=$CONST ***"
                mkdir -p "$RESULTS/hits"
                echo "$OUTPUT" > "$RESULTS/hits/${OP}_${CONST}.log"
            fi
        fi
    done
done

echo ""
echo "============================================="
echo "Batch complete. Total hits (found>0): $TOTAL_HITS"
echo "Best candidate: $BEST_OP + const=$BEST_CONST (found=$BEST_SCORE/35)"
