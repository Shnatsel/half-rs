#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::{
    cmp::Ordering,
    fmt::{Debug, Display, Error, Formatter, LowerExp, UpperExp},
    num::{FpCategory, ParseFloatError},
    str::FromStr,
};

pub(crate) mod convert;

/// A 16-bit floating point type implementing the IEEE 754-2008 standard [`binary16`] a.k.a `half`
/// format.
///
/// This 16-bit floating point type is intended for efficient storage where the full range and
/// precision of a larger floating point value is not required. Because [`f16`] is primarily for
/// efficient storage, floating point operations such as addition, multiplication, etc. are not
/// implemented. Operations should be performed with `f32` or higher-precision types and converted
/// to/from [`f16`] as necessary.
///
/// [`f16`]: struct.f16.html
/// [`binary16`]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct f16(u16);

#[deprecated(
    since = "1.4.0",
    note = "all constants moved to associated constants of [`f16`](../struct.f16.html)"
)]
pub mod consts {
    //! Useful `f16` constants.

    use super::f16;

    /// Approximate number of [`f16`](../struct.f16.html) significant digits in base 10.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::DIGITS`](../struct.f16.html#associatedconstant.DIGITS)"
    )]
    pub const DIGITS: u32 = f16::DIGITS;
    /// [`f16`](../struct.f16.html)
    /// [machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon) value.
    ///
    /// This is the difference between 1.0 and the next largest representable number.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::EPSILON`](../struct.f16.html#associatedconstant.EPSILON)"
    )]
    pub const EPSILON: f16 = f16::EPSILON;
    /// [`f16`](../struct.f16.html) positive Infinity (+∞).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::INFINITY`](../struct.f16.html#associatedconstant.INFINITY)"
    )]
    pub const INFINITY: f16 = f16::INFINITY;
    /// Number of [`f16`](../struct.f16.html) significant digits in base 2.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MANTISSA_DIGITS`](../struct.f16.html#associatedconstant.MANTISSA_DIGITS)"
    )]
    pub const MANTISSA_DIGITS: u32 = f16::MANTISSA_DIGITS;
    /// Largest finite [`f16`](../struct.f16.html) value.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MAX`](../struct.f16.html#associatedconstant.MAX)"
    )]
    pub const MAX: f16 = f16::MAX;
    /// Maximum possible [`f16`](../struct.f16.html) power of 10 exponent.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MAX_10_EXP`](../struct.f16.html#associatedconstant.MAX_10_EXP)"
    )]
    pub const MAX_10_EXP: i32 = f16::MAX_10_EXP;
    /// Maximum possible [`f16`](../struct.f16.html) power of 2 exponent.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MAX_EXP`](../struct.f16.html#associatedconstant.MAX_EXP)"
    )]
    pub const MAX_EXP: i32 = f16::MAX_EXP;
    /// Smallest finite [`f16`](../struct.f16.html) value.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MIN`](../struct.f16.html#associatedconstant.MIN)"
    )]
    pub const MIN: f16 = f16::MIN;
    /// Minimum possible normal [`f16`](../struct.f16.html) power of 10 exponent.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MIN_10_EXP`](../struct.f16.html#associatedconstant.MIN_10_EXP)"
    )]
    pub const MIN_10_EXP: i32 = f16::MIN_10_EXP;
    /// One greater than the minimum possible normal [`f16`](../struct.f16.html) power of 2 exponent.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MIN_EXP`](../struct.f16.html#associatedconstant.MIN_EXP)"
    )]
    pub const MIN_EXP: i32 = f16::MIN_EXP;
    /// Smallest positive normal [`f16`](../struct.f16.html) value.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MIN_POSITIVE`](../struct.f16.html#associatedconstant.MIN_POSITIVE)"
    )]
    pub const MIN_POSITIVE: f16 = f16::MIN_POSITIVE;
    /// [`f16`](../struct.f16.html) Not a Number (NaN).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::NAN`](../struct.f16.html#associatedconstant.NAN)"
    )]
    pub const NAN: f16 = f16::NAN;
    /// [`f16`](../struct.f16.html) negative infinity (-∞).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::NEG_INFINITY`](../struct.f16.html#associatedconstant.NEG_INFINITY)"
    )]
    pub const NEG_INFINITY: f16 = f16::NEG_INFINITY;
    /// The radix or base of the internal representation of [`f16`](../struct.f16.html).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::RADIX`](../struct.f16.html#associatedconstant.RADIX)"
    )]
    pub const RADIX: u32 = f16::RADIX;

    /// Minimum positive subnormal [`f16`](../struct.f16.html) value.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MIN_POSITIVE_SUBNORMAL`](../struct.f16.html#associatedconstant.MIN_POSITIVE_SUBNORMAL)"
    )]
    pub const MIN_POSITIVE_SUBNORMAL: f16 = f16::MIN_POSITIVE_SUBNORMAL;
    /// Maximum subnormal [`f16`](../struct.f16.html) value.
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::MAX_SUBNORMAL`](../struct.f16.html#associatedconstant.MAX_SUBNORMAL)"
    )]
    pub const MAX_SUBNORMAL: f16 = f16::MAX_SUBNORMAL;

    /// [`f16`](../struct.f16.html) 1
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::ONE`](../struct.f16.html#associatedconstant.ONE)"
    )]
    pub const ONE: f16 = f16::ONE;
    /// [`f16`](../struct.f16.html) 0
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::ZERO`](../struct.f16.html#associatedconstant.ZERO)"
    )]
    pub const ZERO: f16 = f16::ZERO;
    /// [`f16`](../struct.f16.html) -0
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::NEG_ZERO`](../struct.f16.html#associatedconstant.NEG_ZERO)"
    )]
    pub const NEG_ZERO: f16 = f16::NEG_ZERO;

    /// [`f16`](../struct.f16.html) Euler's number (ℯ).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::E`](../struct.f16.html#associatedconstant.E)"
    )]
    pub const E: f16 = f16::E;
    /// [`f16`](../struct.f16.html) Archimedes' constant (π).
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::PI`](../struct.f16.html#associatedconstant.PI)"
    )]
    pub const PI: f16 = f16::PI;
    /// [`f16`](../struct.f16.html) 1/π
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_1_PI`](../struct.f16.html#associatedconstant.FRAC_1_PI)"
    )]
    pub const FRAC_1_PI: f16 = f16::FRAC_1_PI;
    /// [`f16`](../struct.f16.html) 1/√2
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_1_SQRT_2`](../struct.f16.html#associatedconstant.FRAC_1_SQRT_2)"
    )]
    pub const FRAC_1_SQRT_2: f16 = f16::FRAC_1_SQRT_2;
    /// [`f16`](../struct.f16.html) 2/π
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_2_PI`](../struct.f16.html#associatedconstant.FRAC_2_PI)"
    )]
    pub const FRAC_2_PI: f16 = f16::FRAC_2_PI;
    /// [`f16`](../struct.f16.html) 2/√π
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_2_SQRT_PI`](../struct.f16.html#associatedconstant.FRAC_2_SQRT_PI)"
    )]
    pub const FRAC_2_SQRT_PI: f16 = f16::FRAC_2_SQRT_PI;
    /// [`f16`](../struct.f16.html) π/2
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_PI_2`](../struct.f16.html#associatedconstant.FRAC_PI_2)"
    )]
    pub const FRAC_PI_2: f16 = f16::FRAC_PI_2;
    /// [`f16`](../struct.f16.html) π/3
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_PI_3`](../struct.f16.html#associatedconstant.FRAC_PI_3)"
    )]
    pub const FRAC_PI_3: f16 = f16::FRAC_PI_3;
    /// [`f16`](../struct.f16.html) π/4
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_PI_4`](../struct.f16.html#associatedconstant.FRAC_PI_4)"
    )]
    pub const FRAC_PI_4: f16 = f16::FRAC_PI_4;
    /// [`f16`](../struct.f16.html) π/6
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_PI_6`](../struct.f16.html#associatedconstant.FRAC_PI_6)"
    )]
    pub const FRAC_PI_6: f16 = f16::FRAC_PI_6;
    /// [`f16`](../struct.f16.html) π/8
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::FRAC_PI_8`](../struct.f16.html#associatedconstant.FRAC_PI_8)"
    )]
    pub const FRAC_PI_8: f16 = f16::FRAC_PI_8;
    /// [`f16`](../struct.f16.html) 𝗅𝗇 10
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::LN_10`](../struct.f16.html#associatedconstant.LN_10)"
    )]
    pub const LN_10: f16 = f16::LN_10;
    /// [`f16`](../struct.f16.html) 𝗅𝗇 2
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::LN_2`](../struct.f16.html#associatedconstant.LN_2)"
    )]
    pub const LN_2: f16 = f16::LN_2;
    /// [`f16`](../struct.f16.html) 𝗅𝗈𝗀₁₀ℯ
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::LOG10_E`](../struct.f16.html#associatedconstant.LOG10_E)"
    )]
    pub const LOG10_E: f16 = f16::LOG10_E;
    /// [`f16`](../struct.f16.html) 𝗅𝗈𝗀₂ℯ
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::LOG2_E`](../struct.f16.html#associatedconstant.LOG2_E)"
    )]
    pub const LOG2_E: f16 = f16::LOG2_E;
    /// [`f16`](../struct.f16.html) √2
    #[deprecated(
        since = "1.4.0",
        note = "moved to [`f16::SQRT_2`](../struct.f16.html#associatedconstant.SQRT_2)"
    )]
    pub const SQRT_2: f16 = f16::SQRT_2;
}

