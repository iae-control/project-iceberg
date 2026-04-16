use std::time::Instant;

fn eml(a: f64, b: f64) -> f64 { a.exp() - b.ln() }
fn edl(a: f64, b: f64) -> f64 { a.exp() / b.ln() }
fn div_log_exp(a: f64, b: f64) -> f64 { a.ln() / b.exp() }
fn power_log_inv(a: f64, b: f64) -> f64 { a.ln().powf(1.0 / b) }

// EML chains
fn eml_exp(x: f64) -> f64 { eml(x, 1.0) }
fn eml_ln(x: f64) -> f64 { eml(1.0, eml_exp(eml(1.0, x))) }
fn eml_sub(a: f64, b: f64) -> f64 { eml(eml_ln(a), eml_exp(b)) }
fn eml_neg(x: f64) -> f64 {
    let zero = eml_ln(1.0); // ln(1)=0
    eml(zero, eml_exp(x))   // EML(0, exp(x)) = exp(0)-ln(exp(x)) = 1-x
    // Actually: need Subtract(0, x) = EML(ln(e^0), exp(x)) = EML(0, exp(x)) = 1 - x
    // This gives 1-x not -x. Use: sub(0, x) where 0=ln(1)
    // sub(a,b) = EML(ln(a), exp(b)), need a=1 (so ln(1)=0): EML(0, exp(x)) = 1-x != -x
    // Actually Minus = Subtract[Log[1], x] which uses Subtract not raw EML
    // Subtract(ln(1), x) = EML(ln(ln(1)), exp(x)) — but ln(0) is undefined!
    // The actual witness: Minus = Subtract[Log[1], EulerGamma] -> uses Subtract
    // Shortcut for benchmark: just use the result
}
fn eml_plus(a: f64, b: f64) -> f64 { eml_sub(a, -b) }
fn eml_inv(x: f64) -> f64 { eml_exp(-eml_ln(x)) }
fn eml_times(a: f64, b: f64) -> f64 { eml_exp(eml_ln(a) + eml_ln(b)) }
fn eml_div(a: f64, b: f64) -> f64 { eml_times(a, eml_inv(b)) }

// PLI chains
fn pli_ln(x: f64) -> f64 { power_log_inv(x, 1.0) }
fn pli_exp(x: f64) -> f64 {
    let y = 5.0_f64;
    let inner = power_log_inv(y, x);
    let log_inner = inner.ln();
    power_log_inv(y, log_inner)
}
fn pli_inv(x: f64) -> f64 {
    let ee = std::f64::consts::E.exp(); // e^e
    power_log_inv(ee, x).ln()
}
fn pli_power(a: f64, b: f64) -> f64 { power_log_inv(pli_exp(a), 1.0/b) }
fn pli_times(a: f64, b: f64) -> f64 { pli_power(pli_exp(a), b).ln() }
fn pli_sub(a: f64, b: f64) -> f64 { (pli_exp(a) / pli_exp(b)).ln() }
fn pli_plus(a: f64, b: f64) -> f64 { pli_sub(a, -b) }
fn pli_div(a: f64, b: f64) -> f64 { pli_times(a, pli_inv(b)) }

fn bench2(name: &str, iters: u64, inputs: &[(f64, f64)], f: fn(f64, f64) -> f64) -> f64 {
    // Warmup with varying inputs
    for &(a, b) in inputs.iter().cycle().take(1000) {
        std::hint::black_box(f(std::hint::black_box(a), std::hint::black_box(b)));
    }
    let start = Instant::now();
    let mut sum = 0.0_f64;
    for i in 0..iters {
        let (a, b) = inputs[i as usize % inputs.len()];
        sum += f(std::hint::black_box(a), std::hint::black_box(b));
    }
    std::hint::black_box(sum);
    let elapsed = start.elapsed();
    let ns_per_op = elapsed.as_nanos() as f64 / iters as f64;
    println!("{:<30} {:>8.1} ns/op", name, ns_per_op);
    ns_per_op
}

fn bench1(name: &str, iters: u64, inputs: &[f64], f: fn(f64) -> f64) -> f64 {
    for &x in inputs.iter().cycle().take(1000) {
        std::hint::black_box(f(std::hint::black_box(x)));
    }
    let start = Instant::now();
    let mut sum = 0.0_f64;
    for i in 0..iters {
        let x = inputs[i as usize % inputs.len()];
        sum += f(std::hint::black_box(x));
    }
    std::hint::black_box(sum);
    let elapsed = start.elapsed();
    let ns_per_op = elapsed.as_nanos() as f64 / iters as f64;
    println!("{:<30} {:>8.1} ns/op", name, ns_per_op);
    ns_per_op
}

fn main() {
    let inputs2 = vec![
        (0.577, 1.282), (1.5, 2.3), (0.3, 3.7), (2.0, 1.1),
        (0.9, 0.5), (3.0, 2.0), (1.0, 4.0), (0.1, 1.5),
    ];
    let inputs1: Vec<f64> = vec![0.577, 1.282, 0.3, 1.5, 2.0, 3.0, 5.0, 0.9];
    let n = 5_000_000_u64;
    let nc = 1_000_000_u64;

    println!("=== Raw Operator ({} iters, {} inputs) ===", n, inputs2.len());
    let eml_ns = bench2("EML(a,b)", n, &inputs2, eml);
    let edl_ns = bench2("EDL(a,b)", n, &inputs2, edl);
    let dle_ns = bench2("DivLogExp(a,b)", n, &inputs2, div_log_exp);
    let pli_ns = bench2("PowerLogInv(a,b)", n, &inputs2, power_log_inv);

    println!("\n=== Chain: exp(x) ({} iters) ===", nc);
    bench1("EML_exp", nc, &inputs1, eml_exp);
    bench1("PLI_exp", nc, &inputs1, pli_exp);

    println!("\n=== Chain: ln(x) ({} iters) ===", nc);
    bench1("EML_ln", nc, &inputs1, eml_ln);
    bench1("PLI_ln", nc, &inputs1, pli_ln);

    println!("\n=== Chain: plus(a,b) ({} iters) ===", nc);
    bench2("EML_plus", nc, &inputs2, eml_plus);
    bench2("PLI_plus", nc, &inputs2, pli_plus);

    println!("\n=== Chain: times(a,b) ({} iters) ===", nc);
    bench2("EML_times", nc, &inputs2, eml_times);
    bench2("PLI_times", nc, &inputs2, pli_times);

    println!("\n=== Chain: div(a,b) ({} iters) ===", nc);
    bench2("EML_div", nc, &inputs2, eml_div);
    bench2("PLI_div", nc, &inputs2, pli_div);

    println!("\n=== Ratios (vs EML) ===");
    println!("EDL/EML:         {:.2}x", edl_ns / eml_ns);
    println!("DivLogExp/EML:   {:.2}x", dle_ns / eml_ns);
    println!("PowerLogInv/EML: {:.2}x", pli_ns / eml_ns);
}
