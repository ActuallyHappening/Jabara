pub enum RationalI32 {
	Zero,
	Integer(i32),
	Fractional(FractionI32),
}

pub struct FractionI32 {
	numerator: i32,
	denominator: i32,
}