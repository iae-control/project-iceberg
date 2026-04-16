# Prior Art Analysis: EML Sheffer Operator (arXiv:2603.21852v2)

**Prepared for:** Project Iceberg  
**Date:** 2026-04-15  
**Source paper:** Andrzej Odrzywołek, "All elementary functions from a single binary operator," arXiv:2603.21852v2 (submitted 2026-03-23, revised 2026-04-04)

---

## Executive Summary

The EML paper's central claim — that `eml(x,y) = exp(x) - ln(y)` with constant 1 is a **concrete, explicit construction** generating all 36 elementary function primitives — is factually verifiable and internally consistent within the paper's stated definition. However, the community discussion since March 2026 has surfaced five categories of prior art and two substantive mathematical criticisms that are relevant to assessing novelty. The key distinction throughout is **existence theorem vs. explicit construction**: prior work often establishes that a universal primitive *can* exist; EML provides a *specific, named, elementary function* that *demonstrably works*.

---

## 1. Webb 1935 — "Generation of Any N-Valued Logic by One Binary Operation" (PNAS)

### What the paper is

**Citation:** Donald L. Webb, "Generation of any N-Valued Logic by One Binary Operation," *Proceedings of the National Academy of Sciences*, Vol. 21, No. 5, pp. 252–254 (May 15, 1935).  
**URL:** https://www.pnas.org/doi/10.1073/pnas.21.5.252

This is the paper identified by Hacker News commenter **SideQuark** as directly relevant prior art. It is *not* (as one might initially assume) about universal computation in Turing's sense. Webb's paper is about **multi-valued (n-valued) propositional logic**.

### What it proves

Webb shows that for any n-valued (finite, discrete) logic system, a single binary operation can generate all truth functions of that system. This is a generalization of Sheffer's 1913 result for classical 2-valued Boolean logic (where NAND is the single universal gate). Webb is essentially the person who showed the Sheffer stroke concept generalizes beyond binary logic to n-valued logic — hence the "Webb function" in multi-valued logic.

### What it does NOT do

- It is purely about **discrete, finite-valued logic** — it does not address continuous mathematics, real-valued functions, or elementary functions.
- It establishes that *some* universal n-valued operation exists; it does not find one for continuous real functions.
- It gives no procedure for expressing transcendental functions (sin, exp, etc.) from a single operation.

### Does it diminish EML's novelty?

**No — different domain entirely.** Webb 1935 is the precedent for the *name* "Sheffer operator" being applied to systems that can generate all operations, but EML operates in continuous mathematics where no discrete-logic precedent applies. The HN commenter who cited this was pointing to terminological precedent, not mathematical overlap.

---

## 2. Terence Tao — Universal Dynamical Systems / MathOverflow Construction

### What Tao's work is

Tao has two relevant bodies of work:

**2017 — Universality of the Euler equation:** Tao showed in "On the universality of the incompressible Euler equation on compact manifolds" (*Discrete and Continuous Dynamical Systems*, 2018; blog post 2017-07-25) that any quadratic ODE can be embedded into the incompressible Euler equations. This is part of a broader program to show that the Euler/Navier-Stokes equations are "Turing universal" as dynamical systems — i.e., that they can simulate any computable process.

**MathOverflow context (cited by HN):** The Hacker News commenter SideQuark stated: "that 2nd link has a nice construction by Terry Tao giving a clear way to show infinitely many such functions exist for pretty much any set of operations." This refers to a MathOverflow answer where Tao (or someone referencing his methods) demonstrated that **infinitely many** binary functions are universal for any given "closed" set of operations, using tools from dynamical systems and universal function theory.

**Rubel 1981 + Bournez-Pouly 2017 (closely related):** Lee Rubel proved in 1981 (PNAS 78:8, p. 4661) that there exists a fixed fourth-order polynomial differential algebraic equation whose solutions approximate any continuous function to any desired accuracy. Bournez and Pouly (ICALP 2017, arXiv:1702.08328) extended this to show a fixed **polynomial ODE** (not DAE) exists with analytic solutions that are Turing-universal — i.e., initial conditions can be chosen to simulate any Turing machine computation.

### What it does NOT do

