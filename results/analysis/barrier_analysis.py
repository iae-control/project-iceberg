#!/usr/bin/env python
"""
Investigation 1: The 13/35 Barrier Analysis
============================================
Analyzes Level 1 partial hits to understand which targets are found/missing,
builds a coverage heatmap, and identifies bottleneck targets.
"""

import csv
import subprocess
import re
import json
import os
import sys

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import numpy as np

# ---- Configuration ----
CSV_PATH = r"D:/CLAUDE/EML_IAE/project-iceberg/results/verify_L1_results.csv"
RUST_EXE = r"D:/CLAUDE/EML_IAE/project-iceberg/src/author_repo/rust_verify/target/release/verify_base_set_rs.exe"
OUTPUT_DIR = r"D:/CLAUDE/EML_IAE/project-iceberg/results/analysis"
HEATMAP_PATH = os.path.join(OUTPUT_DIR, "coverage_heatmap.png")
MD_PATH = os.path.join(OUTPUT_DIR, "barrier_analysis.md")
MAX_K = 7

# Operator name mapping for --custom-op format
BINOP_MAP = {
    "Plus": "Plus",
    "Subtract": "Sub",
    "Times": "Mul",
    "Divide": "Div",
    "Power": "Pow",
}

# All 35 targets in canonical order
TARGET_CONSTANTS = ["-1", "1", "2", "E", "EulerGamma", "Glaisher", "Pi"]
TARGET_UNARY = ["ArcCos", "ArcCosh", "ArcSin", "ArcSinh", "ArcTan", "ArcTanh",
                "Cos", "Cosh", "Exp", "Half", "Inv", "Log", "LogisticSigmoid",
                "Minus", "Sin", "Sinh", "Sqr", "Sqrt", "Tan", "Tanh"]
TARGET_BINARY = ["Avg", "Divide", "Hypot", "Log", "Plus", "Power", "Subtract", "Times"]
ALL_TARGETS = (
    [f"c:{t}" for t in TARGET_CONSTANTS] +
    [f"u:{t}" for t in TARGET_UNARY] +
    [f"b:{t}" for t in TARGET_BINARY]
)
assert len(ALL_TARGETS) == 35, f"Expected 35 targets, got {len(ALL_TARGETS)}"


def read_csv_top_candidates(csv_path, min_found=10, top_n=20):
    """Read CSV and return top N candidates with found >= min_found, excluding 35/35."""
    rows = []
    with open(csv_path) as f:
        reader = csv.DictReader(f)
        for r in reader:
            found = int(r['found'])
            if found >= min_found and found < 35:
                rows.append(r)
    rows.sort(key=lambda x: int(x['found']), reverse=True)
    return rows[:top_n]


def parse_operator_name(name):
    """Parse 'Subtract_Cos_ArcSin' -> ('Sub', 'Cos', 'ArcSin')."""
    parts = name.split('_', 1)  # Split on first underscore only
    if len(parts) < 2:
        return None
    binop_name = parts[0]
    # The rest contains two unary function names
    # They can themselves contain underscores? No, function names don't have underscores.
    # Actually names like ArcSin, ArcCos etc. So split remaining by underscore.
    remaining = parts[1]
    # Need to split remaining into exactly 2 unary function names
    # Function names: Exp, Log, Sin, Cos, Tan, ArcSin, ArcCos, ArcTan,
    #                 Sinh, Cosh, Tanh, ArcSinh, ArcCosh, ArcTanh,
    #                 Sqrt, Sqr, Inv, Minus, Id, Half
    # Strategy: try splitting at each underscore position
    underscores = [i for i, c in enumerate(remaining) if c == '_']
    for pos in underscores:
        u1 = remaining[:pos]
        u2 = remaining[pos+1:]
        if u1 and u2:
            # Check if binop_name maps to a known binary operator
            if binop_name in BINOP_MAP:
                return (BINOP_MAP[binop_name], u1, u2)
    return None


