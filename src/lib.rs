pub mod prelude {
	pub use crate::expr::*;
	pub use crate::numbers::*;
	pub use crate::variables::*;
}

pub struct Equation<LHS, RHS> {
	pub lhs: LHS,
	pub rhs: RHS,
}

mod variables {
	use crate::prelude::*;
	use std::collections::HashMap;
	use uuid::Uuid;

	/// Collection of arbitrary variables
	pub trait VariableSet {}

	impl VariableSet for () {}

	pub struct Variables {
		map: HashMap<VariableID, Real>,
	}

	/// MARK: This impl is the link between type safety and runtime execution
	impl VariableSet for Variables {}

	impl Variables {
		pub fn empty() -> Self {
			Variables {
				map: HashMap::new(),
			}
		}

		pub fn insert(&mut self, id: &VariableID, value: Real) -> &mut Self {
			self.map.insert(id.internal_clone(), value);
			self
		}

		/// TODO: Panics, use type safety to avoid this panic!
		pub fn get(&self, id: &VariableID) -> &Real {
			self.map.get(id).unwrap()
		}
	}

	/// Specific ID of variable
	#[derive(PartialEq, Eq, Hash)]
	pub struct VariableID {
		label: &'static str,
		id: Uuid,
	}

	impl VariableID {
		pub fn new(label: &'static str) -> Self {
			VariableID {
				label,
				id: Uuid::new_v4(),
			}
		}

		/// Don't think I want to implement clone yet, we'll see
		fn internal_clone(&self) -> Self {
			VariableID {
				label: self.label,
				id: self.id,
			}
		}
	}
}

mod functions {
	use crate::prelude::*;

	// Can be evaluated
	// pub enum Function<VAR: VariableSet> {
	// FunctionPointer(Box<dyn Fn(VAR)>),
	// ExpressionConstruction(Expr<VAR>),
	// }
}

mod numbers {
	pub type Real = f64;
	pub type NonZeroReal = f64;
}

mod expr {
	use std::{marker::PhantomData, ops::Deref};

	use crate::prelude::*;

	/// TODO: add input and output variable sets for more advanced calculus
	pub trait Expression<VAR: VariableSet> {
		fn evaluate(&self, variables: VAR) -> Real;
	}

	pub struct Expr<VAR: VariableSet, E: Expression<VAR>> {
		expr: E,
		_variables: PhantomData<VAR>,
	}

	impl<VAR: VariableSet, E: Expression<VAR>> Deref for Expr<VAR, E> {
		type Target = E;

		fn deref(&self) -> &Self::Target {
			&self.expr
		}
	}

	pub use addition::*;
	mod addition {
		use super::Expression;
		use crate::prelude::*;

		pub struct BinaryAddition<LHS, RHS> {
			pub lhs: LHS,
			pub rhs: RHS,
		}

		impl Expression<()> for BinaryAddition<Real, Real> {
			fn evaluate(&self, _variables: ()) -> Real {
				self.lhs + self.rhs
			}
		}

		// impl<V: VariableSet> Expression
		impl Expression<Variables> for BinaryAddition<VariableID, Real> {
			fn evaluate(&self, variables: Variables) -> Real {
				variables.get(&self.lhs) + self.rhs
			}
		}

		#[cfg(test)]
		mod tests {
			use super::*;
			use proptest::prelude::*;

			proptest! {
				// #![proptest_config(ProptestConfig {
				// 	// cases: 99, .. ProptestConfig::default()
				// })]
				#[test]
				fn test_literal_binary_addition(a: Real, b: Real) {
					let expr = BinaryAddition { lhs: a, rhs: b };
					assert_eq!(expr.evaluate(()), a + b);
				}

				#[test]
				fn test_variable_binary_addition(a: Real, b: Real) {
					let x = VariableID::new("x");
					let mut vars = Variables::empty();
					vars.insert(&x, a);
					let expr = BinaryAddition { lhs: x, rhs: b };
					assert_eq!(expr.evaluate(vars), a + b);
				}
			}
		}
	}
}
