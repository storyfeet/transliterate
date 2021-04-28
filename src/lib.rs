#[macro_use]
pub mod macros;

pub mod parser;
pub mod tuple;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use bogobble::*;

    ss_parser! { Catter ,
        (Put("__"),ss("Hello"),Put("__"))
    }

    #[test]
    fn it_works() {
        //let p = (Put("__"), ss("Hello"), Put("__"));
        assert_eq!(Catter.ss_convert("Hello", &()).unwrap(), "__Hello__");
    }
}
