# Project Iceberg

**Discovery of PowerLogInv and computational census of continuous Sheffer operator families**

Odrzywołek (2026) showed that `eml(x,y) = exp(x) - ln(y)` is a continuous analogue of the NAND gate — a single binary operator that generates all elementary functions. We discover a new Sheffer operator:

    pli(x,y) = ln(x)^(1/y)    (companion constant: 1)

and prove it is non-equivalent to all known operators. A computational census of 14,712 verification runs across two structural levels finds exactly three operator families: EML, EDL, and PLI.

## Key Results

| Result | Detail |
|--------|--------|
| New operator | PowerLogInv: ln(x)^(1/y) |
| Non-equivalence | Proven under T1-T5 (Theorem 1) |
| Census | 14,712 runs, 3 families, no 4th |
| Depth hierarchy | Sheffer at depth 1-2 only |
| 13/35 barrier | exp/ln bridge is necessary |
| Theoretical foundation | exp/ln necessity proven via Liouville-Ritt tower |

## Paper

- arXiv: *link TBD*
- PDF: [paper/main.pdf](paper/main.pdf)

## Repository Structure

    project-iceberg/
    ├── README.md
    ├── REPRODUCE.md
    ├── CITATION.cff
    ├── CLAUDE.md
    ├── paper/
    │   ├── main.tex               # arXiv paper (LaTeX)
    │   └── main.pdf
    ├── src/
    │   ├── author_repo/           # forked author's code (MIT)
    │   │   └── rust_verify/       # Rust verifier with --custom-op
    │   ├── enumerator/            # Python candidate enumeration
    │   ├── analyzer/              # stability, cost, SymPy verification
    │   ├── bench/                 # Rust empirical benchmarks
    │   └── verifier/              # Python verification engine
    ├── scripts/                   # batch search scripts
    └── results/
        ├── REPORT.md              # full technical report (v11)
        ├── verify_L1_results.csv  # Level 1 search (14,077 runs)
        ├── verify_L2_stage2.log   # Level 2 search log
        ├── candidates_L1.json     # 2,011 Level 1 candidates
        ├── candidates_L2.json     # 120,700 Level 2 candidates
        ├── stability_four_ops.csv # 4-operator benchmark
        ├── cost/                  # cost model + Pareto chart
        │   ├── empirical_benchmark.json
        │   └── pareto_frontier.png
        └── analysis/              # heatmap, barrier, sin cascade
            ├── coverage_heatmap.png
            ├── separation_venn.png
            └── symbolic_proofs/

## Reproduction

See [REPRODUCE.md](REPRODUCE.md) for step-by-step instructions.

## Citation

    @article{jeong2026powerloginv,
      title   = {PowerLogInv: A New Sheffer-Type Operator for
                 Elementary Functions and a Computational Census
                 of Operator Families},
      author  = {Jeong, SangHyeok},
      journal = {arXiv preprint},
      year    = {2026}
    }

## Acknowledgments

Built upon the foundational work of Andrzej Odrzywołek
([arXiv:2603.21852](https://arxiv.org/abs/2603.21852))
and his open-source verification engine
([SymbolicRegressionPackage](https://github.com/VA00/SymbolicRegressionPackage)).

## License

See [LICENSE](LICENSE) file.