impl f16 {
    /// Constructs a 16-bit floating point value from the raw bits.
    #[inline]
    pub const fn from_bits(bits: u16) -> f16 {
        f16(bits)
    }

    /// Constructs a 16-bit floating point value from a 32-bit floating point value.
    ///
    /// If the 32-bit value is to large to fit in 16-bits, ±∞ will result. NaN values are
    /// preserved. 32-bit subnormal values are too tiny to be represented in 16-bits and result in
    /// ±0. Exponents that underflow the minimum 16-bit exponent will result in 16-bit subnormals
    /// or ±0. All other values are truncated and rounded to the nearest representable 16-bit
    /// value.
    #[inline]
    pub fn from_f32(value: f32) -> f16 {
        f16(convert::f32_to_f16(value))
    }

    /// Constructs a 16-bit floating point value from a 64-bit floating point value.
    ///
    /// If the 64-bit value is to large to fit in 16-bits, ±∞ will result. NaN values are
    /// preserved. 64-bit subnormal values are too tiny to be represented in 16-bits and result in
    /// ±0. Exponents that underflow the minimum 16-bit exponent will result in 16-bit subnormals
    /// or ±0. All other values are truncated and rounded to the nearest representable 16-bit
    /// value.
    #[inline]
    pub fn from_f64(value: f64) -> f16 {
        f16(convert::f64_to_f16(value))
    }

    /// Converts a [`f16`](struct.f16.html) into the underlying bit representation.
    #[inline]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Converts a [`f16`](struct.f16.html) into the underlying bit representation.
    #[deprecated(since = "1.2.0", note = "renamed to [`to_bits`](#method.to_bits)")]
    #[inline]
    pub fn as_bits(self) -> u16 {
        self.to_bits()
    }

    /// Converts a [`f16`](struct.f16.html) value into a `f32` value.
    ///
    /// This conversion is lossless as all 16-bit floating point values can be represented exactly
    /// in 32-bit floating point.
    #[inline]
    pub fn to_f32(self) -> f32 {
        convert::f16_to_f32(self.0)
    }

    /// Converts a [`f16`](struct.f16.html) value into a `f64` value.
    ///
    /// This conversion is lossless as all 16-bit floating point values can be represented exactly
    /// in 64-bit floating point.
    #[inline]
    pub fn to_f64(self) -> f64 {
        convert::f16_to_f64(self.0)
    }

