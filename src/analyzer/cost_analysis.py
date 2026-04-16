"""
PRD-03: Computational Cost Analysis
"Which variant is cheapest to run?"

Cost model: relative cycle counts for basic operations.
Total cost = tree_size × node_cost.
"""
from __future__ import annotations
import csv
import os
from dataclasses import dataclass


# ============================================================
# Cost model (relative cycles, x86 FPU baseline)
# ============================================================

COST_MODELS = {
    "x86_cpu": {
        "add": 1, "sub": 1, "mul": 1, "div": 5,
        "exp": 10, "ln": 12,
    },
    "fpga_cordic": {
        "add": 1, "sub": 1, "mul": 3, "div": 15,
        "exp": 5, "ln": 5,
    },
    "analog": {
        "add": 1, "sub": 1, "mul": 2, "div": 3,
        "exp": 1, "ln": 1,
    },
}


def node_cost(operator: str, model: str = "x86_cpu") -> int:
    """Cost of a single operator node evaluation."""
    m = COST_MODELS[model]
    if operator == "EML":
        # exp(a) - ln(b): 1 exp + 1 ln + 1 sub
        return m["exp"] + m["ln"] + m["sub"]
    elif operator == "EDL":
        # exp(a) / ln(b): 1 exp + 1 ln + 1 div
        return m["exp"] + m["ln"] + m["div"]
    elif operator == "DivLogExp":
        # ln(a) / exp(b): 1 ln + 1 exp + 1 div
        return m["ln"] + m["exp"] + m["div"]
    elif operator == "NegEML":
        # ln(a) - exp(b): 1 ln + 1 exp + 1 sub
        return m["ln"] + m["exp"] + m["sub"]
    else:
        return 0


# ============================================================
# Tree size data (K values from Rust verification)
# ============================================================

# Witnesses from EML verification (K = RPN code length)
# K includes terminals (constants) and operators
# Node count = number of operator calls = (K-1)/2 for complete binary tree
# But in practice, node count = K - number_of_leaves

EML_WITNESSES = {
    "exp":  ("EML[x, 1]", 3),
    "e":    ("EML[1, 1]", 3),
    "ln":   ("EML[1, Exp[EML[1, x]]]", 6),
    "sub":  ("EML[Log[x], Exp[y]]", 5),
    "-1":   ("Subtract[Log[1], 1]", 4),
    "neg":  ("Subtract[Log[1], x]", 4),
    "2":    ("Subtract[1, -1]", 3),
    "inv":  ("Exp[Minus[Log[x]]]", 4),
    "plus": ("Subtract[x, Minus[y]]", 4),
    "times":("Exp[Plus[Log[x], Log[y]]]", 6),
    "sqr":  ("Times[x, x]", 3),
    "div":  ("Times[x, Inv[y]]", 4),
    "half": ("Divide[x, 2]", 3),
    "sqrt": ("Exp[Half[Log[x]]]", 4),
    "pow":  ("Exp[Times[y, Log[x]]]", 5),
    "pi":   ("Sqrt[Minus[Sqr[Log[-1]]]]", 5),
}

# Witnesses from DivLogExp verification
DLE_WITNESSES = {
    "ln":   ("DivLogExp[x, DivLogExp[1, 1]]", 5),
    "log":  ("DivLogExp[y, Log[Log[x]]]", 5),
    "e":    ("DivLogExp[x, Log[DivLogExp[x, 1]]]", 6),
    "-1":   ("Log[DivLogExp[E, 1]]", 4),
    "neg":  ("Log[DivLogExp[E, x]]", 4),
    "inv":  ("DivLogExp[E, Log[x]]", 4),
    "exp":  ("Inv[DivLogExp[E, x]]", 4),
    "div":  ("DivLogExp[Exp[x], Log[y]]", 5),
    "times":("Divide[x, Inv[y]]", 4),
    "sqr":  ("Times[x, x]", 3),
    "2":    ("Log[Sqr[E]]", 3),
    "half": ("Divide[x, 2]", 3),
    "sqrt": ("Exp[Half[Log[x]]]", 4),
    "pow":  ("Exp[Times[y, Log[x]]]", 5),
    "pi":   ("Sqrt[Minus[Sqr[Log[-1]]]]", 5),
    "sub":  ("Log[DivLogExp[Exp[Exp[x]], y]]", 6),
    "plus": ("Subtract[x, Minus[y]]", 4),
}


