"""
4-operator cost model + Pareto frontier chart.
EML, EDL, DivLogExp, PowerLogInv.
"""
import os
import csv
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

# ============================================================
# Cost model
# ============================================================

# Node cost: what operations does each operator perform per evaluation?
# EML(a,b) = exp(a) - ln(b)       -> 1 exp + 1 ln + 1 sub
# EDL(a,b) = exp(a) / ln(b)       -> 1 exp + 1 ln + 1 div
# DivLogExp(a,b) = ln(a) / exp(b) -> 1 ln + 1 exp + 1 div
# PowerLogInv(a,b) = ln(a)^(1/b)  -> 1 ln + 1 div(for 1/b) + 1 pow

COST_MODELS = {
    "x86_cpu":  {"add": 1, "sub": 1, "mul": 1, "div": 5, "exp": 10, "ln": 12, "pow": 15},
    "fpga":     {"add": 1, "sub": 1, "mul": 3, "div": 15, "exp": 5,  "ln": 5,  "pow": 20},
    "analog":   {"add": 1, "sub": 1, "mul": 2, "div": 3,  "exp": 1,  "ln": 1,  "pow": 2},
}

def node_cost(op: str, model: str) -> int:
    m = COST_MODELS[model]
    if op == "EML":        return m["exp"] + m["ln"] + m["sub"]
    if op == "EDL":        return m["exp"] + m["ln"] + m["div"]
    if op == "DivLogExp":  return m["ln"] + m["exp"] + m["div"]
    if op == "PowerLogInv": return m["ln"] + m["div"] + m["pow"]  # ln(a), 1/b, pow
    return 0

# K values from bootstrapping chains (Section 4A of report)
K_VALUES = {
    # func: {operator: K}
    "exp":  {"EML": 3, "EDL": 3, "DivLogExp": 4, "PowerLogInv": 6},
    "ln":   {"EML": 6, "EDL": 7, "DivLogExp": 5, "PowerLogInv": 3},
    "neg":  {"EML": 4, "EDL": 4, "DivLogExp": 4, "PowerLogInv": 4},
    "inv":  {"EML": 4, "EDL": 4, "DivLogExp": 4, "PowerLogInv": 5},
    "plus": {"EML": 4, "EDL": 4, "DivLogExp": 4, "PowerLogInv": 4},
    "sub":  {"EML": 5, "EDL": 5, "DivLogExp": 6, "PowerLogInv": 6},
    "times":{"EML": 6, "EDL": 6, "DivLogExp": 4, "PowerLogInv": 5},
    "div":  {"EML": 4, "EDL": 4, "DivLogExp": 5, "PowerLogInv": 4},
}

OPERATORS = ["EML", "EDL", "DivLogExp", "PowerLogInv"]
CORE_FUNCS = ["exp", "ln", "neg", "inv", "plus", "sub", "times", "div"]


