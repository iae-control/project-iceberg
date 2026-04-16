"""
Convert the Project Iceberg LaTeX paper to PDF using reportlab.
Renders a proper two-column academic paper layout.
"""
import os
from reportlab.lib.pagesizes import letter
from reportlab.lib.units import inch, cm
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_CENTER, TA_JUSTIFY, TA_LEFT
from reportlab.lib.colors import black, HexColor
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle,
    PageBreak, KeepTogether, HRFlowable
)
from reportlab.lib import colors

OUT = os.path.join(os.path.dirname(__file__), "main.pdf")

styles = getSampleStyleSheet()

# Custom styles
styles.add(ParagraphStyle(
    'PaperTitle', parent=styles['Title'],
    fontSize=14, leading=17, spaceAfter=6, alignment=TA_CENTER,
))
styles.add(ParagraphStyle(
    'Author', parent=styles['Normal'],
    fontSize=10, leading=12, alignment=TA_CENTER, spaceAfter=12,
))
styles.add(ParagraphStyle(
    'AbstractTitle', parent=styles['Normal'],
    fontSize=10, leading=12, fontName='Helvetica-Bold', alignment=TA_CENTER, spaceAfter=4,
))
styles.add(ParagraphStyle(
    'AbstractBody', parent=styles['Normal'],
    fontSize=9, leading=11, alignment=TA_JUSTIFY, leftIndent=36, rightIndent=36, spaceAfter=12,
))
styles.add(ParagraphStyle(
    'SectionHead', parent=styles['Heading2'],
    fontSize=11, leading=13, fontName='Helvetica-Bold', spaceAbove=14, spaceAfter=4,
))
styles.add(ParagraphStyle(
    'SubHead', parent=styles['Heading3'],
    fontSize=10, leading=12, fontName='Helvetica-Bold', spaceAbove=8, spaceAfter=3,
))
styles.add(ParagraphStyle(
    'Body', parent=styles['Normal'],
    fontSize=9, leading=11.5, alignment=TA_JUSTIFY, spaceAfter=4,
))
styles.add(ParagraphStyle(
    'SmallBody', parent=styles['Normal'],
    fontSize=8, leading=10, alignment=TA_JUSTIFY, spaceAfter=3,
))
styles.add(ParagraphStyle(
    'Equation', parent=styles['Normal'],
    fontSize=10, leading=13, alignment=TA_CENTER, spaceAbove=4, spaceAfter=4,
    fontName='Courier',
))
styles.add(ParagraphStyle(
    'TheoremHead', parent=styles['Normal'],
    fontSize=9, leading=11.5, fontName='Helvetica-BoldOblique', spaceAbove=6, spaceAfter=2,
))
styles.add(ParagraphStyle(
    'Proof', parent=styles['Normal'],
    fontSize=9, leading=11.5, alignment=TA_JUSTIFY, spaceAfter=4,
    fontName='Times-Italic',
))
styles.add(ParagraphStyle(
    'Reference', parent=styles['Normal'],
    fontSize=8, leading=10, leftIndent=18, firstLineIndent=-18, spaceAfter=2,
))

def make_table(headers, rows, col_widths=None):
    data = [headers] + rows
    if col_widths is None:
        col_widths = [None] * len(headers)
    t = Table(data, colWidths=col_widths)
    t.setStyle(TableStyle([
        ('FONTNAME', (0, 0), (-1, 0), 'Helvetica-Bold'),
        ('FONTSIZE', (0, 0), (-1, -1), 8),
        ('LEADING', (0, 0), (-1, -1), 10),
        ('TOPPADDING', (0, 0), (-1, -1), 2),
        ('BOTTOMPADDING', (0, 0), (-1, -1), 2),
        ('LINEBELOW', (0, 0), (-1, 0), 0.8, black),
        ('LINEBELOW', (0, -1), (-1, -1), 0.8, black),
        ('LINEABOVE', (0, 0), (-1, 0), 0.8, black),
        ('ALIGN', (1, 0), (-1, -1), 'CENTER'),
        ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
    ]))
    return t