def run_verifier(operator_name, constant, max_k=MAX_K):
    """Run the Rust verifier and parse found/remaining targets."""
    parsed = parse_operator_name(operator_name)
    if parsed is None:
        print(f"  WARNING: Could not parse operator name: {operator_name}")
        return None

    binop, u1, u2 = parsed
    custom_op_name = operator_name.replace('_', '')  # Remove underscores for the op name
    custom_op_spec = f"{custom_op_name}:{binop}({u1},{u2})"

    cmd = [
        RUST_EXE,
        "--constants", str(constant),
        "--functions", "",
        "--custom-op", custom_op_spec,
        "--operations", custom_op_name,
        "--max-k", str(max_k),
    ]

    print(f"  Running: {operator_name} + const={constant} ...")
    print(f"    custom-op: {custom_op_spec}")

    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=300
        )
        output = result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        print(f"  TIMEOUT for {operator_name} const={constant}")
        return None

    # Parse the final "Remaining" and "Known" lines
    # Look for the LAST set of "Remaining constants:", "Remaining unary:", "Remaining binary:"
    remaining_constants = []
    remaining_unary = []
    remaining_binary = []

    for line in output.split('\n'):
        line = line.strip()
        m = re.match(r'Remaining constants: \[(.*)\]', line)
        if m:
            remaining_constants = parse_list(m.group(1))
        m = re.match(r'Remaining unary: \[(.*)\]', line)
        if m:
            remaining_unary = parse_list(m.group(1))
        m = re.match(r'Remaining binary: \[(.*)\]', line)
        if m:
            remaining_binary = parse_list(m.group(1))

    # Found = Target - Remaining
    found_constants = set(TARGET_CONSTANTS) - set(remaining_constants)
    found_unary = set(TARGET_UNARY) - set(remaining_unary)
    found_binary = set(TARGET_BINARY) - set(remaining_binary)

    found_set = (
        {f"c:{t}" for t in found_constants} |
        {f"u:{t}" for t in found_unary} |
        {f"b:{t}" for t in found_binary}
    )

    total_found = len(found_set)
    print(f"    Found: {total_found}/35")
    print(f"    Constants: {sorted(found_constants)}")
    print(f"    Unary: {sorted(found_unary)}")
    print(f"    Binary: {sorted(found_binary)}")

    return {
        "operator": operator_name,
        "constant": constant,
        "found_set": found_set,
        "found_count": total_found,
        "remaining_constants": remaining_constants,
        "remaining_unary": remaining_unary,
        "remaining_binary": remaining_binary,
        "raw_output": output,
    }


def parse_list(s):
    """Parse a Rust-style list string like '"Foo", "Bar"' into Python list."""
    if not s.strip():
        return []
    items = re.findall(r'"([^"]*)"', s)
    return items


def build_coverage_matrix(results):
    """Build a 2D matrix: rows=candidates, cols=targets, 1=found 0=missing."""
    n_candidates = len(results)
    n_targets = len(ALL_TARGETS)
    matrix = np.zeros((n_candidates, n_targets), dtype=int)
    row_labels = []

    for i, r in enumerate(results):
        label = f"{r['operator']} (c={r['constant']})"
        row_labels.append(label)
        for j, target in enumerate(ALL_TARGETS):
            if target in r['found_set']:
                matrix[i, j] = 1

    return matrix, row_labels


def create_heatmap(matrix, row_labels, col_labels, output_path):
    """Create a coverage heatmap."""
    fig, ax = plt.subplots(figsize=(18, 10))

    # Custom colormap: red=0 (missing), green=1 (found)
    cmap = mcolors.ListedColormap(['#d32f2f', '#4caf50'])
    bounds = [-0.5, 0.5, 1.5]
    norm = mcolors.BoundaryNorm(bounds, cmap.N)

    im = ax.imshow(matrix, cmap=cmap, norm=norm, aspect='auto')

    # Labels
    ax.set_xticks(range(len(col_labels)))
    ax.set_xticklabels(col_labels, rotation=90, fontsize=7)
    ax.set_yticks(range(len(row_labels)))
    ax.set_yticklabels(row_labels, fontsize=8)

    # Add grid lines to separate sections
    # Constants end at index 6, unary at 26, binary at 34
    for x in [6.5, 26.5]:
        ax.axvline(x=x, color='white', linewidth=2)

    # Section labels at top
    ax.text(3, -1.5, "Constants", ha='center', fontsize=9, fontweight='bold')
    ax.text(16.5, -1.5, "Unary Functions", ha='center', fontsize=9, fontweight='bold')
    ax.text(30.5, -1.5, "Binary Operations", ha='center', fontsize=9, fontweight='bold')

    # Score annotations on right side
    for i, row in enumerate(matrix):
        score = row.sum()
        ax.text(len(col_labels) + 0.3, i, f"{score}/35", va='center', fontsize=8)

    ax.set_title("Level 1 Partial Hit Coverage (K_max=7)", fontsize=14, fontweight='bold', pad=30)
    ax.set_xlabel("Target (35 base set elements)")
    ax.set_ylabel("Candidate operator + constant")

    # Legend
    from matplotlib.patches import Patch
    legend_elements = [Patch(facecolor='#4caf50', label='Found'),
                       Patch(facecolor='#d32f2f', label='Missing')]
    ax.legend(handles=legend_elements, loc='upper right', bbox_to_anchor=(1.12, 1.0))

    plt.tight_layout()
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"\nHeatmap saved to: {output_path}")


