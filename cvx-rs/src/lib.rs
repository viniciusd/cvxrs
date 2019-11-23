// Gonna be useful:
// https://crates.io/crates/ndarray-rand
// https://github.com/rust-ndarray/ndarray-linalg

mod functions;
mod variable;

#[cfg(test)]
mod tests {
    use crate::functions::*;
    use crate::variable::*;
    use ndarray::prelude::*;

    #[test]
    fn it_may_work() {
        use osqp::{CscMatrix, Problem, Settings};
        use std::f64::INFINITY;

        // Define problem data
        let a = array![
            [1.62, -0.61],
            [-1.07, 0.86],
            [1.74, -0.76],
            [-0.24, 1.46],
        ];

        let b = array![[-0.32, -0.38, 1.13, -1.09]];

        let P = a.t().dot(&a);

        let q = -b.dot(&a);

        println!("{:#?}", P);

        let A = &[[0.0, 0.0],
                  [0.0, 0.0],
                  [0.0, 0.0]];
        let l = &[-INFINITY, -INFINITY, -INFINITY];
        let u = &[INFINITY, INFINITY, INFINITY];

        // Extract the upper triangular elements of `P`
        let P = CscMatrix::from(P.outer_iter()).into_upper_tri();

        // Change the default alpha and disable verbose output
        let settings = Settings::default()
            .alpha(1.0)
            .verbose(false);

        // Create an OSQP problem
        let mut prob = Problem::new(P, &q.iter().map(|x| *x).collect::<Vec<_>>(), A, l, u, &settings).expect("failed to setup problem");

        // Solve problem
        let result = prob.solve();

        // Print the solution
        println!("{:?}", result.x().expect("failed to solve problem"));

        assert_eq!(1, 2);
    }
    #[test]
    fn it_works() {
        let x = Variable::new(Shape(3, 2), "x".to_owned());
        let a = array![
            [1.62, -0.61, -0.52],
            [-1.07, 0.86, -2.30],
            [1.74, -0.76, 0.31],
            [-0.24, 1.46, -2.06],
        ];
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

        assert_eq!(
            problem,
            Problem {
                objective: Expression::Atom(BuiltIn::Sum(Box::new(Expression::Atom(
                    BuiltIn::ElementPower(
                        Box::new(Expression::Atom(BuiltIn::Subtraction(
                            Box::new(Expression::Atom(BuiltIn::Multiplication(
                                Box::new(Expression::Constant(Constant {
                                    shape: Shape(3, 2,),
                                    value: array![
                                        [1.62, -0.61, -0.52],
                                        [-1.07, 0.86, -2.30],
                                        [1.74, -0.76, 0.31],
                                        [-0.24, 1.46, -2.06],
                                    ],
                                },)),
                                Box::new(Expression::Variable(Variable {
                                    name: "x".to_string(),
                                    shape: Shape(3, 2,),
                                },),)
                            ),)),
                            Box::new(Expression::Constant(Constant {
                                shape: Shape(3, 2,),
                                value: array![[-0.32, -0.38, 1.13, -1.09]],
                            },),)
                        ))),
                        2
                    )
                )))),
                equalities: Vec::new(),
                inequalities: Vec::new(),
            }
        );
    }
}
