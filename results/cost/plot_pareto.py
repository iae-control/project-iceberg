import os
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

# Data
operators = ['EML', 'EDL', 'DivLogExp']
cost_scores = [253, 297, 297]
mean_rel_errs = [3.48e-10, 9.19e-09, 8.71e-09]
stability = [1.0 / e for e in mean_rel_errs]

# Styles
colors  = ['blue',  'red',       'green']
markers = ['o',     '^',         's']
labels  = ['EML',   'EDL',       'DivLogExp']

# ---- Pareto frontier (minimise cost, maximise stability) ---------------
# Sort by cost ascending; keep a point if its stability >= all previous best
points = sorted(zip(cost_scores, stability, operators, colors, markers, labels),
                key=lambda x: (x[0], -x[1]))

pareto = []
best_stability = -np.inf
for pt in points:
    if pt[1] >= best_stability:
        pareto.append(pt)
        best_stability = pt[1]

# ---- Plot ---------------------------------------------------------------
fig, ax = plt.subplots(figsize=(8, 6))

# All points
for c, s, op, col, mk, lbl in zip(cost_scores, stability, operators, colors, markers, labels):
    ax.scatter(c, s, color=col, marker=mk, s=120, zorder=5, label=lbl)
    ax.annotate(op, (c, s), textcoords='offset points', xytext=(8, 4),
                fontsize=10, color=col)

# Pareto frontier line
px = [pt[0] for pt in pareto]
py = [pt[1] for pt in pareto]
# Extend with a step-style staircase so the frontier looks correct
# (lower cost → higher stability staircase)
step_x = []
step_y = []
for i, (x, y) in enumerate(zip(px, py)):
    step_x.append(x)
    step_y.append(y)
    if i < len(px) - 1:
        # horizontal segment at current stability level up to next cost
        step_x.append(px[i + 1])
        step_y.append(y)

ax.plot(step_x, step_y, 'k--', linewidth=1.5, label='Pareto frontier', zorder=3)

# Annotation "Pareto optimal" near the frontier
mid_idx = len(pareto) // 2
ax.annotate('Pareto optimal',
            xy=(pareto[mid_idx][0], pareto[mid_idx][1]),
            xytext=(pareto[mid_idx][0] - 30, pareto[mid_idx][1] * 0.6),
            fontsize=9, color='black',
            arrowprops=dict(arrowstyle='->', color='black', lw=1.2))

# Formatting
ax.set_xlabel('Cost Score (lower is better)', fontsize=12)
ax.set_ylabel('Numerical Stability (higher is better)', fontsize=12)
ax.set_title('Pareto Frontier: Cost vs Stability Tradeoff', fontsize=13, fontweight='bold')
ax.legend(loc='upper right', fontsize=10)
ax.grid(True, linestyle='--', alpha=0.6)

# Use scientific notation on y-axis
ax.yaxis.set_major_formatter(matplotlib.ticker.ScalarFormatter(useMathText=True))
ax.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))

plt.tight_layout()

out_path = os.path.join(os.path.dirname(__file__), 'pareto_frontier.png')
plt.savefig(out_path, dpi=150)
print(f"Saved: {out_path}")