def build():
    doc = SimpleDocTemplate(
        OUT, pagesize=letter,
        leftMargin=0.75*inch, rightMargin=0.75*inch,
        topMargin=0.75*inch, bottomMargin=0.75*inch,
    )
    story = []

    # === TITLE ===
    story.append(Paragraph(
        "PowerLogInv: A New Sheffer-Type Operator for Elementary Functions<br/>"
        "and a Computational Census of Operator Families",
        styles['PaperTitle']
    ))
    story.append(Paragraph(
        "SangHyeok Jeong<br/>"
        "Institute for Advanced Engineering, Gyeonggi-do, Republic of Korea<br/>"
        "<font size=8>ORCID: 0000-0002-9458-0792</font>",
        styles['Author']
    ))

    # === ABSTRACT ===
    story.append(Paragraph("Abstract", styles['AbstractTitle']))
    story.append(Paragraph(
        "Odrzywołek [1] showed that eml(x,y) = e<super>x</super> - ln y, together with the constant 1, "
        "generates all 36 elementary function primitives of a scientific calculator. "
        "We discover <b>PowerLogInv</b> (PLI), pli(x,y) = (ln x)<super>1/y</super>, a new continuous "
        "Sheffer operator <i>non-equivalent</i> to the known EML/EDL family. "
        "Non-equivalence is proven via the iterated logarithm ln(ln x) appearing in PLI's "
        "partial derivative structure but absent from all EML/EDL variants. "
        "A systematic computational census of 14,712 verification runs across two structural "
        "levels finds exactly three operator families (EML, EDL, PLI) and no fourth. "
        "We identify a <i>depth hierarchy</i>: Sheffer operators exist at depths 1-2 only. "
        "We characterize the <i>13/35 barrier</i>: non-exp/ln operators cannot achieve "
        "Sheffer-ness due to the additive-multiplicative bridge gap. "
        "Empirical benchmarks show EML dominates all variants in both cost (30.5 ns/op) "
        "and numerical stability (mean relative error 3.48 x 10<super>-10</super>).",
        styles['AbstractBody']
    ))
    story.append(HRFlowable(width="100%", thickness=0.5))
    story.append(Spacer(1, 8))

    # === 1. INTRODUCTION ===
    story.append(Paragraph("1. Introduction", styles['SectionHead']))
    story.append(Paragraph(
        "In Boolean logic, the NAND gate (Sheffer stroke [2]) is a single binary operation "
        "from which all Boolean functions can be derived. No analogous primitive was known for "
        "continuous mathematics until Odrzywołek [1] proved that eml(x,y) = e<super>x</super> - ln y, "
        "together with the companion constant 1, generates all 36 elementary function primitives "
        "of a standard scientific calculator. The author also identified two additional operators -- "
        "edl(x,y) = e<super>x</super>/ln y (with constant e) and -eml(x,y) = ln x - e<super>y</super> "
        "(with constant -infinity) -- and conjectured these were 'the tip of the iceberg.'",
        styles['Body']
    ))
    story.append(Paragraph(
        "Tao [3] showed on MathOverflow that infinitely many universal binary functions exist, "
        "but no second explicit, elementary-function Sheffer operator was known, "
        "and no systematic census had been performed.",
        styles['Body']
    ))
    story.append(Paragraph(
        "<b>Contributions.</b> (1) We discover <b>PowerLogInv</b>: pli(x,y) = (ln x)<super>1/y</super> "
        "with companion constant 1, a new Sheffer operator proven non-equivalent to EML/EDL "
        "(Theorem 1). (2) We complete a computational census of 14,712 verification runs across "
        "Level 1 (direct compositions) and Level 2 (nested compositions), finding exactly three "
        "operator families. (3) We establish a depth hierarchy (Proposition 1) and characterize "
        "the 13/35 barrier that prevents non-exp/ln operators from achieving Sheffer-ness.",
        styles['Body']
    ))

    # === 2. METHODOLOGY ===
    story.append(Paragraph("2. Methodology", styles['SectionHead']))
    story.append(Paragraph("2.1 Search Space", styles['SubHead']))
    story.append(Paragraph(
        "<b>Level 1:</b> b(u<sub>1</sub>(x), u<sub>2</sub>(y)) where b is one of 6 binary operations "
        "and u<sub>1</sub>, u<sub>2</sub> are chosen from 20 unary functions. "
        "After filtering: 2,011 candidates x 7 constants = 14,077 verification runs.",
        styles['Body']
    ))
    story.append(Paragraph(
        "<b>Level 2:</b> Three types of nested composition -- left nesting b(u<sub>1</sub>(u<sub>2</sub>(x)), u<sub>3</sub>(y)), "
        "right nesting b(u<sub>1</sub>(x), u<sub>2</sub>(u<sub>3</sub>(y))), and outer wrapping "
        "u<sub>0</sub>(b(u<sub>1</sub>(x), u<sub>2</sub>(y))) -- yielding 120,700 candidates. "
        "A 3-stage pipeline (Python K<sub>max</sub>=3 sieve -> Rust K<sub>max</sub>=7 verification -> "
        "equivalence analysis) reduces this to 635 candidates for full verification.",
        styles['Body']
    ))

    story.append(Paragraph("2.2 Equivalence Criteria", styles['SubHead']))
    story.append(Paragraph(
        "Two operators are trivially equivalent if related by any combination of: "
        "T<sub>1</sub> (argument swap), T<sub>2</sub> (negation), T<sub>3</sub> (reciprocal), "
        "T<sub>4</sub> (constant shift), T<sub>5</sub> (constant multiply). "
        "T<sub>1</sub>, T<sub>2</sub>, T<sub>3</sub> are commuting involutions generating "
        "(Z/2Z)<super>3</super> (order 8). T<sub>4</sub>, T<sub>5</sub> compose to general affine transforms.",
        styles['Body']
    ))

    story.append(Paragraph("2.3 Verification Pipeline", styles['SubHead']))
    story.append(Paragraph(
        "We fork the author's Rust verification engine [5], extending it with a --custom-op flag "
        "for runtime operator definition supporting nested composition syntax. "
        "Verification follows the bootstrapping algorithm of [1]: starting from {constant, op}, "
        "enumerate all RPN expressions up to complexity K, evaluate at algebraically independent "
        "transcendental test points (gamma = 0.5772..., A = 1.2824...), and check for numerical matches "
        "with 35 target functions (epsilon = 16 x epsilon_mach). "
        "Cross-validation: mpmath 128-bit (37/38 PASS for PLI) and SymPy (9 groups, 28 steps, all PASS).",
        styles['Body']
    ))

    # === 3. POWERLOGINV DISCOVERY ===
    story.append(Paragraph("3. PowerLogInv Discovery", styles['SectionHead']))
    story.append(Paragraph(
        "<b>Definition.</b> pli(x,y) = (ln x)<super>1/y</super>, with companion constant 1.",
        styles['Body']
    ))
    story.append(Paragraph(
        "The Rust verifier confirms pli generates all 35 default targets at K<sub>max</sub> = 9 "
        "in 38.53 s (complex domain). In real domain (K<sub>max</sub> = 7), 23/35 targets are found, "
        "including all arithmetic; only 8 trig functions remain (same as EML).",
        styles['Body']
    ))

    story.append(Paragraph("3.1 Bootstrapping Chain", styles['SubHead']))
    story.append(Paragraph("K=3: ln(x) = pli(x, 1)", styles['Equation']))
    story.append(Paragraph("K=5: e = pli(x, ln(ln(x)))  for any x > 1, x != e", styles['Equation']))
    story.append(Paragraph("K=6: e<super>x</super> = pli(y, ln(pli(y, x)))  for any y > 1, y != e", styles['Equation']))
    story.append(Paragraph("K=5: x<super>y</super> = pli(e<super>x</super>, 1/y)", styles['Equation']))
    story.append(Paragraph("K=5: x * y = ln((e<super>x</super>)<super>y</super>)", styles['Equation']))

    story.append(Paragraph(
        "<b>Derivation of e<super>x</super>.</b> Let L = ln y > 0 (requiring y > 1, y != e):",
        styles['Body']
    ))
    story.append(Paragraph(
        "pli(y, ln(pli(y,x))) = pli(y, ln(L<super>1/x</super>)) = pli(y, (1/x) ln L) "
        "= L<super>1/((1/x) ln L)</super> = L<super>x/ln L</super> = e<super>(x/ln L) ln L</super> = e<super>x</super>.",
        styles['Body']
    ))
    story.append(Paragraph(
        "The ln L terms cancel. This holds for any y > 1 with y != e "
        "(since y = e gives L = 1, ln L = 0, division by zero). "
        "Verified by SymPy (difference = 0) and numerically (y=5, x=gamma: exact match).",
        styles['Body']
    ))
    story.append(Paragraph(
        "<b>Mirror image property.</b> EML derives e<super>x</super> at K=3 and ln x at K=6; "
        "PLI derives ln x at K=3 and e<super>x</super> at K=6. They are structural mirrors.",
        styles['Body']
    ))

    story.append(Paragraph("3.2 Non-Equivalence Proof", styles['SubHead']))
    story.append(Paragraph("<b>Theorem 1.</b> pli(x,y) = (ln x)<super>1/y</super> is not equivalent to "
        "eml(x,y) = e<super>x</super> - ln y or edl(x,y) = e<super>x</super>/ln y "
        "under any combination of T<sub>1</sub>-T<sub>5</sub>.", styles['TheoremHead']))
    story.append(Paragraph(
        "<i>Proof.</i> <b>Structural argument:</b> "
        "d(pli)/dy = -(ln x)<super>1/y</super> ln(ln x) / y<super>2</super>. "
        "The term ln(ln x) (iterated logarithm) is absent from all partial derivatives "
        "of EML, EDL, -EML, and DivLogExp, which contain only e<super>x</super>, 1/y, ln y, or e<super>y</super>. "
        "Affine transforms (T<sub>4</sub>, T<sub>5</sub>) preserve gradient structure. "
        "Swap (T<sub>1</sub>), negation (T<sub>2</sub>), and reciprocal (T<sub>3</sub>) "
        "cannot introduce ln(ln(x)). "
        "<b>Exhaustive verification:</b> All 8 elements of (Z/2Z)<super>3</super> applied to PLI "
        "were compared against 4 target operators in both argument orders: "
        "48 SymPy comparisons, all NON-EQUIVALENT. Two supplementary group elements "
        "(-1/pli(x,y), -1/pli(y,x)) verified separately. QED.",
        styles['Body']
    ))

    # === 4. CENSUS RESULTS ===
    story.append(Paragraph("4. Census Results", styles['SectionHead']))
    story.append(Paragraph("4.1 Level 1 (14,077 runs)", styles['SubHead']))
    story.append(Paragraph(
        "Four Sheffer hits: pli + const=1 (35/35), pli + const=e (35/35), "
        "and two DivLogExp variants (equivalent to EDL via T<sub>1</sub> + T<sub>3</sub>).",
        styles['Body']
    ))

    story.append(make_table(
        ['Operator', 'Formula', 'Const', 'Found'],
        [
            ['Divide_ArcCos_Cos', 'arccos(x)/cos(y)', 'pi', '14/35'],
            ['Subtract_Cos_ArcSin', 'cos(x) - arcsin(y)', '0', '13/35'],
            ['Plus_ArcSin_Cos', 'arcsin(x) + cos(y)', '0', '13/35'],
            ['Times_Exp_Log', 'e^x ln(y)', 'e', '11/35'],
            ['Subtract_Inv_Id', '1/x - y', '1', '11/35'],
        ],
        col_widths=[2.2*inch, 1.5*inch, 0.5*inch, 0.6*inch],
    ))
    story.append(Paragraph("<i>Table 1: Top Level 1 partial hits (K<sub>max</sub> = 7).</i>", styles['SmallBody']))
    story.append(Spacer(1, 6))

    story.append(Paragraph("4.2 Level 2 (635 runs after filtering)", styles['SubHead']))
    story.append(Paragraph(
        "Ten Sheffer hits, all equivalent to known families: 5 EML, 3 EDL, "
        "1 PLI(y,x), 1 EDL(y,x). Zero new non-equivalent operators.",
        styles['Body']
    ))
    story.append(Paragraph(
        "<b>Conclusion:</b> At structural Levels 1-2, exactly three Sheffer operator families exist: "
        "EML (subtraction-based), EDL (division-based), and PLI (power-based).",
        styles['Body']
    ))

    # === 5. ANALYSIS ===
    story.append(Paragraph("5. Analysis", styles['SectionHead']))
    story.append(Paragraph("5.1 The 13/35 Barrier", styles['SubHead']))
    story.append(Paragraph(
        "Six targets were never generated by any of the top 20 partial hits: "
        "ArcSinh, ArcTan, LogisticSigmoid, Sinh, Tan, and binary Log. "
        "For cos(x) - arcsin(y) (13/35), the 22 missing targets partition into: "
        "(1) <b>no exponential bridge</b> -- e<super>x</super>, ln x, e, and all hyperbolic functions "
        "unreachable; (2) <b>no multiplicative structure</b> -- multiplication, division, power, "
        "reciprocal require multiplication which subtraction alone cannot produce; "
        "(3) <b>trig isolation</b> -- even tan = sin/cos requires division.",
        styles['Body']
    ))
    story.append(Paragraph(
        "The fundamental reason: cos(x) - arcsin(y) generates a subalgebra closed under "
        "addition and trig identities, but not multiplication. The exp-ln pair provides the "
        "unique elementary bridge between additive and multiplicative structures via "
        "e<super>a+b</super> = e<super>a</super> * e<super>b</super>. "
        "This explains why every known Sheffer operator contains both exp and ln.",
        styles['Body']
    ))

    story.append(Paragraph("5.2 Depth Hierarchy", styles['SubHead']))
    story.append(Paragraph(
        "<b>Proposition 1.</b> Define the derivative depth of an operator as the maximum nesting "
        "of logarithms in its partial derivatives. At K<sub>max</sub> = 7:",
        styles['TheoremHead']
    ))

    story.append(make_table(
        ['Depth', 'Structure', 'Example', 'Sheffer?'],
        [
            ['0', 'Rational', '1/(x-y)', 'No'],
            ['1', 'Contains exp/ln', 'EML, EDL', 'Yes'],
            ['2', 'Contains ln(ln x)', 'PLI', 'Yes'],
            ['>=3', 'Contains ln(ln(ln x))', '(ln(ln x))^(1/y)', 'No (<=6/35)'],
        ],
        col_widths=[0.6*inch, 1.3*inch, 1.5*inch, 1.1*inch],
    ))
    story.append(Paragraph("<i>Table 2: Depth hierarchy of Sheffer operators.</i>", styles['SmallBody']))
    story.append(Spacer(1, 6))

    story.append(Paragraph("5.3 Numerical Stability", styles['SubHead']))
    story.append(make_table(
        ['Metric', 'EML', 'EDL', 'DLE', 'PLI'],
        [
            ['Failure rate', '34.8%', '38.3%', '34.8%', '55.4%'],  # EML=254/729=34.8%
            ['Mean rel. err.', '3.48e-10', '9.19e-9', '8.71e-9', '1.51e-8'],
            ['Max rel. err.', '8.27e-8', '8.27e-8', '8.27e-8', '8.27e-7'],
            ['p99 digits lost', '1.4', '8.6', '8.6', '9.6'],
        ],
        col_widths=[1.1*inch, 0.85*inch, 0.85*inch, 0.85*inch, 0.85*inch],
    ))
    story.append(Paragraph(
        "<i>Table 3: Stability on 729-point grid (mpmath 50-digit reference). "
        "PLI's 164 complex-result failures account for its high failure rate.</i>",
        styles['SmallBody']
    ))
    story.append(Spacer(1, 4))
    story.append(Paragraph(
        "<b>Sin cascade.</b> Computing sin(1.0) via the full bootstrapping chain "
        "(constant -> exp/ln -> arithmetic -> i -> pi -> Euler formula), where derived functions "
        "are cached and reused rather than expanded as pure operator trees: "
        "all four operators achieve 15.9-16.0 correct digits (machine precision). "
        "PLI loses 0.1 digit due to its 3-nested exp chain.",
        styles['Body']
    ))

    story.append(Paragraph("5.4 Computational Cost", styles['SubHead']))
    story.append(make_table(
        ['Function', 'EML (ns)', 'PLI (ns)', 'Ratio'],
        [
            ['Raw operator', '30.5', '42.2', '1.38x'],
            ['exp(x) chain', '9.8', '108.0', '11.0x'],
            ['ln(x) chain', '73.1', '24.0', '0.33x'],
            ['x + y chain', '123.3', '256.0', '2.1x'],
            ['x * y chain', '169.5', '333.7', '2.0x'],
            ['x / y chain', '265.4', '415.0', '1.6x'],
        ],
        col_widths=[1.2*inch, 1.0*inch, 1.0*inch, 0.8*inch],
    ))
    story.append(Paragraph(
        "<i>Table 4: Empirical benchmarks (x86_64, AMD Ryzen 9 9950X3D, Windows 11, Rust 1.94.1, opt-level=3). "
        "PLI derives ln(x) 3x faster but exp(x) 11x slower. "
        "EML/EDL/DivLogExp are indistinguishable at ~30 ns (&lt;2% difference).</i>",
        styles['SmallBody']
    ))

    # === 6. DISCUSSION ===
    story.append(Paragraph("6. Discussion", styles['SectionHead']))
    story.append(Paragraph(
        "EML remains the optimal Sheffer operator on all practical axes: lowest raw cost, "
        "highest numerical stability, and simplest bootstrapping chain. PLI's contribution is "
        "<i>structural</i>, not computational -- it demonstrates that the Sheffer property "
        "extends to power-based operators with iterated logarithmic structure.",
        styles['Body']
    ))
    story.append(Paragraph(
        "The constant-free Sheffer operator (requiring no companion constant) remains open. "
        "Operators producing constants via op(x,x) = c exist (e.g., x * (1/x) = 1) "
        "but none achieves more than 9/35 targets. "
        "Tao's construction [3] implies infinitely many Sheffer operators exist; "
        "our census suggests they cluster into a small number of families.",
        styles['Body']
    ))

    # === 7. CONCLUSION ===
    story.append(Paragraph("7. Conclusion", styles['SectionHead']))
    story.append(Paragraph(
        "We discovered PowerLogInv, pli(x,y) = (ln x)<super>1/y</super>, the first continuous "
        "Sheffer operator proven non-equivalent to the EML family. "
        "A computational census of 14,712 verification runs across two structural levels "
        "identifies exactly three operator families. "
        "The 13/35 barrier analysis reveals that the exp-ln homomorphism is the essential "
        "structural ingredient for Sheffer-ness.",
        styles['Body']
    ))

    # === REFERENCES ===
    story.append(Spacer(1, 12))
    story.append(HRFlowable(width="100%", thickness=0.5))
    story.append(Paragraph("References", styles['SectionHead']))
    refs = [
        "[1] A. Odrzywołek, 'All elementary functions from a single binary operator,' arXiv:2603.21852v2, 2026.",
        "[2] H. M. Sheffer, 'A set of five independent postulates for Boolean algebras,' Trans. Amer. Math. Soc., vol. 14, pp. 481-488, 1913.",
        "[3] T. Tao, answer to 'Can we unify addition and multiplication into one binary operation?', MathOverflow Q57465.",
        "[4] D. L. Webb, 'Generation of any N-valued logic by one binary operation,' Proc. Natl. Acad. Sci., vol. 21, no. 5, pp. 252-254, 1935.",
        "[5] A. Odrzywołek, SymbolicRegressionPackage, github.com/VA00/SymbolicRegressionPackage, 2026.",
    ]
    for r in refs:
        story.append(Paragraph(r, styles['Reference']))

    doc.build(story)
    print(f"PDF saved to: {OUT}")
    print(f"Size: {os.path.getsize(OUT) / 1024:.0f} KB")


if __name__ == "__main__":
    build()
