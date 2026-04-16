#!/bin/bash
# Priority batch: Test the most promising Level 1 candidates first
# Focus on operators involving exp/log combinations that aren't trivially EML-equivalent

VERIFY="D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
RESULTS="D:/CLAUDE/EML_IAE/project-iceberg/results"
mkdir -p "$RESULTS/hits" "$RESULTS/partial_hits"

CONSTANTS=("1" "E" "Pi" "-1" "2" "0" "I")

# Priority candidates: operators with good structural properties for Sheffer-ness
# Key insight: EML works because exp and log are inverse pairs
# We test operators built from exp, log, and their compositions
CANDIDATES=(
    # exp-log with different binary ops (EML family variations)
    "PlusExpLog:Plus(Exp,Log)"
    "TimesExpLog:Mul(Exp,Log)"
    "PowExpLog:Pow(Exp,Log)"
    "LogBaseExpLog:LogBase(Exp,Log)"
    # log-exp reversed
    "PlusLogExp:Plus(Log,Exp)"
    "TimesLogExp:Mul(Log,Exp)"
    "DivLogExp:Div(Log,Exp)"
    "PowLogExp:Pow(Log,Exp)"
    "LogBaseLogExp:LogBase(Log,Exp)"
    # log-log (logarithmic operators)
    "PlusLogLog:Plus(Log,Log)"
    "SubLogLog:Sub(Log,Log)"
    "TimesLogLog:Mul(Log,Log)"
    "DivLogLog:Div(Log,Log)"
    "PowLogLog:Pow(Log,Log)"
    "LogBaseLogLog:LogBase(Log,Log)"
    # exp-exp (exponential operators)
    "SubExpExp:Sub(Exp,Exp)"
    "DivExpExp:Div(Exp,Exp)"
    "PowExpExp:Pow(Exp,Exp)"
    # exp with inverse functions
    "SubExpInv:Sub(Exp,Inv)"
    "DivExpInv:Div(Exp,Inv)"
    "SubExpMinus:Sub(Exp,Minus)"
    "SubExpSqr:Sub(Exp,Sqr)"
    "SubExpSqrt:Sub(Exp,Sqrt)"
    # exp with trig
    "SubExpSin:Sub(Exp,Sin)"
    "DivExpSin:Div(Exp,Sin)"
    "SubExpCos:Sub(Exp,Cos)"
    "DivExpCos:Div(Exp,Cos)"
    "SubExpTan:Sub(Exp,Tan)"
    "DivExpTan:Div(Exp,Tan)"
    # exp with hyperbolic
    "SubExpSinh:Sub(Exp,Sinh)"
    "DivExpSinh:Div(Exp,Sinh)"
    "SubExpCosh:Sub(Exp,Cosh)"
    "SubExpTanh:Sub(Exp,Tanh)"
    # log with trig
    "SubLogSin:Sub(Log,Sin)"
    "DivLogSin:Div(Log,Sin)"
    "SubLogCos:Sub(Log,Cos)"
    # Power-based structural operators
    "PowExpId:Pow(Exp,Id)"
    "PowIdExp:Pow(Id,Exp)"
    "PowLogId:Pow(Log,Id)"
    "PowIdLog:Pow(Id,Log)"
    # Inverse trig combinations
    "SubExpArcSin:Sub(Exp,ArcSin)"
    "SubExpArcTan:Sub(Exp,ArcTan)"
    "DivExpArcSin:Div(Exp,ArcSin)"
    "DivExpArcTan:Div(Exp,ArcTan)"
    # ArcSinh/ArcCosh (related to log)
    "SubExpArcSinh:Sub(Exp,ArcSinh)"
    "SubExpArcCosh:Sub(Exp,ArcCosh)"
    "DivExpArcSinh:Div(Exp,ArcSinh)"
    # Log with inverse trig
    "SubLogArcSin:Sub(Log,ArcSin)"
    "SubLogArcTan:Sub(Log,ArcTan)"
    # Sigmoid combinations
    "SubExpSigmoid:Sub(Exp,LogisticSigmoid)"
    "DivExpSigmoid:Div(Exp,LogisticSigmoid)"
    "SubLogSigmoid:Sub(Log,LogisticSigmoid)"
)

echo "=== Priority Batch: ${#CANDIDATES[@]} operators × ${#CONSTANTS[@]} constants ==="
echo "Total runs: $((${#CANDIDATES[@]} * ${#CONSTANTS[@]}))"

SHEFFER_COUNT=0
PARTIAL_10PLUS=0
COUNT=0

for SPEC in "${CANDIDATES[@]}"; do
    OP_NAME=$(echo "$SPEC" | cut -d: -f1)
    OP_DEF=$(echo "$SPEC" | cut -d: -f2)

    for CONST in "${CONSTANTS[@]}"; do
        COUNT=$((COUNT + 1))

        OUTPUT=$("$VERIFY" --constants "$CONST" --functions '' \
            --custom-op "$SPEC" \
            --operations "$OP_NAME" \
            --max-k 7 2>&1)

        RC=$(echo "$OUTPUT" | grep "^Remaining constants:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RU=$(echo "$OUTPUT" | grep "^Remaining unary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        RB=$(echo "$OUTPUT" | grep "^Remaining binary:" | tail -1 | grep -o '"[^"]*"' | wc -l)
        REMAINING=$((RC + RU + RB))
        FOUND=$((35 - REMAINING))
        ELAPSED=$(echo "$OUTPUT" | grep "^Elapsed:" | awk '{print $2}')

        if [ "$FOUND" -ge 8 ]; then
            echo "  [$COUNT] $OP_NAME + $CONST -> $FOUND/35 ($ELAPSED)"
        fi

        if [ "$FOUND" -ge 10 ]; then
            echo "$OUTPUT" > "$RESULTS/partial_hits/${OP_NAME}_${CONST}.log"
            PARTIAL_10PLUS=$((PARTIAL_10PLUS + 1))
        fi

        if [ "$REMAINING" -eq 0 ]; then
            echo "*** SHEFFER: $OP_NAME + const=$CONST -> 35/35! ***"
            echo "$OUTPUT" > "$RESULTS/hits/${OP_NAME}_${CONST}.log"
            SHEFFER_COUNT=$((SHEFFER_COUNT + 1))
        fi
    done
done

echo ""
echo "============================================="
echo "Priority batch complete. Runs: $COUNT"
echo "Sheffer candidates: $SHEFFER_COUNT"
echo "Partial hits (10+/35): $PARTIAL_10PLUS"
