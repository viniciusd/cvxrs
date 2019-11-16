use crate::variable::{Curvature, Expression, Monotonicity, Sign};

#[derive(Debug, PartialEq)]
pub enum BuiltIn {
    Multiplication(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    ElementPower(Box<Expression>, u32),
    Sum(Box<Expression>),
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
            BuiltIn::ElementPower(_, _) => None,
            BuiltIn::Sum(_) => None,
        }
    }
    fn curvature(&self) -> Option<Curvature> {
        match self {
            BuiltIn::Multiplication(_, _) => Some(Curvature::Convex),
            BuiltIn::Subtraction(_, _) => Some(Curvature::Affine),
            BuiltIn::ElementPower(_, _) => None,
            BuiltIn::Sum(_) => None,
        }
    }
    fn monotonicity(&self) -> Option<Monotonicity> {
        match self {
            BuiltIn::Multiplication(_, _) => Some(Monotonicity::Nonincreasing),
            BuiltIn::Subtraction(_, _) => Some(Monotonicity::Nonincreasing),
            BuiltIn::ElementPower(_, _) => None,
            BuiltIn::Sum(_) => None,
        }
    }
}

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

#[derive(Debug)]
pub struct Sum(pub Expression);
impl Into<Expression> for Sum {
    fn into(self) -> Expression {
        Expression::Atom(BuiltIn::Sum(Box::new(self.0)))
    }
}

#[derive(Debug)]
pub struct Square(pub Expression);
impl Into<Expression> for Square {
    fn into(self) -> Expression {
        Expression::Atom(BuiltIn::ElementPower(Box::new(self.0), 2))
    }
}
