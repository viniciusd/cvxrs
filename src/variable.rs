pub struct Variable<'a> {
    pub shape: &'a[u32]
}

impl<'a> Variable<'a> {
	pub fn new(shape: &'a[u32]) -> Variable {
		Variable {
            shape
        }
	}
}

pub struct Constant<'a> {
    pub shape: &'a[u32]
}

impl<'a> Constant<'a> {
	pub fn new(shape: &'a[u32]) -> Constant {
		Constant {
            shape
        }
	}
}

pub trait Function {
	fn sign(&self) -> Option<Sign>;
	fn curvature(&self) -> Option<Curvature>;
	fn monotonicity(&self) -> Option<Monotonicity>;
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

pub enum Expression<'a> {
    Variable(Variable<'a>),
    Constant(Constant<'a>),
    Function(Box<dyn Function>),
}
