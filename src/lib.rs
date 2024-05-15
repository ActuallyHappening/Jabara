pub mod prelude {
	pub use crate::variables::*;
}
use crate::prelude::*;

pub struct Equation<LHS, RHS> {
	pub lhs: LHS,
	pub rhs: RHS,
}

mod variables {

	pub trait VariableSet {}
}

/// Can be evaluated
pub enum Function<VAR: VariableSet> {
	FunctionPointer(Box<dyn Fn(VAR)>),
	ExpressionConstruction(Expr<VAR>),
}

mod expr {
	use crate::prelude::*;

	pub trait Expression {
		
	}

	pub struct Expr<VAR: VariableSet, T: Expression>(T);
}
