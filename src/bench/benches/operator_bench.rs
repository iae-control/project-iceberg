use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Inline operator implementations (no external deps)

fn eml(a: f64, b: f64) -> f64 {
    a.exp() - b.ln()
}

fn edl(a: f64, b: f64) -> f64 {
    a.exp() / b.ln()
}

fn div_log_exp(a: f64, b: f64) -> f64 {
    a.ln() / b.exp()
}

fn power_log_inv(a: f64, b: f64) -> f64 {
    // ln(a)^(1/b) — for a>1 (ln(a)>0), this is real
    a.ln().powf(1.0 / b)
}

// === Bootstrapping chains for core functions ===

// EML chain
fn eml_exp(x: f64) -> f64 { eml(x, 1.0) }
fn eml_ln(x: f64) -> f64 { eml(1.0, eml_exp(eml(1.0, x))) }
fn eml_sub(a: f64, b: f64) -> f64 { eml(eml_ln(a), eml_exp(b)) }
fn eml_neg(x: f64) -> f64 { eml_sub(eml_ln(1.0).exp(), x) } // 0 - x via chain
fn eml_plus(a: f64, b: f64) -> f64 { eml_sub(a, eml_neg(b)) }
fn eml_inv(x: f64) -> f64 { eml_exp(eml_neg(eml_ln(x))) }
fn eml_times(a: f64, b: f64) -> f64 { eml_exp(eml_plus(eml_ln(a), eml_ln(b))) }
fn eml_div(a: f64, b: f64) -> f64 { eml_times(a, eml_inv(b)) }

// PLI chain
fn pli_ln(x: f64) -> f64 { power_log_inv(x, 1.0) }
fn pli_exp(x: f64) -> f64 {
    // PLI(5, Log(PLI(5, x))) where 5 is arbitrary y>1
    let y = 5.0_f64;
    let inner = power_log_inv(y, x); // ln(5)^(1/x)
    let log_inner = inner.ln();       // ln(ln(5)^(1/x))
    power_log_inv(y, log_inner)       // ln(5)^(1/log_inner) = exp(x)
}
fn pli_inv(x: f64) -> f64 { (power_log_inv(std::f64::consts::E.exp(), x)).ln() }
fn pli_neg(x: f64) -> f64 { pli_inv(pli_exp(x)).ln() }
fn pli_power(a: f64, b: f64) -> f64 { power_log_inv(pli_exp(a), pli_inv(b)) }
fn pli_times(a: f64, b: f64) -> f64 { (pli_power(pli_exp(a), b)).ln() }
fn pli_sub(a: f64, b: f64) -> f64 { (pli_exp(a) / pli_exp(b)).ln() }
fn pli_plus(a: f64, b: f64) -> f64 { pli_sub(a, pli_neg(b)) }
fn pli_div(a: f64, b: f64) -> f64 { pli_times(a, pli_inv(b)) }

fn bench_raw_operators(c: &mut Criterion) {
    let a = 0.5772156649015329_f64; // Euler-Mascheroni
    let b = 1.2824271291006227_f64; // Glaisher

    let mut group = c.benchmark_group("raw_operator");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml(black_box(a), black_box(b)))
    });
    group.bench_function("EDL", |bencher| {
        bencher.iter(|| edl(black_box(a), black_box(b)))
    });
    group.bench_function("DivLogExp", |bencher| {
        bencher.iter(|| div_log_exp(black_box(a), black_box(b)))
    });
    group.bench_function("PowerLogInv", |bencher| {
        bencher.iter(|| power_log_inv(black_box(a), black_box(b)))
    });
    group.finish();
}

fn bench_chain_functions(c: &mut Criterion) {
    let x = 2.0_f64;
    let a = 2.5_f64;
    let b = 3.0_f64;

    let mut group = c.benchmark_group("chain_exp");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml_exp(black_box(x)))
    });
    group.bench_function("PLI", |bencher| {
        bencher.iter(|| pli_exp(black_box(x)))
    });
    group.finish();

    let mut group = c.benchmark_group("chain_ln");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml_ln(black_box(x)))
    });
    group.bench_function("PLI", |bencher| {
        bencher.iter(|| pli_ln(black_box(x)))
    });
    group.finish();

    let mut group = c.benchmark_group("chain_plus");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml_plus(black_box(a), black_box(b)))
    });
    group.bench_function("PLI", |bencher| {
        bencher.iter(|| pli_plus(black_box(a), black_box(b)))
    });
    group.finish();

    let mut group = c.benchmark_group("chain_times");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml_times(black_box(a), black_box(b)))
    });
    group.bench_function("PLI", |bencher| {
        bencher.iter(|| pli_times(black_box(a), black_box(b)))
    });
    group.finish();

    let mut group = c.benchmark_group("chain_div");
    group.bench_function("EML", |bencher| {
        bencher.iter(|| eml_div(black_box(a), black_box(b)))
    });
    group.bench_function("PLI", |bencher| {
        bencher.iter(|| pli_div(black_box(a), black_box(b)))
    });
    group.finish();
}

criterion_group!(benches, bench_raw_operators, bench_chain_functions);
criterion_main!(benches);