    /// Returns `true` if this value is `NaN` and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let nan = f16::NAN;
    /// let f = f16::from_f32(7.0_f32);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.0 & 0x7FFFu16 > 0x7C00u16
    }

    /// Returns `true` if this value is ±∞ and `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let f = f16::from_f32(7.0f32);
    /// let inf = f16::INFINITY;
    /// let neg_inf = f16::NEG_INFINITY;
    /// let nan = f16::NAN;
    ///
    /// assert!(!f.is_infinite());
    /// assert!(!nan.is_infinite());
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// ```
    #[inline]
    pub const fn is_infinite(self) -> bool {
        self.0 & 0x7FFFu16 == 0x7C00u16
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let f = f16::from_f32(7.0f32);
    /// let inf = f16::INFINITY;
    /// let neg_inf = f16::NEG_INFINITY;
    /// let nan = f16::NAN;
    ///
    /// assert!(f.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.0 & 0x7C00u16 != 0x7C00u16
    }

    /// Returns `true` if the number is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let min = f16::MIN_POSITIVE;
    /// let max = f16::MAX;
    /// let lower_than_min = f16::from_f32(1.0e-10_f32);
    /// let zero = f16::from_f32(0.0_f32);
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!f16::NAN.is_normal());
    /// assert!(!f16::INFINITY.is_normal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(!lower_than_min.is_normal());
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        let exp = self.0 & 0x7C00u16;
        exp != 0x7C00u16 && exp != 0
    }

    /// Returns the floating point category of the number.
    ///
    /// If only one property is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::num::FpCategory;
    /// # use half::prelude::*;
    ///
    /// let num = f16::from_f32(12.4_f32);
    /// let inf = f16::INFINITY;
    ///
    /// assert_eq!(num.classify(), FpCategory::Normal);
    /// assert_eq!(inf.classify(), FpCategory::Infinite);
    /// ```
    pub fn classify(self) -> FpCategory {
        let exp = self.0 & 0x7C00u16;
        let man = self.0 & 0x03FFu16;
        match (exp, man) {
            (0, 0) => FpCategory::Zero,
            (0, _) => FpCategory::Subnormal,
            (0x7C00u16, 0) => FpCategory::Infinite,
            (0x7C00u16, _) => FpCategory::Nan,
            _ => FpCategory::Normal,
        }
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * `NAN` if the number is `NAN`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let f = f16::from_f32(3.5_f32);
    ///
    /// assert_eq!(f.signum(), f16::from_f32(1.0));
    /// assert_eq!(f16::NEG_INFINITY.signum(), f16::from_f32(-1.0));
    ///
    /// assert!(f16::NAN.signum().is_nan());
    /// ```
    pub fn signum(self) -> f16 {
        if self.is_nan() {
            self
        } else if self.0 & 0x8000u16 != 0 {
            f16::from_f32(-1.0)
        } else {
            f16::from_f32(1.0)
        }
    }

    /// Returns `true` if and only if `self` has a positive sign, including `+0.0`, `NaNs` with a
    /// positive sign bit and +∞.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let nan = f16::NAN;
    /// let f = f16::from_f32(7.0_f32);
    /// let g = f16::from_f32(-7.0_f32);
    ///
    /// assert!(f.is_sign_positive());
    /// assert!(!g.is_sign_positive());
    /// // `NaN` can be either positive or negative
    /// assert!(nan.is_sign_positive() != nan.is_sign_negative());
    /// ```
    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        self.0 & 0x8000u16 == 0
    }

    /// Returns `true` if and only if `self` has a negative sign, including `-0.0`, `NaNs` with a
    /// negative sign bit and −∞.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use half::prelude::*;
    ///
    /// let nan = f16::NAN;
    /// let f = f16::from_f32(7.0f32);
    /// let g = f16::from_f32(-7.0f32);
    ///
    /// assert!(!f.is_sign_negative());
    /// assert!(g.is_sign_negative());
    /// // `NaN` can be either positive or negative
    /// assert!(nan.is_sign_positive() != nan.is_sign_negative());
    /// ```
    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        self.0 & 0x8000u16 != 0
    }

    /// Approximate number of [`f16`](struct.f16.html) significant digits in base 10.
    pub const DIGITS: u32 = 3;
    /// [`f16`](struct.f16.html)
    /// [machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon) value.
    ///
    /// This is the difference between 1.0 and the next largest representable number.
    pub const EPSILON: f16 = f16(0x1400u16);
    /// [`f16`](struct.f16.html) positive Infinity (+∞).
    pub const INFINITY: f16 = f16(0x7C00u16);
    /// Number of [`f16`](struct.f16.html) significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 11;
    /// Largest finite [`f16`](struct.f16.html) value.
    pub const MAX: f16 = f16(0x7BFF);
    /// Maximum possible [`f16`](struct.f16.html) power of 10 exponent.
    pub const MAX_10_EXP: i32 = 4;
    /// Maximum possible [`f16`](struct.f16.html) power of 2 exponent.
    pub const MAX_EXP: i32 = 16;
    /// Smallest finite [`f16`](struct.f16.html) value.
    pub const MIN: f16 = f16(0xFBFF);
    /// Minimum possible normal [`f16`](struct.f16.html) power of 10 exponent.
    pub const MIN_10_EXP: i32 = -4;
    /// One greater than the minimum possible normal [`f16`](struct.f16.html) power of 2 exponent.
    pub const MIN_EXP: i32 = -13;
    /// Smallest positive normal [`f16`](struct.f16.html) value.
    pub const MIN_POSITIVE: f16 = f16(0x0400u16);
    /// [`f16`](struct.f16.html) Not a Number (NaN).
    pub const NAN: f16 = f16(0x7E00u16);
    /// [`f16`](struct.f16.html) negative infinity (-∞).
    pub const NEG_INFINITY: f16 = f16(0xFC00u16);
    /// The radix or base of the internal representation of [`f16`](struct.f16.html).
    pub const RADIX: u32 = 2;

    /// Minimum positive subnormal [`f16`](struct.f16.html) value.
    pub const MIN_POSITIVE_SUBNORMAL: f16 = f16(0x0001u16);
    /// Maximum subnormal [`f16`](struct.f16.html) value.
    pub const MAX_SUBNORMAL: f16 = f16(0x03FFu16);

    /// [`f16`](struct.f16.html) 1
    pub const ONE: f16 = f16(0x3C00u16);
    /// [`f16`](struct.f16.html) 0
    pub const ZERO: f16 = f16(0x0000u16);
    /// [`f16`](struct.f16.html) -0
    pub const NEG_ZERO: f16 = f16(0x8000u16);

    /// [`f16`](struct.f16.html) Euler's number (ℯ).
    pub const E: f16 = f16(0x4170u16);
    /// [`f16`](struct.f16.html) Archimedes' constant (π).
    pub const PI: f16 = f16(0x4248u16);
    /// [`f16`](struct.f16.html) 1/π
    pub const FRAC_1_PI: f16 = f16(0x3518u16);
    /// [`f16`](struct.f16.html) 1/√2
    pub const FRAC_1_SQRT_2: f16 = f16(0x39A8u16);
    /// [`f16`](struct.f16.html) 2/π
    pub const FRAC_2_PI: f16 = f16(0x3918u16);
    /// [`f16`](struct.f16.html) 2/√π
    pub const FRAC_2_SQRT_PI: f16 = f16(0x3C83u16);
    /// [`f16`](struct.f16.html) π/2
    pub const FRAC_PI_2: f16 = f16(0x3E48u16);
    /// [`f16`](struct.f16.html) π/3
    pub const FRAC_PI_3: f16 = f16(0x3C30u16);
    /// [`f16`](struct.f16.html) π/4
    pub const FRAC_PI_4: f16 = f16(0x3A48u16);
    /// [`f16`](struct.f16.html) π/6
    pub const FRAC_PI_6: f16 = f16(0x3830u16);
    /// [`f16`](struct.f16.html) π/8
    pub const FRAC_PI_8: f16 = f16(0x3648u16);
    /// [`f16`](struct.f16.html) 𝗅𝗇 10
    pub const LN_10: f16 = f16(0x409Bu16);
    /// [`f16`](struct.f16.html) 𝗅𝗇 2
    pub const LN_2: f16 = f16(0x398Cu16);
    /// [`f16`](struct.f16.html) 𝗅𝗈𝗀₁₀ℯ
    pub const LOG10_E: f16 = f16(0x36F3u16);
    /// [`f16`](struct.f16.html) 𝗅𝗈𝗀₁₀2
    pub const LOG10_2: f16 = f16(0x34D1u16);
    /// [`f16`](struct.f16.html) 𝗅𝗈𝗀₂ℯ
    pub const LOG2_E: f16 = f16(0x3DC5u16);
    /// [`f16`](struct.f16.html) 𝗅𝗈𝗀₂10
    pub const LOG2_10: f16 = f16(0x42A5u16);
    /// [`f16`](struct.f16.html) √2
    pub const SQRT_2: f16 = f16(0x3DA8u16);
}

