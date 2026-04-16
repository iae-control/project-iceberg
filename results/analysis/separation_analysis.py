"""
B1: Separation Witness Search & B2: Closure Comparison
Enumerates values reachable by EML and PLI operators via bootstrapping.
"""

import cmath
import json
import math
import os

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

# ── Constants ──
GAMMA = 0.5772156649015329
GLAISHER = 1.2824271291006227
QUANT = 1e-10

def quantize(z):
    """Quantize a complex number to a grid for deduplication."""
    re = round(z.real / QUANT) * QUANT
    im = round(z.imag / QUANT) * QUANT
    return complex(re, im)

def quant_key(z):
    """Return a hashable key for a quantized complex number."""
    return (round(z.real / QUANT), round(z.imag / QUANT))

def is_finite(z):
    return math.isfinite(z.real) and math.isfinite(z.imag)

def eml(a, b):
    """EML(a,b) = exp(a) - log(b), b != 0"""
    if b == 0:
        return None
    try:
        val = cmath.exp(a) - cmath.log(b)
        if is_finite(val):
            return quantize(val)
    except (ValueError, OverflowError, ZeroDivisionError):
        pass
    return None

def pli(a, b):
    """PLI(a,b) = log(a) ** (1/b), a != 0, b != 0"""
    if a == 0 or b == 0:
        return None
    try:
        val = cmath.log(a) ** (1.0 / b)
        if is_finite(val):
            return quantize(val)
    except (ValueError, OverflowError, ZeroDivisionError):
        pass
    return None

def apply_op(op_func, pool_a, pool_b):
    """Apply operator to all pairs from two pools, return set of quant keys and value map."""
    results = {}
    for a in pool_a:
        for b in pool_b:
            val = op_func(a, b)
            if val is not None:
                k = quant_key(val)
                if k not in results:
                    results[k] = val
    return results

# ── K=1 seed pool ──
seed_values = [complex(1, 0), complex(GAMMA, 0), complex(GLAISHER, 0)]
seed_keys = {quant_key(v): v for v in [quantize(v) for v in seed_values]}

print("=" * 70)
print("B1: SEPARATION WITNESS SEARCH")
print("=" * 70)
print(f"\nK=1 seed pool: 1, gamma={GAMMA}, glaisher={GLAISHER}")
print(f"Seed pool size: {len(seed_keys)}")

# Pools by K: maps quant_key -> complex value
pools = {1: dict(seed_keys)}

# For each operator, track sets by K
eml_by_k = {1: set(seed_keys.keys())}
pli_by_k = {1: set(seed_keys.keys())}
eml_values = {1: dict(seed_keys)}
pli_values = {1: dict(seed_keys)}

# K=2 is not produced (operators take 2 inputs each of cost 1, so min output K=1+1+1=3 for binary op)
# Actually: K counts total constants used. Each seed has K=1.
# Applying op(a,b) where a has cost k_a and b has cost k_b gives cost k_a + k_b + 1 (the op itself costs 1?
# Re-reading: "K=3: apply the operator to all pairs from K=1 pool"
# So K=3 means applying the operator once to two K=1 elements: cost = 1 + 1 + 1(op) = 3?
# But the problem says K=1,2,3,4,5,6,7 and K=1 pool is the seeds.
# K=3: apply to K=1 pairs. K=5: apply to K=1 x K=3 and K=3 x K=1. K=7: apply to lower pairs summing to 5 (K=1 x K=5, K=3 x K=3, K=5 x K=1).
# So only odd K values get new elements. Even K values are empty (no way to combine odd+odd=even with +2 step).
# Wait: K=2 could be: apply op to two K=1 elements? That would be K=1+1=2.
# But the instructions say K=3 for pairs from K=1. So K for op(a,b) = K_a + K_b + 1.
# Then K=2 is impossible (need K_a+K_b+1=2, so K_a+K_b=1, but min is 1+1=2).
# K=3: K_a+K_b=2, so K_a=1, K_b=1. Correct.
# K=5: K_a+K_b=4: (1,3) or (3,1). Correct.
# K=7: K_a+K_b=6: (1,5), (3,3), (5,1). Correct.
# K=2,4,6: no combinations possible with this parity. Report empty sets.