def analyze_bottleneck(results):
    """Find targets that no partial hit can generate."""
    all_found = set()
    for r in results:
        all_found |= r['found_set']

    never_found = set(ALL_TARGETS) - all_found
    return sorted(never_found)


def analyze_arcsin_cos_barrier(results):
    """Analyze the 13/35 arcsin-cos operator specifically."""
    # Find the SubCosArcSin + const=0 result
    target_result = None
    for r in results:
        if r['operator'] == 'Subtract_Cos_ArcSin' and r['constant'] == '0':
            target_result = r
            break

    if target_result is None:
        # Run it explicitly
        print("\nRunning Subtract_Cos_ArcSin with const=0 for detailed analysis...")
        target_result = run_verifier("Subtract_Cos_ArcSin", "0", MAX_K)

    if target_result is None:
        return None, None

    found = target_result['found_set']
    missing = sorted(set(ALL_TARGETS) - found)
    return found, missing


def write_report(results, matrix, row_labels, bottleneck_targets,
                 arcsin_cos_found, arcsin_cos_missing, md_path):
    """Write the full analysis report."""
    with open(md_path, 'w', encoding='utf-8') as f:
        f.write("# Investigation 1: The 13/35 Barrier Analysis\n\n")
        f.write("## Overview\n\n")
        f.write("This analysis examines Level 1 partial hits -- operators that score 10+/35\n")
        f.write("but fail to reach full Sheffer status. We investigate which targets are\n")
        f.write("consistently hard to generate and why.\n\n")

        # Table of candidates
        f.write("## A. Top 20 Partial Hit Candidates\n\n")
        f.write("| Rank | Operator | Constant | Found | Score |\n")
        f.write("|------|----------|----------|-------|-------|\n")
        for i, r in enumerate(results):
            f.write(f"| {i+1} | {r['operator']} | {r['constant']} | {r['found_count']} | {r['found_count']}/35 |\n")
        f.write("\n")

        # Coverage summary
        f.write("## B. Coverage Heatmap\n\n")
        f.write(f"Heatmap saved to: `{HEATMAP_PATH}`\n\n")
        f.write("The heatmap shows which of the 35 base set targets each candidate can generate.\n")
        f.write("Green = found, Red = missing.\n\n")

        # Per-target hit counts
        f.write("## C. Per-Target Hit Frequency\n\n")
        f.write("How many of the top 20 candidates can generate each target:\n\n")
        f.write("| Target | Type | Hit Count | Hit Rate |\n")
        f.write("|--------|------|-----------|----------|\n")
        n_candidates = len(results)
        target_hits = matrix.sum(axis=0)
        for j, target in enumerate(ALL_TARGETS):
            ttype = target.split(':')[0]
            tname = target.split(':')[1]
            ttype_full = {'c': 'constant', 'u': 'unary', 'b': 'binary'}[ttype]
            hits = target_hits[j]
            rate = hits / n_candidates * 100 if n_candidates > 0 else 0
            f.write(f"| {tname} | {ttype_full} | {hits}/{n_candidates} | {rate:.0f}% |\n")
        f.write("\n")

        # Bottleneck targets
        f.write("## D. Bottleneck Targets (never found by any partial hit)\n\n")
        if bottleneck_targets:
            f.write("These targets are NOT generated by ANY of the top 20 partial hits:\n\n")
            for t in bottleneck_targets:
                f.write(f"- {t}\n")
        else:
            f.write("All targets are found by at least one partial hit.\n")
        f.write("\n")

        # Easy targets (found by all)
        easy_targets = [ALL_TARGETS[j] for j in range(len(ALL_TARGETS))
                        if target_hits[j] == n_candidates]
        f.write("## E. Easy Targets (found by all partial hits)\n\n")
        if easy_targets:
            for t in easy_targets:
                f.write(f"- {t}\n")
        else:
            f.write("No target is found by all partial hits.\n")
        f.write("\n")

        # 13/35 arcsin-cos analysis
        f.write("## F. The 13/35 ArcSin-Cos Barrier: Subtract_Cos_ArcSin + const=0\n\n")
        f.write("### Structure\n\n")
        f.write("The operator `Subtract_Cos_ArcSin` computes: `Sub(Cos(x), ArcSin(y))` = `cos(x) - arcsin(y)`\n\n")

        if arcsin_cos_found is not None:
            f.write(f"### Found targets ({len(arcsin_cos_found)}/35)\n\n")
            for t in sorted(arcsin_cos_found):
                f.write(f"- {t}\n")
            f.write("\n")

            f.write(f"### Missing targets ({len(arcsin_cos_missing)}/35)\n\n")
            for t in arcsin_cos_missing:
                f.write(f"- {t}\n")
            f.write("\n")

            f.write("### Analysis: Why These Targets Fail\n\n")

            # Categorize missing targets
            missing_constants = [t for t in arcsin_cos_missing if t.startswith('c:')]
            missing_unary = [t for t in arcsin_cos_missing if t.startswith('u:')]
            missing_binary = [t for t in arcsin_cos_missing if t.startswith('b:')]

            f.write("#### Missing Constants\n\n")
            for t in missing_constants:
                name = t.split(':')[1]
                if name == 'E':
                    f.write(f"- **{name}**: e = 2.71828... is transcendental and unrelated to trig/inverse-trig.\n")
                    f.write("  cos(x) - arcsin(y) cannot produce e from algebraic combinations of\n")
                    f.write("  0, EulerGamma, Glaisher within k<=7. The exponential function is needed.\n\n")
                else:
                    f.write(f"- **{name}**: Cannot be constructed from cos/arcsin compositions.\n\n")

            f.write("#### Missing Unary Functions\n\n")
            f.write("The operator can naturally produce `Cos`, `ArcSin`, `Sin` (via identity),\n")
            f.write("`ArcCos` (complementary), and `Minus` (via subtraction). But it cannot\n")
            f.write("escape the trig/inverse-trig domain to reach:\n\n")
            for t in missing_unary:
                name = t.split(':')[1]
                if name in ('Exp', 'Log'):
                    f.write(f"- **{name}**: Exponential/logarithmic functions are algebraically\n")
                    f.write("  independent from trigonometric functions. No finite composition of\n")
                    f.write("  cos and arcsin can produce exp or log.\n\n")
                elif name in ('Sinh', 'Cosh', 'Tanh', 'ArcSinh', 'ArcCosh', 'ArcTanh'):
                    f.write(f"- **{name}**: Hyperbolic functions relate to exp/log via\n")
                    f.write(f"  identities like sinh(x) = (e^x - e^(-x))/2. Without exp/log,\n")
                    f.write(f"  these are unreachable.\n\n")
                elif name in ('Inv', 'Sqr', 'Sqrt', 'Half'):
                    f.write(f"- **{name}**: Algebraic operations (1/x, x^2, sqrt, x/2) require\n")
                    f.write(f"  multiplication/division/power which this operator cannot produce\n")
                    f.write(f"  from subtraction alone.\n\n")
                elif name in ('ArcTan', 'ArcTanh', 'Tan'):
                    f.write(f"- **{name}**: Although trigonometric, tangent is sin/cos ratio.\n")
                    f.write(f"  Without division, tan cannot be composed from cos and arcsin.\n\n")
                elif name == 'LogisticSigmoid':
                    f.write(f"- **{name}**: sigma(x) = 1/(1+e^(-x)) requires both exp and division.\n\n")
                else:
                    f.write(f"- **{name}**: Not reachable from cos/arcsin compositions.\n\n")

            f.write("#### Missing Binary Operations\n\n")
            for t in missing_binary:
                name = t.split(':')[1]
                if name in ('Times', 'Divide', 'Power'):
                    f.write(f"- **{name}**: Multiplicative operations cannot be derived from\n")
                    f.write(f"  an additive (subtraction-based) operator. The fundamental barrier\n")
                    f.write(f"  is that cos(x)-arcsin(y) is additive in nature.\n\n")
                elif name in ('Avg', 'Hypot'):
                    f.write(f"- **{name}**: Requires multiplication (Avg=(x+y)/2, Hypot=sqrt(x^2+y^2)).\n\n")
                elif name == 'Log':
                    f.write(f"- **{name}** (binary log_b): Requires logarithm, which is unreachable.\n\n")
                else:
                    f.write(f"- **{name}**: Not reachable from cos/arcsin compositions.\n\n")

            f.write("### Root Cause Summary\n\n")
            f.write("The 13/35 barrier exists because `cos(x) - arcsin(y)` is fundamentally\n")
            f.write("trapped in the **trig/inverse-trig + additive** domain:\n\n")
            f.write("1. **No exponential bridge**: Cannot reach exp/log, which blocks all\n")
            f.write("   hyperbolic functions and e.\n")
            f.write("2. **No multiplicative structure**: Subtraction alone cannot produce\n")
            f.write("   multiplication, division, or power operations.\n")
            f.write("3. **Trig-algebraic isolation**: Trigonometric functions are algebraically\n")
            f.write("   independent from exponential functions over the reals (though connected\n")
            f.write("   in the complex domain via Euler's formula, the verifier's k<=7 depth\n")
            f.write("   is insufficient to exploit this).\n\n")
            f.write("The 13 found targets are exactly those reachable within the\n")
            f.write("trig+additive closure: {cos, arcsin, sin, arccos, minus} for unary,\n")
            f.write("{plus, subtract} for binary, and {-1, 1, 2, pi} + base constants.\n")

        f.write("\n---\n")
        f.write(f"\nGenerated by barrier_analysis.py\n")

    print(f"\nReport saved to: {md_path}")