impl From<f16> for f32 {
    #[inline]
    fn from(x: f16) -> f32 {
        x.to_f32()
    }
}

impl From<f16> for f64 {
    #[inline]
    fn from(x: f16) -> f64 {
        x.to_f64()
    }
}

impl From<i8> for f16 {
    #[inline]
    fn from(x: i8) -> f16 {
        // Convert to f32, then to f16
        f16::from_f32(f32::from(x))
    }
}

impl From<u8> for f16 {
    #[inline]
    fn from(x: u8) -> f16 {
        // Convert to f32, then to f16
        f16::from_f32(f32::from(x))
    }
}

impl PartialEq for f16 {
    fn eq(&self, other: &f16) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            (self.0 == other.0) || ((self.0 | other.0) & 0x7FFFu16 == 0)
        }
    }
}

impl PartialOrd for f16 {
    fn partial_cmp(&self, other: &f16) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else {
            let neg = self.0 & 0x8000u16 != 0;
            let other_neg = other.0 & 0x8000u16 != 0;
            match (neg, other_neg) {
                (false, false) => Some(self.0.cmp(&other.0)),
                (false, true) => {
                    if (self.0 | other.0) & 0x7FFFu16 == 0 {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                (true, false) => {
                    if (self.0 | other.0) & 0x7FFFu16 == 0 {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                }
                (true, true) => Some(other.0.cmp(&self.0)),
            }
        }
    }

    fn lt(&self, other: &f16) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            let neg = self.0 & 0x8000u16 != 0;
            let other_neg = other.0 & 0x8000u16 != 0;
            match (neg, other_neg) {
                (false, false) => self.0 < other.0,
                (false, true) => false,
                (true, false) => (self.0 | other.0) & 0x7FFFu16 != 0,
                (true, true) => self.0 > other.0,
            }
        }
    }

    fn le(&self, other: &f16) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            let neg = self.0 & 0x8000u16 != 0;
            let other_neg = other.0 & 0x8000u16 != 0;
            match (neg, other_neg) {
                (false, false) => self.0 <= other.0,
                (false, true) => (self.0 | other.0) & 0x7FFFu16 == 0,
                (true, false) => true,
                (true, true) => self.0 >= other.0,
            }
        }
    }

    fn gt(&self, other: &f16) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            let neg = self.0 & 0x8000u16 != 0;
            let other_neg = other.0 & 0x8000u16 != 0;
            match (neg, other_neg) {
                (false, false) => self.0 > other.0,
                (false, true) => (self.0 | other.0) & 0x7FFFu16 != 0,
                (true, false) => false,
                (true, true) => self.0 < other.0,
            }
        }
    }

    fn ge(&self, other: &f16) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            let neg = self.0 & 0x8000u16 != 0;
            let other_neg = other.0 & 0x8000u16 != 0;
            match (neg, other_neg) {
                (false, false) => self.0 >= other.0,
                (false, true) => true,
                (true, false) => (self.0 | other.0) & 0x7FFFu16 == 0,
                (true, true) => self.0 <= other.0,
            }
        }
    }
}

impl FromStr for f16 {
    type Err = ParseFloatError;
    fn from_str(src: &str) -> Result<f16, ParseFloatError> {
        f32::from_str(src).map(f16::from_f32)
    }
}

impl Debug for f16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "0x{:X}", self.0)
    }
}

impl Display for f16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_f32())
    }
}