# Track cumulative sets for final Venn
eml_all_keys = set(seed_keys.keys())
pli_all_keys = set(seed_keys.keys())
eml_all_values = dict(seed_keys)
pli_all_values = dict(seed_keys)

results_data = {}

for K in range(1, 8):
    if K == 1:
        e_set = set(seed_keys.keys())
        p_set = set(seed_keys.keys())
        intersection = e_set & p_set
        e_only = e_set - p_set
        p_only = p_set - e_set
        results_data[K] = {
            "eml_count": len(e_set),
            "pli_count": len(p_set),
            "intersection": len(intersection),
            "eml_only": len(e_only),
            "pli_only": len(p_only),
            "eml_only_values": [],
            "pli_only_values": [],
        }
        print(f"\nK={K}: |EML|={len(e_set)}, |PLI|={len(p_set)}, "
              f"|intersection|={len(intersection)}, |EML_only|={len(e_only)}, |PLI_only|={len(p_only)}")
        continue

    # Find all (k_a, k_b) pairs where k_a + k_b + 1 = K, k_a >= 1, k_b >= 1
    # So k_a + k_b = K - 1
    target_sum = K - 1
    pair_list = []
    for ka in range(1, target_sum):
        kb = target_sum - ka
        if kb >= 1 and ka in pools and kb in pools:
            pair_list.append((ka, kb))

    if not pair_list:
        # No valid combinations for this K
        eml_by_k[K] = set()
        pli_by_k[K] = set()
        eml_values[K] = {}
        pli_values[K] = {}
        pools[K] = {}
        results_data[K] = {
            "eml_count": 0, "pli_count": 0, "intersection": 0,
            "eml_only": 0, "pli_only": 0,
            "eml_only_values": [], "pli_only_values": [],
        }
        print(f"\nK={K}: No valid pair combinations. Empty sets.")
        continue

    eml_new = {}
    pli_new = {}

    for (ka, kb) in pair_list:
        pool_a_vals = list(pools[ka].values())
        pool_b_vals = list(pools[kb].values())
        print(f"  K={K}: combining K={ka} ({len(pool_a_vals)} vals) x K={kb} ({len(pool_b_vals)} vals)")

        eml_res = apply_op(eml, pool_a_vals, pool_b_vals)
        pli_res = apply_op(pli, pool_a_vals, pool_b_vals)

        eml_new.update(eml_res)
        pli_new.update(pli_res)

    eml_by_k[K] = set(eml_new.keys())
    pli_by_k[K] = set(pli_new.keys())
    eml_values[K] = eml_new
    pli_values[K] = pli_new

    # Merge into pools for future use (union of EML and PLI results)
    combined_pool = {}
    combined_pool.update(eml_new)
    combined_pool.update(pli_new)
    pools[K] = combined_pool

    # Update cumulative
    eml_all_keys.update(eml_new.keys())
    pli_all_keys.update(pli_new.keys())
    eml_all_values.update(eml_new)
    pli_all_values.update(pli_new)

    e_set = set(eml_new.keys())
    p_set = set(pli_new.keys())
    intersection = e_set & p_set
    e_only = e_set - p_set
    p_only = p_set - e_set

    # Get actual values for separation witnesses
    e_only_vals = [str(eml_new[k]) for k in sorted(e_only, key=lambda x: (x[0], x[1]))]
    p_only_vals = [str(pli_new[k]) for k in sorted(p_only, key=lambda x: (x[0], x[1]))]

    results_data[K] = {
        "eml_count": len(e_set),
        "pli_count": len(p_set),
        "intersection": len(intersection),
        "eml_only": len(e_only),
        "pli_only": len(p_only),
        "eml_only_values": e_only_vals[:20],  # cap for readability
        "pli_only_values": p_only_vals[:20],
    }

    print(f"\nK={K}: |EML|={len(e_set)}, |PLI|={len(p_set)}, "
          f"|intersection|={len(intersection)}, |EML_only|={len(e_only)}, |PLI_only|={len(p_only)}")

    if e_only:
        print(f"  ** SEPARATION WITNESSES (EML only, first 5):")
        for v in e_only_vals[:5]:
            print(f"     {v}")
    if p_only:
        print(f"  ** SEPARATION WITNESSES (PLI only, first 5):")
        for v in p_only_vals[:5]:
            print(f"     {v}")

