#![feature(box_patterns)]
use cvxrs::functions::*;
use cvxrs::variable::*;
use cvxrs::solvers;
use ndarray::prelude::*;

fn main() {

    let x = Variable::new(Shape(3, 2), "x".to_owned());
    let a = array![[1.62, -0.61], [-1.07, 0.86], [1.74, -0.76], [-0.24, 1.46],];
    let a = Constant::new(Shape(3, 2), a);
    let b = array![[-0.32, -0.38, 1.13, -1.09]];
    let b = Constant::new(Shape(3, 2), b);

    let objective =
        Sum(Square(Subtract(Multiply(a.into(), x.into()).into(), b.into()).into()).into());

    let problem = Problem {
        objective: objective.into(),
        equalities: Vec::new(),
        inequalities: Vec::new(),
    };

    println!("{:?}", problem.solve());
}