- Tao's dynamical-systems universality is an **existence result**: it shows universal ODEs/PDEs exist but does not identify `eml(x,y) = exp(x) - ln(y)` or any other specific elementary function expression.
- Rubel and Bournez-Pouly prove existence of universal ODEs with **non-elementary** initial conditions (initial conditions must be computed, not given as named constants).
- None of this work claims a **binary function on reals** (rather than an ODE) generates all elementary functions from a single constant.

### Does it diminish EML's novelty?

**Partially — as an existence claim, but not as an explicit construction.** If EML's only claim were "a universal binary operator for elementary functions *exists*," then Tao's MathOverflow construction and related work would constitute clear prior art. The claim that EML is *novel* rests on:

1. The operator is itself an **elementary function** (simple, named: `exp(x) - ln(y)`).
2. The companion constant is **1** (simple, natural).
3. It was found by **exhaustive search** and each of the 36 targets is **explicitly constructed** — not just shown to exist.
4. Tao's approach (per SideQuark's summary) shows infinitely many such functions exist; it does not identify which ones or provide explicit derivations of sin, cos, sqrt, etc. from them.

The distinction matters: "there exist infinitely many universal binary functions" (Tao) vs. "here is one, it is `exp(x) - ln(y)`, and here is how to build sin(x) from it" (EML). EML is a **constructive existence proof with an explicit witness**.

---

## 3. SKI Combinator Calculus

### What it is

SKI combinator calculus is a minimal Turing-complete formal system using three operations:
- **I**: `I x = x` (identity)
- **K**: `K x y = x` (constant)
- **S**: `S x y z = (x z)(y z)` (substitution)

The S and K combinators together are sufficient to represent all lambda-calculus computations (I is redundant — `SKK = I`). The **Iota combinator** (introduced by Chris Barker) is a single combinator from which both S and K can be derived, making it a "single combinator" universal system:

`ι = λf. ((f S) K)`

### How EML relates

HN commenter **lioeters** drew this parallel: "Reminds me of the Iota combinator, one of the smallest formal systems that can be combined to produce a universal Turing machine." The EML paper itself draws this analogy explicitly: the uniform grammar `S → 1 | eml(S,S)` mirrors the recursive structure of combinator calculus.

### Key distinction

- SKI calculus is **symbolic/discrete**: it operates on abstract terms, not numerical values.
- SKI is Turing-complete for **computable functions** (integers, strings), not specifically for **elementary functions of real numbers**.
- The Iota combinator requires infinitely deep terms to represent arbitrary computations; EML requires only finite depth (max K=9 observed for all 36 targets).
- SKI was known since the 1930s (Curry); it is **not** an existence result for continuous elementary functions.

### Does it diminish EML's novelty?

**No — different computational substrate.** The analogy is conceptually valid (single primitive generating everything), but the mathematical content is entirely different. EML's claim is specifically about real-valued numerical computation, not symbolic computation. The HN comparison is aesthetically apt but mathematically non-overlapping.

---

## 4. SUBLEQ — Single-Instruction Set Computer (OISC)

### What it is

SUBLEQ (SUBtract and Branch if Less-than or EQual to zero) is a one-instruction computer architecture in which the single instruction `subleq a, b, c` computes `mem[b] = mem[b] - mem[a]` and jumps to address `c` if the result ≤ 0. Despite having only one instruction, SUBLEQ is Turing-complete (it can simulate any Minsky machine, which is Turing-complete). The paper [28] cited in the EML paper is: O. Mazonka and A. Kolodin, "A Simple Multi-Processor Computer Based on Subleq" (2011).

### How EML relates

The EML paper cites SUBLEQ explicitly as an example from the "minimal instruction set" tradition — the conceptual precedent that computing power can be concentrated into a single, repeatedly applied operation. It is used as a **motivating analogy**, not a mathematical precursor.

### Key distinction

- SUBLEQ is Turing-complete for **discrete, integer computation** (memory cells are integers, jumps are to integer addresses).
- SUBLEQ does not generate elementary functions — it does not even "know about" exp, ln, sin, etc.
- SUBLEQ's universality is indirect: it simulates arbitrary computation, but encoding sin(x) in SUBLEQ requires implementing a numerical algorithm (Taylor series, CORDIC, etc.) in subleq programs — not a direct expression.
- EML's targets are **closed-form expressions** at finite depth, not algorithmic procedures.

