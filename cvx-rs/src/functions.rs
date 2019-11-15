use crate::variable::{Curvature, Expression, Monotonicity, Sign};

#[derive(Debug)]
pub enum BuiltIn {
    Multiplication(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
}

pub trait Dcp: Into<Expression> {
    fn sign(&self) -> Option<Sign>;
    fn curvature(&self) -> Option<Curvature>;
    fn monotonicity(&self) -> Option<Monotonicity>;
}

impl Into<Expression> for BuiltIn {
    fn into(self) -> Expression {
        Expression::Atom(self)
    }
}

impl Dcp for BuiltIn {
    fn sign(&self) -> Option<Sign> {
        match self {
            BuiltIn::Multiplication(_, _) => None,
            BuiltIn::Subtraction(_, _) => None,
        }
    }
    fn curvature(&self) -> Option<Curvature> {
        match self {
            BuiltIn::Multiplication(_, _) => Some(Curvature::Convex),
            BuiltIn::Subtraction(_, _) => Some(Curvature::Affine),
        }
    }
    fn monotonicity(&self) -> Option<Monotonicity> {
        match self {
            BuiltIn::Multiplication(_, _) => Some(Monotonicity::Nonincreasing),
            BuiltIn::Subtraction(_, _) => Some(Monotonicity::Nonincreasing),
        }
    }
}

#[derive(Debug)]
pub struct Multiply(pub Expression, pub Expression);
impl Into<Expression> for Multiply {
    fn into(self) -> Expression {
        Expression::Atom(BuiltIn::Multiplication(Box::new(self.0), Box::new(self.1)))
    }
}

#[derive(Debug)]
pub struct Subtract(pub Expression, pub Expression);
impl Into<Expression> for Subtract {
    fn into(self) -> Expression {
        Expression::Atom(BuiltIn::Subtraction(Box::new(self.0), Box::new(self.1)))
    }
}