# ── Summary ──
print("\n" + "=" * 70)
print("CUMULATIVE SUMMARY (K <= 7)")
print("=" * 70)

cum_intersection = eml_all_keys & pli_all_keys
cum_eml_only = eml_all_keys - pli_all_keys
cum_pli_only = pli_all_keys - eml_all_keys

print(f"|EML total|     = {len(eml_all_keys)}")
print(f"|PLI total|     = {len(pli_all_keys)}")
print(f"|intersection|  = {len(cum_intersection)}")
print(f"|EML only|      = {len(cum_eml_only)}")
print(f"|PLI only|      = {len(cum_pli_only)}")

if cum_eml_only:
    print(f"\nEML-only values (first 10):")
    for k in sorted(cum_eml_only, key=lambda x: (x[0], x[1]))[:10]:
        print(f"  {eml_all_values[k]}")

if cum_pli_only:
    print(f"\nPLI-only values (first 10):")
    for k in sorted(cum_pli_only, key=lambda x: (x[0], x[1]))[:10]:
        print(f"  {pli_all_values[k]}")

# ── B2: Venn Diagram ──
print("\n" + "=" * 70)
print("B2: CLOSURE COMPARISON - VENN DIAGRAM")
print("=" * 70)

eml_only_count = len(cum_eml_only)
both_count = len(cum_intersection)
pli_only_count = len(cum_pli_only)

print(f"EML only: {eml_only_count}")
print(f"Both:     {both_count}")
print(f"PLI only: {pli_only_count}")

try:
    from matplotlib_venn import venn2
    fig, ax = plt.subplots(figsize=(8, 6))
    v = venn2(subsets=(eml_only_count, pli_only_count, both_count),
              set_labels=('EML (K<=7)', 'PLI (K<=7)'), ax=ax)
    ax.set_title('Separation Analysis: EML vs PLI Reachable Values (K <= 7)', fontsize=13)
    plt.tight_layout()
    fig.savefig('D:/CLAUDE/EML_IAE/project-iceberg/results/analysis/separation_venn.png', dpi=150)
    print("\nVenn diagram saved.")
except Exception as e:
    print(f"matplotlib-venn failed ({e}), using bar chart fallback.")
    fig, ax = plt.subplots(figsize=(8, 5))
    categories = ['EML only', 'Both', 'PLI only']
    counts = [eml_only_count, both_count, pli_only_count]
    colors = ['#4a90d9', '#7bc67e', '#e07b54']
    bars = ax.bar(categories, counts, color=colors, edgecolor='black', linewidth=0.8)
    for bar, c in zip(bars, counts):
        ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
                str(c), ha='center', va='bottom', fontsize=12, fontweight='bold')
    ax.set_ylabel('Number of distinct values')
    ax.set_title('Separation Analysis: EML vs PLI Reachable Values (K <= 7)')
    plt.tight_layout()
    fig.savefig('D:/CLAUDE/EML_IAE/project-iceberg/results/analysis/separation_venn.png', dpi=150)
    print("\nBar chart saved as fallback.")

plt.close()

# ── Save JSON ──
def complex_to_str(z):
    if z.imag == 0:
        return str(z.real)
    return str(z)

json_data = {
    "description": "Separation witness search: EML vs PLI reachable values",
    "seed_pool": {"1": 1.0, "gamma": GAMMA, "glaisher": GLAISHER},
    "by_k": {},
    "cumulative": {
        "eml_total": len(eml_all_keys),
        "pli_total": len(pli_all_keys),
        "intersection": len(cum_intersection),
        "eml_only": eml_only_count,
        "pli_only": pli_only_count,
        "eml_only_sample": [complex_to_str(eml_all_values[k]) for k in sorted(cum_eml_only)[:30]],
        "pli_only_sample": [complex_to_str(pli_all_values[k]) for k in sorted(cum_pli_only)[:30]],
    }
}

for K in range(1, 8):
    json_data["by_k"][str(K)] = results_data[K]

with open('D:/CLAUDE/EML_IAE/project-iceberg/results/analysis/separation_data.json', 'w') as f:
    json.dump(json_data, f, indent=2)

print("\nJSON data saved.")
print("\nDone.")
