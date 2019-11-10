use crate::functions;
use ndarray::ArrayView;

pub struct Problem<'a, A, D> {
    pub objective: Expression<'a, A, D>,
    pub equalities: &'a [Relation<'a, A, D>],
    pub inequalities: &'a [Relation<'a, A, D>],
}

pub struct Relation<'a, A, D> {
    pub rhs: Expression<'a, A, D>,
    pub lhs: Expression<'a, A, D>,
}

#[derive(Clone)]
pub struct Variable<'a> {
    pub shape: &'a [u32],
}

impl<'a> Variable<'a> {
    pub fn new(shape: &'a [u32]) -> Variable {
        Variable { shape }
    }
}

pub struct Constant<'a, A, D> {
    pub shape: &'a [u32],
    pub value: ArrayView<'a, A, D>,
}

impl<'a, A, D> Constant<'a, A, D> {
    pub fn new(shape: &'a [u32], value: ArrayView<'a, A, D>) -> Constant<'a, A, D> {
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

pub enum Expression<'a, A, D> {
    Variable(Variable<'a>),
    Constant(Constant<'a, A, D>),
    Function(Box<functions::Function<'a, A, D>>),
}

impl<'a, A, D> From<Variable<'a>> for Expression<'a, A, D> {
    fn from(item: Variable<'a>) -> Self {
        Expression::Variable(item)
    }
}

impl<'a, A, D> From<Constant<'a, A, D>> for Expression<'a, A, D> {
    fn from(item: Constant<'a, A, D>) -> Self {
        Expression::Constant(item)
    }
}

impl<'a, A, D> From<functions::Function<'a, A, D>> for Expression<'a, A, D> {
    fn from(item: functions::Function<'a, A, D>) -> Self {
        Expression::Function(Box::new(item))
    }
}