impl LowerExp for f16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:e}", self.to_f32())
    }
}

impl UpperExp for f16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:E}", self.to_f32())
    }
}

#[allow(
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::neg_cmp_op_on_partial_ord
)]
#[cfg(test)]
mod test {
    use super::*;
    use core;
    use core::cmp::Ordering;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_f16_consts() {
        // DIGITS
        let digits = ((f16::MANTISSA_DIGITS as f32 - 1.0) * 2f32.log10()).floor() as u32;
        assert_eq!(f16::DIGITS, digits);
        // sanity check to show test is good
        let digits32 = ((core::f32::MANTISSA_DIGITS as f32 - 1.0) * 2f32.log10()).floor() as u32;
        assert_eq!(core::f32::DIGITS, digits32);

        // EPSILON
        let one = f16::from_f32(1.0);
        let one_plus_epsilon = f16::from_bits(one.to_bits() + 1);
        let epsilon = f16::from_f32(one_plus_epsilon.to_f32() - 1.0);
        assert_eq!(f16::EPSILON, epsilon);
        // sanity check to show test is good
        let one_plus_epsilon32 = f32::from_bits(1.0f32.to_bits() + 1);
        let epsilon32 = one_plus_epsilon32 - 1f32;
        assert_eq!(core::f32::EPSILON, epsilon32);

        // MAX, MIN and MIN_POSITIVE
        let max = f16::from_bits(f16::INFINITY.to_bits() - 1);
        let min = f16::from_bits(f16::NEG_INFINITY.to_bits() - 1);
        let min_pos = f16::from_f32(2f32.powi(f16::MIN_EXP - 1));
        assert_eq!(f16::MAX, max);
        assert_eq!(f16::MIN, min);
        assert_eq!(f16::MIN_POSITIVE, min_pos);
        // sanity check to show test is good
        let max32 = f32::from_bits(core::f32::INFINITY.to_bits() - 1);
        let min32 = f32::from_bits(core::f32::NEG_INFINITY.to_bits() - 1);
        let min_pos32 = 2f32.powi(core::f32::MIN_EXP - 1);
        assert_eq!(core::f32::MAX, max32);
        assert_eq!(core::f32::MIN, min32);
        assert_eq!(core::f32::MIN_POSITIVE, min_pos32);

        // MIN_10_EXP and MAX_10_EXP
        let ten_to_min = 10f32.powi(f16::MIN_10_EXP);
        assert!(ten_to_min / 10.0 < f16::MIN_POSITIVE.to_f32());
        assert!(ten_to_min > f16::MIN_POSITIVE.to_f32());
        let ten_to_max = 10f32.powi(f16::MAX_10_EXP);
        assert!(ten_to_max < f16::MAX.to_f32());
        assert!(ten_to_max * 10.0 > f16::MAX.to_f32());
        // sanity check to show test is good
        let ten_to_min32 = 10f64.powi(core::f32::MIN_10_EXP);
        assert!(ten_to_min32 / 10.0 < f64::from(core::f32::MIN_POSITIVE));
        assert!(ten_to_min32 > f64::from(core::f32::MIN_POSITIVE));
        let ten_to_max32 = 10f64.powi(core::f32::MAX_10_EXP);
        assert!(ten_to_max32 < f64::from(core::f32::MAX));
        assert!(ten_to_max32 * 10.0 > f64::from(core::f32::MAX));
    }

    #[test]
    fn test_f16_consts_from_f32() {
        let one = f16::from_f32(1.0);
        let zero = f16::from_f32(0.0);
        let neg_zero = f16::from_f32(-0.0);
        let inf = f16::from_f32(core::f32::INFINITY);
        let neg_inf = f16::from_f32(core::f32::NEG_INFINITY);
        let nan = f16::from_f32(core::f32::NAN);

        assert_eq!(f16::ONE, one);
        assert_eq!(f16::ZERO, zero);
        assert!(zero.is_sign_positive());
        assert_eq!(f16::NEG_ZERO, neg_zero);
        assert!(neg_zero.is_sign_negative());
        assert_eq!(f16::INFINITY, inf);
        assert_eq!(f16::NEG_INFINITY, neg_inf);
        assert!(nan.is_nan());
        assert!(f16::NAN.is_nan());

        let e = f16::from_f32(core::f32::consts::E);
        let pi = f16::from_f32(core::f32::consts::PI);
        let frac_1_pi = f16::from_f32(core::f32::consts::FRAC_1_PI);
        let frac_1_sqrt_2 = f16::from_f32(core::f32::consts::FRAC_1_SQRT_2);
        let frac_2_pi = f16::from_f32(core::f32::consts::FRAC_2_PI);
        let frac_2_sqrt_pi = f16::from_f32(core::f32::consts::FRAC_2_SQRT_PI);
        let frac_pi_2 = f16::from_f32(core::f32::consts::FRAC_PI_2);
        let frac_pi_3 = f16::from_f32(core::f32::consts::FRAC_PI_3);
        let frac_pi_4 = f16::from_f32(core::f32::consts::FRAC_PI_4);
        let frac_pi_6 = f16::from_f32(core::f32::consts::FRAC_PI_6);
        let frac_pi_8 = f16::from_f32(core::f32::consts::FRAC_PI_8);
        let ln_10 = f16::from_f32(core::f32::consts::LN_10);
        let ln_2 = f16::from_f32(core::f32::consts::LN_2);
        let log10_e = f16::from_f32(core::f32::consts::LOG10_E);
        // core::f32::consts::LOG10_2 requires rustc 1.43.0
        let log10_2 = f16::from_f32(2f32.log10());
        let log2_e = f16::from_f32(core::f32::consts::LOG2_E);
        // core::f32::consts::LOG2_10 requires rustc 1.43.0
        let log2_10 = f16::from_f32(10f32.log2());
        let sqrt_2 = f16::from_f32(core::f32::consts::SQRT_2);

        assert_eq!(f16::E, e);
        assert_eq!(f16::PI, pi);
        assert_eq!(f16::FRAC_1_PI, frac_1_pi);
        assert_eq!(f16::FRAC_1_SQRT_2, frac_1_sqrt_2);
        assert_eq!(f16::FRAC_2_PI, frac_2_pi);
        assert_eq!(f16::FRAC_2_SQRT_PI, frac_2_sqrt_pi);
        assert_eq!(f16::FRAC_PI_2, frac_pi_2);
        assert_eq!(f16::FRAC_PI_3, frac_pi_3);
        assert_eq!(f16::FRAC_PI_4, frac_pi_4);
        assert_eq!(f16::FRAC_PI_6, frac_pi_6);
        assert_eq!(f16::FRAC_PI_8, frac_pi_8);
        assert_eq!(f16::LN_10, ln_10);
        assert_eq!(f16::LN_2, ln_2);
        assert_eq!(f16::LOG10_E, log10_e);
        assert_eq!(f16::LOG10_2, log10_2);
        assert_eq!(f16::LOG2_E, log2_e);
        assert_eq!(f16::LOG2_10, log2_10);
        assert_eq!(f16::SQRT_2, sqrt_2);
    }

