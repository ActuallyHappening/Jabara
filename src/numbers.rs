use crate::prelude::*;

pub enum Rational<CX: Context> where CX::Set: RealSubset {
	Zero,
	Integer(<<CX as Context>::Set as RealSubset>::NonZeroSigned),
	Fractional(Fraction<CX>),
}

/// NonZero
pub struct Fraction<SS: RealSubset> {
	numerator: SS::NonZeroSigned,
	denominator: SS::NonZeroUnsigned,
}