### Does it diminish EML's novelty?

**No — conceptual analogy only.** The paper uses SUBLEQ as a rhetorical comparison ("in computing, one instruction suffices; in continuous math, one operator suffices"). There is no mathematical overlap.

---

## 5. NAND Gate / Sheffer Stroke (Boolean Logic)

### What it is

**Henry Sheffer** showed in 1913 (Transactions of the American Mathematical Society) that the single Boolean operation NAND (`NOT(A AND B)`) suffices to express all Boolean functions. This is the original "Sheffer stroke." Independently, C.S. Peirce discovered NOR has the same property. Jean Nicod formalized the notation in 1917.

The key properties:
- `NOT(x) = NAND(x, x)`
- `AND(x, y) = NOT(NAND(x, y)) = NAND(NAND(x,y), NAND(x,y))`
- `OR(x, y) = NAND(NOT(x), NOT(y))`

This is the direct mathematical precedent the EML paper explicitly cites for its claim of a "continuous analog to the Sheffer stroke."

### How EML relates

The paper's abstract begins: "A single two-input gate suffices for all of Boolean logic in digital hardware. No comparable primitive has been known for continuous mathematics." EML explicitly positions itself as **the continuous-function analog of NAND**. The analogy is:

| Domain | System | Single operator | Constant |
|--------|--------|----------------|---------|
| Boolean | Classical logic | NAND | — |
| N-valued logic | Webb 1935 | Webb function | — |
| Continuous math | EML (2026) | exp(x) - ln(y) | 1 |

### Does it diminish EML's novelty?

**No — it is the explicit motivation.** The paper acknowledges NAND as the template and claims to fill the gap it identifies: "no comparable primitive has been known for continuous mathematics." The Sheffer/NAND precedent is what makes EML's claim interesting, not what undermines it.

---

## 6. The 1/(x-y) Universality Counterargument (HN Discussion)

### What was claimed

HN commenter **SideQuark** argued: "For example, let f(x,y) = 1/(x-y). This too is universal: x#0 = 1/(x-0) = 1/x (reciprocal); (x#y)#0 = x-y (subtraction)."

This would, if true, show that EML is not special — many functions are universal.

### Analysis

Commenter **freehorse** correctly rebutted: "The article is about producing all elementary functions, which 1/(x-y) clearly doesn't, as it doesn't produce any transcendental function."

**SideQuark's counterargument collapses** because:
1. `1/(x-y)` generates arithmetic operations but has no path to exp, ln, sin, or cos — these require transcendental content that pure rational operations cannot provide.
2. This is provable via Liouville's theorem: the field generated by compositions of a rational function over Q is still a rational function field; it cannot contain transcendentals.
3. EML gets its transcendental power from the `exp` and `ln` components — these are irreducibly transcendental.

**Conclusion:** The `1/(x-y)` argument is mathematically invalid for the transcendental case. EML's universality over elementary functions (including exp, ln, sin, cos) is a substantively harder claim.

---

## 7. Independent Verification Since March 2026

### 7.1 lilting.ch — Technical Verification Blog Post

**URL:** https://lilting.ch/en/articles/eml-single-operator-elementary-functions  
**Published:** Shortly after arXiv submission (April 2026)

**Author's verification approach:** Independent implementation of EML in Node.js, Python, PHP, Go, and Rust. Results:

- Cross-language results agree at LSB level (IEEE 754 precision)
- EML formula for `exp(x)` (depth 1): zero error
- EML formula for `ln(x)` (depth 3): ~10^-16 error
- Arithmetic operations (depth 27+): proportionally larger but bounded errors
- Multiplication requires 41+ nesting levels

**Key limitation findings:**
- Complex numbers are required internally (for π via `ln(-1) = iπ`)
- Extended reals required (`ln(0) = -∞`, `e^{-∞} = 0`)
- **Practical characterization:** "theoretically elegant but computationally nonsensical"
- Symbolic regression success drops below 1% at depth 5

