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
	use std::{any::TypeId, collections::HashMap, marker::PhantomData};
	use uuid::Uuid;

	// Collection of arbitrary variables
	// pub trait VariableSet {}

	// impl VariableSet for () {}

	pub trait Variable {
		fn label(&self) -> &'static str {
			"not implemented"
		}
	}

	impl Variable for () {
		fn label(&self) -> &'static str {
			"()"
		}
	}

	pub struct Variables<T> {
		_t: PhantomData<T>,
		map: HashMap<TypeId, Real>,
	}

	/// MARK: This impl is the link between type safety and runtime execution
	// impl VariableSet for Variables {}

	impl Variables<()> {
		pub fn empty() -> Self {
			Variables {
				_t: PhantomData,
				map: HashMap::new(),
			}
		}

		pub fn insert<T: Variable + 'static>(mut self, value: Real) -> Variables<T> {
			self.map.insert(TypeId::of::<T>(), value);
			Variables {
				_t: PhantomData,
				map: self.map,
			}
		}

		// /// TODO: Panics, use type safety to avoid this panic!
		// pub fn get(&self, id: &VariableID) -> &Real {
		// 	self.map.get(id).unwrap()
		// }
	}

	impl<T: Variable + 'static> Variables<T> {
		pub fn get(&self) -> Real {
			// MARK: This unwrap is safe because we know that the variable is in the map by this point
			*self.map.get(&TypeId::of::<T>()).unwrap()
		}
	}

	#[cfg(test)]
	mod test {
    use crate::prelude::{Variable, Variables};

		#[test]
		fn variables_typing() {
			let empty: Variables<()> = Variables::empty();

			struct VarX;
			impl Variable for VarX {}

			let variables_with_x: Variables<VarX> = empty.insert(1.0);
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
	pub trait Expression<VAR: Variable> {
		fn evaluate(&self, variables: Variables<VAR>) -> Real;
	}

	// pub struct Expr<VAR: VariableSet, E: Expression<VAR>> {
	// 	expr: E,
	// 	_variables: PhantomData<VAR>,
	// }

	// impl<VAR: VariableSet, E: Expression<VAR>> Deref for Expr<VAR, E> {
	// 	type Target = E;

	// 	fn deref(&self) -> &Self::Target {
	// 		&self.expr
	// 	}
	// }

	pub use addition::*;
	mod addition {
		use std::marker::PhantomData;

		use super::Expression;
		use crate::prelude::*;

		pub struct BinaryAddition<LHS, RHS, VAR: Variable = ()> {
			pub lhs: LHS,
			pub rhs: RHS,
			_vars: PhantomData<VAR>,
		}

		impl<LHS, RHS> BinaryAddition<LHS, RHS> {
			pub fn new(lhs: LHS, rhs: RHS) -> Self {
				BinaryAddition {
					lhs,
					rhs,
					_vars: PhantomData,
				}
			}
		}

		impl Expression<()> for BinaryAddition<Real, Real> {
			fn evaluate(&self, _variables: Variables<()>) -> Real {
				self.lhs + self.rhs
			}
		}

		// impl<V: VariableSet> Expression
		impl<VAR: Variable + 'static> Expression<VAR> for BinaryAddition<VAR, Real> {
			fn evaluate(&self, variables: Variables<VAR>) -> Real {
				variables.get() + self.rhs
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
					let expr = BinaryAddition::new(a, b);
					assert_eq!(expr.evaluate(Variables::empty()), a + b);
				}

				#[test]
				fn test_variable_binary_addition(a: Real, b: Real) {
					pub struct VarX;
					impl Variable for VarX {}

					let x = VarX;
					let vars = Variables::empty().insert::<VarX>(a);
					let expr = BinaryAddition::new(x, b);
					assert_eq!(expr.evaluate(vars), a + b);
				}
			}
		}
	}
}
