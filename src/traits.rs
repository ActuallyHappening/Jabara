use crate::prelude::*;

pub trait RealSubset {
	/// Can be any value
	type Signed;
	/// Must always be >= 0
	type Unsigned;

	/// Must never be 0
	type NonZeroSigned;
	/// Must always be > 0
	type NonZeroUnsigned;
}

impl RealSubset for i32 {
	type Unsigned = u32;
	type Signed = i32;

	type NonZeroUnsigned = NonZeroI32;
	type NonZeroSigned = NonZeroU32;
}

pub trait Context {
	type OperationSet: OperationSet;
	type Set;
}

pub trait OperationSet {
	type UnaryOperations;
	type BinaryOperations;
}