    #[test]
    fn test_f16_consts_from_f64() {
        let one = f16::from_f64(1.0);
        let zero = f16::from_f64(0.0);
        let neg_zero = f16::from_f64(-0.0);
        let inf = f16::from_f64(core::f64::INFINITY);
        let neg_inf = f16::from_f64(core::f64::NEG_INFINITY);
        let nan = f16::from_f64(core::f64::NAN);

        assert_eq!(f16::ONE, one);
        assert_eq!(f16::ZERO, zero);
        assert!(zero.is_sign_positive());
        assert_eq!(f16::NEG_ZERO, neg_zero);
        assert!(neg_zero.is_sign_negative());
        assert_eq!(f16::INFINITY, inf);
        assert_eq!(f16::NEG_INFINITY, neg_inf);
        assert!(nan.is_nan());
        assert!(f16::NAN.is_nan());

        let e = f16::from_f64(core::f64::consts::E);
        let pi = f16::from_f64(core::f64::consts::PI);
        let frac_1_pi = f16::from_f64(core::f64::consts::FRAC_1_PI);
        let frac_1_sqrt_2 = f16::from_f64(core::f64::consts::FRAC_1_SQRT_2);
        let frac_2_pi = f16::from_f64(core::f64::consts::FRAC_2_PI);
        let frac_2_sqrt_pi = f16::from_f64(core::f64::consts::FRAC_2_SQRT_PI);
        let frac_pi_2 = f16::from_f64(core::f64::consts::FRAC_PI_2);
        let frac_pi_3 = f16::from_f64(core::f64::consts::FRAC_PI_3);
        let frac_pi_4 = f16::from_f64(core::f64::consts::FRAC_PI_4);
        let frac_pi_6 = f16::from_f64(core::f64::consts::FRAC_PI_6);
        let frac_pi_8 = f16::from_f64(core::f64::consts::FRAC_PI_8);
        let ln_10 = f16::from_f64(core::f64::consts::LN_10);
        let ln_2 = f16::from_f64(core::f64::consts::LN_2);
        let log10_e = f16::from_f64(core::f64::consts::LOG10_E);
        // core::f64::consts::LOG10_2 requires rustc 1.43.0
        let log10_2 = f16::from_f64(2f64.log10());
        let log2_e = f16::from_f64(core::f64::consts::LOG2_E);
        // core::f64::consts::LOG2_10 requires rustc 1.43.0
        let log2_10 = f16::from_f64(10f64.log2());
        let sqrt_2 = f16::from_f64(core::f64::consts::SQRT_2);

        assert_eq!(f16::E, e);
        assert_eq!(f16::PI, pi);
        assert_eq!(f16::FRAC_1_PI, frac_1_pi);
        assert_eq!(f16::FRAC_1_SQRT_2, frac_1_sqrt_2);
        assert_eq!(f16::FRAC_2_PI, frac_2_pi);
        assert_eq!(f16::FRAC_2_SQRT_PI, frac_2_sqrt_pi);
        assert_eq!(f16::FRAC_PI_2, frac_pi_2);
        assert_eq!(f16::FRAC_PI_3, frac_pi_3);
        assert_eq!(f16::FRAC_PI_4, frac_pi_4);
        assert_eq!(f16::FRAC_PI_6, frac_pi_6);
        assert_eq!(f16::FRAC_PI_8, frac_pi_8);
        assert_eq!(f16::LN_10, ln_10);
        assert_eq!(f16::LN_2, ln_2);
        assert_eq!(f16::LOG10_E, log10_e);
        assert_eq!(f16::LOG10_2, log10_2);
        assert_eq!(f16::LOG2_E, log2_e);
        assert_eq!(f16::LOG2_10, log2_10);
        assert_eq!(f16::SQRT_2, sqrt_2);
    }

    #[test]
    fn test_nan_conversion_to_smaller() {
        let nan64 = f64::from_bits(0x7FF0_0000_0000_0001u64);
        let neg_nan64 = f64::from_bits(0xFFF0_0000_0000_0001u64);
        let nan32 = f32::from_bits(0x7F80_0001u32);
        let neg_nan32 = f32::from_bits(0xFF80_0001u32);
        let nan32_from_64 = nan64 as f32;
        let neg_nan32_from_64 = neg_nan64 as f32;
        let nan16_from_64 = f16::from_f64(nan64);
        let neg_nan16_from_64 = f16::from_f64(neg_nan64);
        let nan16_from_32 = f16::from_f32(nan32);
        let neg_nan16_from_32 = f16::from_f32(neg_nan32);

        assert!(nan64.is_nan() && nan64.is_sign_positive());
        assert!(neg_nan64.is_nan() && neg_nan64.is_sign_negative());
        assert!(nan32.is_nan() && nan32.is_sign_positive());
        assert!(neg_nan32.is_nan() && neg_nan32.is_sign_negative());
        assert!(nan32_from_64.is_nan() && nan32_from_64.is_sign_positive());
        assert!(neg_nan32_from_64.is_nan() && neg_nan32_from_64.is_sign_negative());
        assert!(nan16_from_64.is_nan() && nan16_from_64.is_sign_positive());
        assert!(neg_nan16_from_64.is_nan() && neg_nan16_from_64.is_sign_negative());
        assert!(nan16_from_32.is_nan() && nan16_from_32.is_sign_positive());
        assert!(neg_nan16_from_32.is_nan() && neg_nan16_from_32.is_sign_negative());
    }