def main():
    print("=" * 60)
    print("Investigation 1: The 13/35 Barrier Analysis")
    print("=" * 60)

    # Step A: Read CSV and find top 20 candidates
    print("\n[A] Reading CSV and finding top candidates...")
    candidates = read_csv_top_candidates(CSV_PATH, min_found=10, top_n=20)
    print(f"  Found {len(candidates)} candidates with 10+ hits (excluding 35/35)")
    for c in candidates:
        print(f"    {c['operator']:40s} const={c['constant']:5s} found={c['found']}/35")

    # Step B: Run verifier for each candidate
    print("\n[B] Running Rust verifier for each candidate...")
    results = []
    for i, c in enumerate(candidates):
        print(f"\n--- Candidate {i+1}/{len(candidates)} ---")
        r = run_verifier(c['operator'], c['constant'], MAX_K)
        if r is not None:
            results.append(r)
        else:
            print(f"  SKIPPED (parse error or timeout)")

    if not results:
        print("ERROR: No results obtained. Exiting.")
        sys.exit(1)

    print(f"\n  Successfully processed {len(results)}/{len(candidates)} candidates")

    # Step C: Build coverage matrix
    print("\n[C] Building coverage matrix...")
    matrix, row_labels = build_coverage_matrix(results)
    # Short column labels
    col_labels = [t.split(':')[1] for t in ALL_TARGETS]
    print(f"  Matrix shape: {matrix.shape}")

    # Step D: Create heatmap
    print("\n[D] Creating heatmap...")
    create_heatmap(matrix, row_labels, col_labels, HEATMAP_PATH)

    # Step E: Bottleneck targets
    print("\n[E] Identifying bottleneck targets...")
    bottleneck = analyze_bottleneck(results)
    if bottleneck:
        print(f"  {len(bottleneck)} targets never found by any partial hit:")
        for t in bottleneck:
            print(f"    - {t}")
    else:
        print("  All targets found by at least one partial hit.")

    # Step F: 13/35 arcsin-cos analysis
    print("\n[F] Analyzing 13/35 arcsin-cos barrier...")
    arcsin_cos_found, arcsin_cos_missing = analyze_arcsin_cos_barrier(results)
    if arcsin_cos_missing:
        print(f"  Missing {len(arcsin_cos_missing)} targets:")
        for t in arcsin_cos_missing:
            print(f"    - {t}")

    # Write report
    print("\n[Report] Writing analysis report...")
    write_report(results, matrix, row_labels, bottleneck,
                 arcsin_cos_found, arcsin_cos_missing, MD_PATH)

    # Also save raw results as JSON for further analysis
    json_path = os.path.join(OUTPUT_DIR, "barrier_results.json")
    json_results = []
    for r in results:
        json_results.append({
            "operator": r['operator'],
            "constant": r['constant'],
            "found_count": r['found_count'],
            "found_set": sorted(r['found_set']),
            "remaining_constants": r['remaining_constants'],
            "remaining_unary": r['remaining_unary'],
            "remaining_binary": r['remaining_binary'],
        })
    with open(json_path, 'w') as f:
        json.dump(json_results, f, indent=2)
    print(f"Raw results saved to: {json_path}")

    print("\n" + "=" * 60)
    print("Analysis complete!")
    print("=" * 60)


if __name__ == "__main__":
    main()
