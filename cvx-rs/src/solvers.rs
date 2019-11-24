use crate::functions::*;
use crate::variable::*;
use ndarray::Array;
use osqp;

#[derive(Debug, PartialEq)]
pub enum Result {
    Solved(Vec<f64>),
}

pub trait Solver {
    fn new(problem: Problem) -> Self;
    fn solve(self) -> Result;
}

pub struct Osqp {
    problem: osqp::Problem,
}

impl Solver for Osqp {
    fn new(problem: Problem) -> Self {
        use std::f64::INFINITY;
        match problem {
            Problem {
                objective:
                    Expression::Atom(BuiltIn::Sum(box Expression::Atom(BuiltIn::ElementPower(
                        box Expression::Atom(BuiltIn::Subtraction(
                            box Expression::Atom(BuiltIn::Multiplication(
                                box Expression::Constant(Constant { shape: _, value: a }),
                                box Expression::Variable(_),
                            )),
                            box Expression::Constant(Constant { shape: _, value: b }),
                        )),
                        2,
                    )))),
                equalities,
                inequalities: _,
            } => {
                let P = a.t().dot(&a);

                let q = -b.dot(&a);

                println!("{:#?}", P);

                let A = if equalities.is_empty() {
                    Array::zeros(P.shape())
                } else {
                    panic!("Equalities aren't supported yet")
                };
                let l = if equalities.is_empty() {
                    Array::from_elem((1, A.shape()[0]), -INFINITY)
                } else {
                    panic!("Equalities aren't supported yet")
                };
                let u = if equalities.is_empty() {
                    Array::from_elem((1, A.shape()[0]), INFINITY)
                } else {
                    panic!("Equalities aren't supported yet")
                };

                // Extract the upper triangular elements of `P`
                let P = osqp::CscMatrix::from(P.outer_iter()).into_upper_tri();

                // Change the default alpha and disable verbose output
                let settings = osqp::Settings::default().alpha(1.0).verbose(false);

                Osqp {
                    problem: osqp::Problem::new(
                        P,
                        &q.iter().map(|x| *x).collect::<Vec<_>>(),
                        A.outer_iter(),
                        &l.iter().map(|x| *x).collect::<Vec<_>>(),
                        &u.iter().map(|x| *x).collect::<Vec<_>>(),
                        &settings,
                    )
                    .expect("failed to setup problem"),
                }
            }
            _ => panic!("Problem type not implemented"),
        }
    }
    fn solve(mut self) -> Result {
        match self.problem.solve() {
            osqp::Status::Solved(solution) => {
                Result::Solved(solution.x().iter().map(|x| *x).collect::<Vec<_>>())
            }
            _ => panic!("Cannot convert osqp result"),
        }
    }
}