**Verdict:** The blog **confirms** the paper's arithmetic and constructions work numerically. It is critical of practical utility, not mathematical validity.

### 7.2 stylewarning.com — "Not All Elementary Functions Can Be Expressed with exp-minus-log"

**URL:** https://www.stylewarning.com/posts/not-all-elementary/  
**Published/edited:** April 16, 2026 (edit note: original contained an absolute value example that was removed as "more distracting than illuminating")

**Core argument:** The blog uses **Khovanskii's topological Galois theory** to argue EML cannot express all elementary functions in the *standard mathematical sense*:

1. Every EML term has a **solvable monodromy group** (because exp and ln compositions only produce solvable extensions of the fundamental group).
2. The generic quintic polynomial has monodromy group S₅, which is **not solvable** (Abel-Ruffini theorem).
3. Therefore, the roots of a generic quintic polynomial cannot be expressed as EML terms.
4. Conclusion: EML terms form a **strict subset** of what mathematicians standardly call "elementary functions."

**Critical caveat acknowledged by the author:** "I audited graduate-level mathematics courses almost 20 years ago, and I am not a professional mathematician." The argument is presented tentatively and has not been peer-reviewed.

**Paper's actual position:** Odrzywołek does **not** claim universality in the Liouville sense. The paper explicitly adopts a "scientific calculator" definition (36 primitives from Table 1) and acknowledges: "there is no agreed list of elementary functions." The critic and the paper are operating under **different definitions of "elementary."**

**Verdict on this criticism:** The stylewarning critique is **valid as a terminological/definitional correction** but does not contradict the paper's own claims. The paper does not claim EML generates all Liouville-elementary functions; it claims EML generates a specific named 36-item basis. The blog post's rebuttal applies to a claim the paper does not make.

### 7.3 emlvm (nullwiz, GitHub)

**URL:** https://github.com/nullwiz/emlvm  
**Type:** Independent implementation

A CLI stack machine executing EML programs in postfix (RPN) notation. Features:
- `emlvm verify`: Verifies all known EML programs against Python's `math` library
- `emlvm sym`: SymPy symbolic proof of expressions
- Cross-verification confirms paper's constructions

**Verdict:** Confirms the paper's constructions numerically and symbolically for the functions verified.

### 7.4 Marouane Lamharzi Alaoui — "I Found Out Why You Can't Build a One-Button Calculator"

**Platform:** X (Twitter), April 13, 2026  
**Finding:** Investigated removing the constant 1 (making EML fully "self-starting"). Concluded: "the most natural self-starting strategy is impossible in the analytic setting, and the purely real [strategy is also impossible]."

This is consistent with the paper's own stated limitation: "Whether a univariate Sheffer exists... remains open" and "a continuous Sheffer working purely in the real domain seems impossible."

**Verdict:** Independent confirmation of a paper-acknowledged limitation, not a contradiction.

### 7.5 The Register — Media Coverage

**URL:** https://www.theregister.com/2026/04/14/two_button_calculator/  
**Published:** April 14, 2026

Popular press coverage noting the paper is unreviewed, the methods are numerical/heuristic (not proof-level formal), and that "there is no agreed list of elementary functions." The article characterizes the paper fairly but does not provide technical criticism beyond what the paper acknowledges.

---

## 8. Synthesis: Novelty Assessment

### What the prior art shows

| Prior work | Domain | Type of result | Overlaps EML? |
|-----------|--------|---------------|--------------|
| Sheffer 1913 / NAND | Boolean logic (discrete) | Existence + construction | No (different domain) |
| Webb 1935 | N-valued discrete logic | Existence + construction | No (different domain) |
| SKI / Iota combinator | Symbolic computation | Existence + construction | No (different substrate) |
| SUBLEQ | Integer computation | Existence + construction | No (different domain) |
| Rubel 1981 | Smooth function approximation | Existence only | Partial (different claim) |
| Bournez-Pouly 2017 | Computable functions via ODE | Existence only | Partial (different claim) |
| Tao MathOverflow | Binary universality existence | Existence only | YES — see below |

### The most significant prior art issue: Tao's existence result

The Hacker News commenter SideQuark cited a Tao MathOverflow construction showing "infinitely many such functions exist for pretty much any set of operations." If this claim is accurate (the exact MathOverflow post was not retrieved), it would mean:

