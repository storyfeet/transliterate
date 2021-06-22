#[macro_use]
pub mod macros;

pub mod bo_char;
pub mod bo_common;
pub mod bo_imports;
pub mod bo_part;
pub mod parser;
pub mod std_types;
pub mod tuple;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use bogobble::*;

    ss_parser! { Catter: BackTo ,
        (Put("__"),ss("Hello"),Put("__"))
    }

    ss_parser! { Car:BackTo ,
        (sskip("* ".istar()),Put("("),ss("abcdr".istar()),sskip("* ".istar()),Put(")"))
    }

    #[test]
    fn it_works() {
        assert_eq!(Catter.ss_convert("Hello", &()).unwrap(), "__Hello__");
        assert_eq!(Car.ss_convert("*  * car*  * ", &()).unwrap(), "(car)");
    }
}