def compute_cost_scores():
    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results', 'cost')
    os.makedirs(results_dir, exist_ok=True)

    print("=== 4-Operator Cost Analysis ===\n")

    for model in ["x86_cpu", "fpga", "analog"]:
        print(f"--- {model} ---")
        nc = {op: node_cost(op, model) for op in OPERATORS}
        print(f"Node costs: {nc}\n")

        header = f"{'func':<8}"
        for op in OPERATORS:
            header += f" {op+'_K':<6} {op+'_cost':<8}"
        print(header)

        scores = {op: 0 for op in OPERATORS}
        for func in CORE_FUNCS:
            row = f"{func:<8}"
            for op in OPERATORS:
                k = K_VALUES[func][op]
                approx_ops = max(1, (k - 1) // 2)
                cost = approx_ops * nc[op]
                scores[op] += cost
                row += f" {k:<6} {cost:<8}"
            print(row)

        print(f"\ncost_score: ", end="")
        for op in OPERATORS:
            print(f"{op}={scores[op]}  ", end="")
        print("\n")

    # Save CSV
    csv_path = os.path.join(results_dir, 'cost_four_ops.csv')
    with open(csv_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['function', 'operator', 'K', 'scenario', 'node_cost', 'approx_ops', 'total_cost'])
        for func in CORE_FUNCS:
            for op in OPERATORS:
                k = K_VALUES[func][op]
                ops = max(1, (k - 1) // 2)
                for model in COST_MODELS:
                    nc = node_cost(op, model)
                    writer.writerow([func, op, k, model, nc, ops, ops * nc])
    print(f"Saved: {csv_path}")

    # Return x86 scores for Pareto
    nc_x86 = {op: node_cost(op, "x86_cpu") for op in OPERATORS}
    scores_x86 = {}
    for op in OPERATORS:
        scores_x86[op] = sum(
            max(1, (K_VALUES[f][op] - 1) // 2) * nc_x86[op]
            for f in CORE_FUNCS
        )
    return scores_x86


def make_pareto_chart(cost_scores: dict):
    """Create Pareto frontier chart with 4 operators."""
    # Stability data from stability_four_ops.py
    mean_rel_err = {
        "EML": 3.48e-10,
        "EDL": 9.19e-09,
        "DivLogExp": 8.71e-09,
        "PowerLogInv": 1.51e-08,
    }

    results_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'results', 'cost')

    fig, ax = plt.subplots(1, 1, figsize=(8, 6))

    markers = {"EML": ("o", "blue"), "EDL": ("^", "red"),
               "DivLogExp": ("s", "green"), "PowerLogInv": ("D", "purple")}

    xs, ys, labels = [], [], []
    for op in OPERATORS:
        x = cost_scores[op]
        y = 1.0 / mean_rel_err[op]
        xs.append(x)
        ys.append(y)
        labels.append(op)
        m, c = markers[op]
        ax.scatter(x, y, marker=m, c=c, s=120, zorder=5, label=op)
        ax.annotate(op, (x, y), textcoords="offset points", xytext=(8, 8), fontsize=10)

    # Find Pareto-optimal points (lower cost AND higher stability)
    pareto = []
    sorted_pts = sorted(zip(xs, ys, labels), key=lambda p: p[0])
    best_y = -1
    for x, y, lbl in sorted_pts:
        if y > best_y:
            pareto.append((x, y, lbl))
            best_y = y

    # Draw Pareto frontier
    if len(pareto) >= 1:
        px = [p[0] for p in pareto]
        py = [p[1] for p in pareto]
        # Extend staircase
        stair_x, stair_y = [px[0]], [py[0]]
        for i in range(1, len(pareto)):
            stair_x.extend([px[i], px[i]])
            stair_y.extend([py[i-1], py[i]])
        ax.plot(stair_x, stair_y, 'k--', alpha=0.4, linewidth=1.5)

        # Annotate Pareto
        for px_i, py_i, lbl in pareto:
            ax.annotate("Pareto optimal", (px_i, py_i),
                       textcoords="offset points", xytext=(-20, -25),
                       fontsize=8, color='gray', fontstyle='italic',
                       arrowprops=dict(arrowstyle='->', color='gray', lw=0.8))

    ax.set_xlabel("Cost Score (lower is better)", fontsize=12)
    ax.set_ylabel("Numerical Stability: 1/mean_rel_err (higher is better)", fontsize=12)
    ax.set_title("Pareto Frontier: Cost vs Stability Tradeoff\n(x86 CPU cost model, 729-point benchmark)", fontsize=13)
    ax.set_yscale('log')
    ax.grid(True, alpha=0.3)
    ax.legend(loc='center right', fontsize=10)

    plt.tight_layout()
    chart_path = os.path.join(results_dir, 'pareto_frontier.png')
    fig.savefig(chart_path, dpi=150)
    plt.close()
    print(f"Saved: {chart_path}")


if __name__ == "__main__":
    scores = compute_cost_scores()
    make_pareto_chart(scores)
