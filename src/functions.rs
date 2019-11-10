use crate::variable::{Curvature, Expression, Monotonicity, Sign};

pub enum Function<'a, A, D> {
    Multiply(Expression<'a, A, D>, Expression<'a, A, D>),
    Subtract(Expression<'a, A, D>, Expression<'a, A, D>),
}

trait Atom {
    fn sign(&self) -> Option<Sign>;
    fn curvature(&self) -> Option<Curvature>;
    fn monotonicity(&self) -> Option<Monotonicity>;
}

impl<'a, A, D> Atom for Function<'a, A, D> {
    fn sign(&self) -> Option<Sign> {
        match self {
            Function::Multiply(_, _) => None,
            Function::Subtract(_, _) => None,
        }
    }
    fn curvature(&self) -> Option<Curvature> {
        match self {
            Function::Multiply(_, _) => Some(Curvature::Convex),
            Function::Subtract(_, _) => Some(Curvature::Affine),
        }
    }
    fn monotonicity(&self) -> Option<Monotonicity> {
        match self {
            Function::Multiply(_, _) => Some(Monotonicity::Nonincreasing),
            Function::Subtract(_, _) => Some(Monotonicity::Nonincreasing),
        }
    }
}