- **EML's existence** was not surprising in principle — such operators were known to exist.
- **EML's novelty** then rests on: (a) identifying the specific operator `exp(x) - ln(y)`, (b) the simplicity of the constant (1), (c) the explicit construction of all 36 targets, (d) the application to symbolic regression.

The paper's own abstract partially acknowledges this framing by saying "That such an operator exists was **not anticipated**" — which would be incorrect if Tao's work genuinely implies existence. However, Tao's work (if it exists in the form described) likely operates in a broader or different theoretical framework (e.g., closure under a topology or measure-theoretic argument) rather than giving an explicit elementary function as the operator.

**Key distinction, stated precisely:**

> Prior existence results (if any) show that *some* binary function f: R×R → R generates all elementary functions under iterated composition. EML shows that `f(x,y) = exp(x) - ln(y)` specifically is such a function and provides explicit finite-depth expressions for all 36 targets. The prior existence claim, even if valid, does not tell you the operator is elementary, that the constant is 1, or how to build any specific function from it.

### Final assessment

EML's claim of **novelty** is defensible in the following specific sense:

1. **No prior work provides an explicit elementary function as a single universal primitive** for continuous mathematics.
2. **No prior work gives the explicit bootstrapping chains** for all standard elementary functions from a single such primitive.
3. **The Sheffer/discrete-logic analogy is genuinely new** in the continuous domain.

EML's claim is **weakened** by:

1. The paper's own admission that the result is "not anticipated" may misstate the situation if Tao or others have shown such operators exist in principle.
2. The definition of "elementary functions" is non-standard (36-item basis, not Liouville elementary), and this limits the claim's scope.
3. The verification is numerical/heuristic, not formally proven (acknowledged by the paper itself).
4. Practical utility is extremely limited (depth-27+ trees for basic arithmetic).

---

## 9. Open Questions and Recommended Follow-Up

1. **Retrieve the specific Tao MathOverflow answer** referenced by SideQuark (the URL was not directly captured). This is the most important unresolved prior art question.

2. **Retrieve the Goldstern TU Wien paper** (`singlebinary.pdf`) which SideQuark also linked — this may be a more direct mathematical predecessor to EML's claim for continuous functions.

3. **Stylewarning Khovanskii argument** warrants expert review: is the monodromy group argument correctly applied? If EML's expressions do have solvable monodromy, this would be a genuine mathematical limitation (the paper cannot, e.g., express roots of general quintics) — but this is not what the paper claims.

4. **Lobste.rs branch cut issue:** Commenter tzlil identified that `ln(z) = eml(1, eml[eml(1,z), 1])` fails for negative values (z = -1). The paper acknowledges the branch cut issue but the practical severity for the 36-item basis is unclear.

---

## References Consulted

- arXiv:2603.21852v2 (the paper itself)
- https://lilting.ch/en/articles/eml-single-operator-elementary-functions (independent verification)
- https://www.stylewarning.com/posts/not-all-elementary/ (Khovanskii theory critique, edited 2026-04-16)
- https://news.ycombinator.com/item?id=47746610 (HN discussion, via brianlovin.com mirror)
- https://lobste.rs/s/rnx6fh/all_elementary_functions_from_single (Lobsters discussion)
- https://www.theregister.com/2026/04/14/two_button_calculator/ (press coverage)
- https://github.com/nullwiz/emlvm (independent implementation)
- https://www.pnas.org/doi/10.1073/pnas.21.5.252 (Webb 1935 — the cited 1935 PNAS paper)
- https://www.pnas.org/doi/10.1073/pnas.78.8.4661 (Rubel 1981 — universal differential equation)
- https://arxiv.org/abs/1702.08328 (Bournez-Pouly 2017 — universal ODE)
- https://en.wikipedia.org/wiki/SKI_combinator_calculus
- https://en.wikipedia.org/wiki/One-instruction_set_computer (SUBLEQ)
- https://en.wikipedia.org/wiki/Sheffer_stroke (Sheffer 1913, NAND)
- https://x.com/marouane53/status/2043763900844589448 (self-starting impossibility)