    #[test]
    fn test_nan_conversion_to_larger() {
        let nan16 = f16::from_bits(0x7C01u16);
        let neg_nan16 = f16::from_bits(0xFC01u16);
        let nan32 = f32::from_bits(0x7F80_0001u32);
        let neg_nan32 = f32::from_bits(0xFF80_0001u32);
        let nan32_from_16 = f32::from(nan16);
        let neg_nan32_from_16 = f32::from(neg_nan16);
        let nan64_from_16 = f64::from(nan16);
        let neg_nan64_from_16 = f64::from(neg_nan16);
        let nan64_from_32 = f64::from(nan32);
        let neg_nan64_from_32 = f64::from(neg_nan32);

        assert!(nan16.is_nan() && nan16.is_sign_positive());
        assert!(neg_nan16.is_nan() && neg_nan16.is_sign_negative());
        assert!(nan32.is_nan() && nan32.is_sign_positive());
        assert!(neg_nan32.is_nan() && neg_nan32.is_sign_negative());
        assert!(nan32_from_16.is_nan() && nan32_from_16.is_sign_positive());
        assert!(neg_nan32_from_16.is_nan() && neg_nan32_from_16.is_sign_negative());
        assert!(nan64_from_16.is_nan() && nan64_from_16.is_sign_positive());
        assert!(neg_nan64_from_16.is_nan() && neg_nan64_from_16.is_sign_negative());
        assert!(nan64_from_32.is_nan() && nan64_from_32.is_sign_positive());
        assert!(neg_nan64_from_32.is_nan() && neg_nan64_from_32.is_sign_negative());
    }

    #[test]
    fn test_f16_to_f32() {
        let f = f16::from_f32(7.0);
        assert_eq!(f.to_f32(), 7.0f32);

        // 7.1 is NOT exactly representable in 16-bit, it's rounded
        let f = f16::from_f32(7.1);
        let diff = (f.to_f32() - 7.1f32).abs();
        // diff must be <= 4 * EPSILON, as 7 has two more significant bits than 1
        assert!(diff <= 4.0 * f16::EPSILON.to_f32());

        assert_eq!(f16::from_bits(0x0000_0001).to_f32(), 2.0f32.powi(-24));
        assert_eq!(f16::from_bits(0x0000_0005).to_f32(), 5.0 * 2.0f32.powi(-24));

        assert_eq!(f16::from_bits(0x0000_0001), f16::from_f32(2.0f32.powi(-24)));
        assert_eq!(
            f16::from_bits(0x0000_0005),
            f16::from_f32(5.0 * 2.0f32.powi(-24))
        );
    }

    #[test]
    fn test_f16_to_f64() {
        let f = f16::from_f64(7.0);
        assert_eq!(f.to_f64(), 7.0f64);

        // 7.1 is NOT exactly representable in 16-bit, it's rounded
        let f = f16::from_f64(7.1);
        let diff = (f.to_f64() - 7.1f64).abs();
        // diff must be <= 4 * EPSILON, as 7 has two more significant bits than 1
        assert!(diff <= 4.0 * f16::EPSILON.to_f64());

        assert_eq!(f16::from_bits(0x0000_0001).to_f64(), 2.0f64.powi(-24));
        assert_eq!(f16::from_bits(0x0000_0005).to_f64(), 5.0 * 2.0f64.powi(-24));

        assert_eq!(f16::from_bits(0x0000_0001), f16::from_f64(2.0f64.powi(-24)));
        assert_eq!(
            f16::from_bits(0x0000_0005),
            f16::from_f64(5.0 * 2.0f64.powi(-24))
        );
    }

