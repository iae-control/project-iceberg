# Reproduction Guide

## Prerequisites

- Rust 1.94+ (for verification engine)
- Python 3.10+ (for enumeration and analysis)
- Python packages: mpmath, sympy, numpy, matplotlib

## Step 1: Build the Rust verifier

    cd src/author_repo/rust_verify
    cargo build --release

## Step 2: Reproduce EML verification

    ./target/release/verify_base_set_rs --constants 1 --functions '' --operations EML --max-k 9 --explain

Expected: 35/35 targets found in ~37s.

## Step 3: Reproduce PLI verification

    ./target/release/verify_base_set_rs --constants 1 --functions '' --custom-op "PLI:Pow(Log,Inv)" --operations PLI --max-k 9 --explain

Expected: 35/35 targets found in ~38s.

## Step 4: Level 1 full census

    cd ../../../
    python src/enumerator/enumerate_level1.py
    bash scripts/batch_level1_full.sh

Expected: 14,077 runs, 4 Sheffer hits (2 PLI + 2 DLE-equiv).

## Step 5: Level 2 census

    python src/enumerator/enumerate_level2.py
    python src/enumerator/stage1_filter.py
    bash scripts/batch_level2_stage2.sh

Expected: 120,700 candidates → 635 after filtering → 10 Sheffer hits (all equivalent to known families).

## Step 6: Numerical stability benchmark

    python src/analyzer/stability_four_ops.py

Expected: Generates results/stability_four_ops.csv.

## Step 7: Empirical cost benchmark

    cd src/bench
    cargo run --release

Expected: Raw operator ns and chain function ns for EML/PLI.

## Verification

All results should match the files in the results/ directory exactly. The pipeline is fully deterministic given the same operator definition, constant set, K_max, evaluation points, numerical precision, and tolerance threshold. No randomized components are used.

## Hardware

Original results were produced on:
- CPU: AMD Ryzen 9 9950X3D
- OS: Windows 11
- Rust: 1.94.1 (release build, opt-level=3)

Timing results may differ on other hardware, but target counts and Sheffer hit counts are deterministic and should be identical.