@dataclass
class CostEntry:
    function: str
    operator: str
    K: int  # RPN code length
    node_cost_per_call: int
    # For K, the number of operator nodes is roughly (K-1)/2
    # But the actual number depends on the tree structure
    # We approximate: operator calls ≈ K // 2 (each call uses 2 inputs + 1 op)
    estimated_ops: int
    total_cost: int  # estimated_ops × node_cost


def analyze_costs():
    """Compute cost scores for each operator variant."""

    # Core 8 functions for depth_score
    core_functions = ["exp", "ln", "neg", "inv", "plus", "sub", "times", "div"]

    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results', 'cost')
    os.makedirs(results_dir, exist_ok=True)

    print("=== Computational Cost Analysis (PRD-03) ===\n")

    for model_name in ["x86_cpu", "fpga_cordic", "analog"]:
        print(f"\n--- Scenario: {model_name} ---")
        eml_nc = node_cost("EML", model_name)
        dle_nc = node_cost("DivLogExp", model_name)
        edl_nc = node_cost("EDL", model_name)

        print(f"  Node cost: EML={eml_nc}, EDL={edl_nc}, DivLogExp={dle_nc}")

        print(f"\n  {'Function':<10} {'EML_K':<8} {'DLE_K':<8} {'EML_cost':<12} {'DLE_cost':<12} {'Winner':<10}")
        eml_total = 0
        dle_total = 0

        for func in core_functions:
            eml_k = EML_WITNESSES.get(func, (None, 0))[1]
            dle_k = DLE_WITNESSES.get(func, (None, 0))[1]

            # Approximate operator calls from K:
            # In a binary tree with K tokens, internal nodes ≈ (K-1)/2
            eml_ops = max(1, (eml_k - 1) // 2) if eml_k > 0 else 0
            dle_ops = max(1, (dle_k - 1) // 2) if dle_k > 0 else 0

            eml_cost = eml_ops * eml_nc
            dle_cost = dle_ops * dle_nc

            eml_total += eml_cost
            dle_total += dle_cost

            winner = "EML" if eml_cost < dle_cost else "DLE" if dle_cost < eml_cost else "TIE"
            print(f"  {func:<10} {eml_k:<8} {dle_k:<8} {eml_cost:<12} {dle_cost:<12} {winner:<10}")

        print(f"\n  Total cost_score: EML={eml_total}, DivLogExp={dle_total}")
        diff_pct = (dle_total - eml_total) / eml_total * 100 if eml_total > 0 else 0
        if diff_pct > 0:
            print(f"  EML is {diff_pct:.1f}% cheaper than DivLogExp in {model_name}")
        else:
            print(f"  DivLogExp is {-diff_pct:.1f}% cheaper than EML in {model_name}")

    # Save summary
    csv_path = os.path.join(results_dir, 'cost_comparison.csv')
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['function', 'operator', 'K', 'scenario', 'node_cost', 'approx_ops', 'total_cost'])
        for func in core_functions:
            for op, witnesses, op_name in [
                ("EML", EML_WITNESSES, "EML"),
                ("DivLogExp", DLE_WITNESSES, "DivLogExp"),
            ]:
                k = witnesses.get(func, (None, 0))[1]
                ops = max(1, (k - 1) // 2) if k > 0 else 0
                for model_name in COST_MODELS:
                    nc = node_cost(op_name, model_name)
                    writer.writerow([func, op_name, k, model_name, nc, ops, ops * nc])

    print(f"\nSaved to {csv_path}")


if __name__ == "__main__":
    analyze_costs()