    #[test]
    fn test_comparisons() {
        let zero = f16::from_f64(0.0);
        let one = f16::from_f64(1.0);
        let neg_zero = f16::from_f64(-0.0);
        let neg_one = f16::from_f64(-1.0);

        assert_eq!(zero.partial_cmp(&neg_zero), Some(Ordering::Equal));
        assert_eq!(neg_zero.partial_cmp(&zero), Some(Ordering::Equal));
        assert!(zero == neg_zero);
        assert!(neg_zero == zero);
        assert!(!(zero != neg_zero));
        assert!(!(neg_zero != zero));
        assert!(!(zero < neg_zero));
        assert!(!(neg_zero < zero));
        assert!(zero <= neg_zero);
        assert!(neg_zero <= zero);
        assert!(!(zero > neg_zero));
        assert!(!(neg_zero > zero));
        assert!(zero >= neg_zero);
        assert!(neg_zero >= zero);

        assert_eq!(one.partial_cmp(&neg_zero), Some(Ordering::Greater));
        assert_eq!(neg_zero.partial_cmp(&one), Some(Ordering::Less));
        assert!(!(one == neg_zero));
        assert!(!(neg_zero == one));
        assert!(one != neg_zero);
        assert!(neg_zero != one);
        assert!(!(one < neg_zero));
        assert!(neg_zero < one);
        assert!(!(one <= neg_zero));
        assert!(neg_zero <= one);
        assert!(one > neg_zero);
        assert!(!(neg_zero > one));
        assert!(one >= neg_zero);
        assert!(!(neg_zero >= one));

        assert_eq!(one.partial_cmp(&neg_one), Some(Ordering::Greater));
        assert_eq!(neg_one.partial_cmp(&one), Some(Ordering::Less));
        assert!(!(one == neg_one));
        assert!(!(neg_one == one));
        assert!(one != neg_one);
        assert!(neg_one != one);
        assert!(!(one < neg_one));
        assert!(neg_one < one);
        assert!(!(one <= neg_one));
        assert!(neg_one <= one);
        assert!(one > neg_one);
        assert!(!(neg_one > one));
        assert!(one >= neg_one);
        assert!(!(neg_one >= one));
    }

    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn round_to_even_f32() {
        // smallest positive subnormal = 0b0.0000_0000_01 * 2^-14 = 2^-24
        let min_sub = f16::from_bits(1);
        let min_sub_f = (-24f32).exp2();
        assert_eq!(f16::from_f32(min_sub_f).to_bits(), min_sub.to_bits());
        assert_eq!(f32::from(min_sub).to_bits(), min_sub_f.to_bits());

        // 0.0000000000_011111 rounded to 0.0000000000 (< tie, no rounding)
        // 0.0000000000_100000 rounded to 0.0000000000 (tie and even, remains at even)
        // 0.0000000000_100001 rounded to 0.0000000001 (> tie, rounds up)
        assert_eq!(
            f16::from_f32(min_sub_f * 0.49).to_bits(),
            min_sub.to_bits() * 0
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 0.50).to_bits(),
            min_sub.to_bits() * 0
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 0.51).to_bits(),
            min_sub.to_bits() * 1
        );

        // 0.0000000001_011111 rounded to 0.0000000001 (< tie, no rounding)
        // 0.0000000001_100000 rounded to 0.0000000010 (tie and odd, rounds up to even)
        // 0.0000000001_100001 rounded to 0.0000000010 (> tie, rounds up)
        assert_eq!(
            f16::from_f32(min_sub_f * 1.49).to_bits(),
            min_sub.to_bits() * 1
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 1.50).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 1.51).to_bits(),
            min_sub.to_bits() * 2
        );

        // 0.0000000010_011111 rounded to 0.0000000010 (< tie, no rounding)
        // 0.0000000010_100000 rounded to 0.0000000010 (tie and even, remains at even)
        // 0.0000000010_100001 rounded to 0.0000000011 (> tie, rounds up)
        assert_eq!(
            f16::from_f32(min_sub_f * 2.49).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 2.50).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f32(min_sub_f * 2.51).to_bits(),
            min_sub.to_bits() * 3
        );

        assert_eq!(
            f16::from_f32(2000.49f32).to_bits(),
            f16::from_f32(2000.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2000.50f32).to_bits(),
            f16::from_f32(2000.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2000.51f32).to_bits(),
            f16::from_f32(2001.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2001.49f32).to_bits(),
            f16::from_f32(2001.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2001.50f32).to_bits(),
            f16::from_f32(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2001.51f32).to_bits(),
            f16::from_f32(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2002.49f32).to_bits(),
            f16::from_f32(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2002.50f32).to_bits(),
            f16::from_f32(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f32(2002.51f32).to_bits(),
            f16::from_f32(2003.0).to_bits()
        );
    }

    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn round_to_even_f64() {
        // smallest positive subnormal = 0b0.0000_0000_01 * 2^-14 = 2^-24
        let min_sub = f16::from_bits(1);
        let min_sub_f = (-24f64).exp2();
        assert_eq!(f16::from_f64(min_sub_f).to_bits(), min_sub.to_bits());
        assert_eq!(f64::from(min_sub).to_bits(), min_sub_f.to_bits());

        // 0.0000000000_011111 rounded to 0.0000000000 (< tie, no rounding)
        // 0.0000000000_100000 rounded to 0.0000000000 (tie and even, remains at even)
        // 0.0000000000_100001 rounded to 0.0000000001 (> tie, rounds up)
        assert_eq!(
            f16::from_f64(min_sub_f * 0.49).to_bits(),
            min_sub.to_bits() * 0
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 0.50).to_bits(),
            min_sub.to_bits() * 0
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 0.51).to_bits(),
            min_sub.to_bits() * 1
        );

        // 0.0000000001_011111 rounded to 0.0000000001 (< tie, no rounding)
        // 0.0000000001_100000 rounded to 0.0000000010 (tie and odd, rounds up to even)
        // 0.0000000001_100001 rounded to 0.0000000010 (> tie, rounds up)
        assert_eq!(
            f16::from_f64(min_sub_f * 1.49).to_bits(),
            min_sub.to_bits() * 1
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 1.50).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 1.51).to_bits(),
            min_sub.to_bits() * 2
        );

        // 0.0000000010_011111 rounded to 0.0000000010 (< tie, no rounding)
        // 0.0000000010_100000 rounded to 0.0000000010 (tie and even, remains at even)
        // 0.0000000010_100001 rounded to 0.0000000011 (> tie, rounds up)
        assert_eq!(
            f16::from_f64(min_sub_f * 2.49).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 2.50).to_bits(),
            min_sub.to_bits() * 2
        );
        assert_eq!(
            f16::from_f64(min_sub_f * 2.51).to_bits(),
            min_sub.to_bits() * 3
        );

        assert_eq!(
            f16::from_f64(2000.49f64).to_bits(),
            f16::from_f64(2000.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2000.50f64).to_bits(),
            f16::from_f64(2000.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2000.51f64).to_bits(),
            f16::from_f64(2001.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2001.49f64).to_bits(),
            f16::from_f64(2001.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2001.50f64).to_bits(),
            f16::from_f64(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2001.51f64).to_bits(),
            f16::from_f64(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2002.49f64).to_bits(),
            f16::from_f64(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2002.50f64).to_bits(),
            f16::from_f64(2002.0).to_bits()
        );
        assert_eq!(
            f16::from_f64(2002.51f64).to_bits(),
            f16::from_f64(2003.0).to_bits()
        );
    }

    impl quickcheck::Arbitrary for f16 {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            use rand::Rng;
            f16(g.gen())
        }
    }

    #[quickcheck]
    fn qc_roundtrip_f16_f32_is_identity(f: f16) -> bool {
        let roundtrip = f16::from_f32(f.to_f32());
        if f.is_nan() {
            roundtrip.is_nan() && f.is_sign_negative() == roundtrip.is_sign_negative()
        } else {
            f.0 == roundtrip.0
        }
    }

    #[quickcheck]
    fn qc_roundtrip_f16_f64_is_identity(f: f16) -> bool {
        let roundtrip = f16::from_f64(f.to_f64());
        if f.is_nan() {
            roundtrip.is_nan() && f.is_sign_negative() == roundtrip.is_sign_negative()
        } else {
            f.0 == roundtrip.0
        }
    }
}
