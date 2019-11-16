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
