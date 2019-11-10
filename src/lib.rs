mod variable;

pub use crate::variable::Variable;


#[cfg(test)]
mod tests {
use crate::variable::Variable;
    #[test]
    fn it_works() {
        let v = Variable::new(&[2]);
        assert_eq!(v.shape[0], 3);
    }
}
