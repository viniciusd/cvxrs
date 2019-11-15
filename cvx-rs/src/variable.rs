use crate::functions::{BuiltIn, Dcp};
use ndarray::Array2;

pub struct Problem {
    pub objective: Expression,
    pub equalities: Vec<Relation>,
    pub inequalities: Vec<Relation>,
}

pub struct Relation {
    pub rhs: Expression,
    pub lhs: Expression,
}

#[derive(Clone, Debug)]
pub struct Shape(pub u32, pub u32);
pub type Scalar = f32;

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub shape: Shape,
}

impl<'a> Variable {
    pub fn new(shape: Shape, name: String) -> Variable {
        Variable { shape, name }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub shape: Shape,
    pub value: Array2<Scalar>,
}

impl Constant {
    pub fn new(shape: Shape, value: Array2<Scalar>) -> Constant {
        Constant { shape, value }
    }
}

pub enum Sign {
    Negative,
    Positive,
}

pub enum Curvature {
    Convex,
    Concave,
    Affine,
}

pub enum Monotonicity {
    Nondecreasing,
    Nonincreasing,
}

#[derive(Debug)]
pub enum Expression {
    Variable(Variable),
    Constant(Constant),
    Atom(BuiltIn),
}

/*impl<'a, A, D> Expression<'a, A, D> {
    pub fn new_function<T>(item: T) -> Expression<'a, A, D>
    where
        T: 'a + functions::Function<'a, A, D>,
    {
        Expression::Function(Box::new(item))
    }
}*/

impl Into<Expression> for Variable {
    fn into(self) -> Expression {
        Expression::Variable(self)
    }
}

impl Dcp for Variable {
    fn sign(&self) -> Option<Sign> {
        None
    }
    fn curvature(&self) -> Option<Curvature> {
        Some(Curvature::Affine)
    }
    fn monotonicity(&self) -> Option<Monotonicity> {
        None
    }
}

impl Into<Expression> for Constant {
    fn into(self) -> Expression {
        Expression::Constant(self)
    }
}

impl Dcp for Constant {
    fn sign(&self) -> Option<Sign> {
        None
    }
    fn curvature(&self) -> Option<Curvature> {
        Some(Curvature::Affine)
    }
    fn monotonicity(&self) -> Option<Monotonicity> {
        None
    }
}
