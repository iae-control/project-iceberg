"""
Complex arithmetic for Sheffer operator verification.
Mirrors the author's Rust complex number implementation with IEEE754 handling.
"""
from __future__ import annotations
import cmath
import math
from typing import Optional


class C:
    """Complex number with explicit Option-style error propagation."""
    __slots__ = ('re', 'im')

    def __init__(self, re: float, im: float = 0.0):
        self.re = re
        self.im = im

    @staticmethod
    def real(x: float) -> C:
        return C(x, 0.0)

    @staticmethod
    def i() -> C:
        return C(0.0, 1.0)

    def is_finite(self) -> bool:
        return math.isfinite(self.re) and math.isfinite(self.im)

    def abs(self) -> float:
        return math.hypot(self.re, self.im)

    def arg(self) -> float:
        return math.atan2(self.im, self.re)

    def add(self, other: C) -> C:
        return C(self.re + other.re, self.im + other.im)

    def sub(self, other: C) -> C:
        return C(self.re - other.re, self.im - other.im)

    def mul(self, other: C) -> C:
        return C(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )

    def div(self, other: C) -> Optional[C]:
        den = other.re * other.re + other.im * other.im
        if den == 0.0:
            return None
        return C(
            (self.re * other.re + self.im * other.im) / den,
            (self.im * other.re - self.re * other.im) / den,
        )

    def neg(self) -> C:
        return C(-self.re, -self.im)

    def exp(self) -> C:
        try:
            ea = math.exp(self.re)
        except OverflowError:
            return C(float('inf'), 0.0)
        return C(ea * math.cos(self.im), ea * math.sin(self.im))

    def ln(self) -> Optional[C]:
        r = self.abs()
        if r == 0.0:
            return None
        return C(math.log(r), self.arg())

    def sqrt(self) -> C:
        r = self.abs()
        t = self.arg() / 2.0
        m = math.sqrt(r)
        return C(m * math.cos(t), m * math.sin(t))

    def sin(self) -> C:
        try:
            return C(
                math.sin(self.re) * math.cosh(self.im),
                math.cos(self.re) * math.sinh(self.im),
            )
        except OverflowError:
            return C(float('inf'), float('inf'))

    def cos(self) -> C:
        try:
            return C(
                math.cos(self.re) * math.cosh(self.im),
                -math.sin(self.re) * math.sinh(self.im),
            )
        except OverflowError:
            return C(float('inf'), float('inf'))

    def tan(self) -> Optional[C]:
        return self.sin().div(self.cos())

    def sinh(self) -> C:
        try:
            return C(
                math.sinh(self.re) * math.cos(self.im),
                math.cosh(self.re) * math.sin(self.im),
            )
        except OverflowError:
            return C(float('inf'), float('inf'))

    def cosh(self) -> C:
        try:
            return C(
                math.cosh(self.re) * math.cos(self.im),
                math.sinh(self.re) * math.sin(self.im),
            )
        except OverflowError:
            return C(float('inf'), float('inf'))

    def tanh(self) -> Optional[C]:
        return self.sinh().div(self.cosh())

    def pow(self, w: C) -> Optional[C]:
        if self.re == 0.0 and self.im == 0.0:
            if w.re == 0.0 and w.im == 0.0:
                return C.real(1.0)
            if w.im == 0.0 and w.re > 0.0:
                return C.real(0.0)
            return None
        l = self.ln()
        if l is None:
            return None
        result = w.mul(l).exp()
        if not result.is_finite():
            return None
        return result

    def asin(self) -> Optional[C]:
        i = C.i()
        one = C.real(1.0)
        iz = i.mul(self)
        inside = one.sub(self.mul(self)).sqrt()
        ln_arg = iz.add(inside)
        ln_val = ln_arg.ln()
        if ln_val is None:
            return None
        return i.neg().mul(ln_val)

    def acos(self) -> Optional[C]:
        v = self.asin()
        if v is None:
            return None
        return C.real(math.pi / 2.0).sub(v)

    def atan(self) -> Optional[C]:
        i = C.i()
        one = C.real(1.0)
        l1 = one.sub(i.mul(self)).ln()
        l2 = one.add(i.mul(self)).ln()
        if l1 is None or l2 is None:
            return None
        diff = l1.sub(l2)
        return C(0.0, 0.5).mul(diff)

    def asinh(self) -> Optional[C]:
        one = C.real(1.0)
        return self.add(self.mul(self).add(one).sqrt()).ln()

    def acosh(self) -> Optional[C]:
        one = C.real(1.0)
        term = self.add(one).sqrt().mul(self.sub(one).sqrt())
        return self.add(term).ln()

    def atanh(self) -> Optional[C]:
        one = C.real(1.0)
        num = one.add(self).ln()
        den = one.sub(self).ln()
        if num is None or den is None:
            return None
        return num.sub(den).mul(C.real(0.5))

    def __repr__(self) -> str:
        if self.im == 0.0:
            return f"C({self.re})"
        return f"C({self.re}, {self.im})"

    def to_complex(self) -> complex:
        return complex(self.re, self.im)


# --- Numeric comparison utilities ---

def qkey(v: C) -> tuple[int, int]:
    """Quantize to 1e-12 grid for dedup."""
    try:
        return (round(v.re * 1e12), round(v.im * 1e12))
    except (OverflowError, ValueError):
        # inf/nan values — use a sentinel that won't collide
        re_k = 9999999999999 if v.re > 0 else -9999999999999 if v.re < 0 else 0
        im_k = 9999999999999 if v.im > 0 else -9999999999999 if v.im < 0 else 0
        return (re_k, im_k)


def near(a: C, b: C, eps: float = 16 * 2.220446049250313e-16) -> bool:
    """Relative equivalence check matching author's implementation."""
    return a.sub(b).abs() <= eps * (1.0 + a.abs() + b.abs())


def value_ok(v: C, real_only: bool = False, eps: float = 16 * 2.220446049250313e-16) -> bool:
    if not v.is_finite():
        return False
    if real_only:
        return abs(v.im) <= eps * (1.0 + abs(v.re) + abs(v.im))
    return True


# --- Transcendental test constants ---

EULER_GAMMA = 0.5772156649015329
GLAISHER = 1.2824271291006227
CATALAN = 0.9159655941772190
KHINCHIN = 2.6854520010653064
