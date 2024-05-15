use crate::prelude::*;

pub enum Expression<SS: RealSubset> {
	Rational(Rational<SS>),
	Operation(Operation<SS>),
}

pub enum Operation<SS: RealSubset> {
	Unary()
}