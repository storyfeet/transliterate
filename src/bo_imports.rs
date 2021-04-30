use crate::parser::*;
use bogobble::*;

impl<CF> SSParser<CF> for EOI {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> ParseRes<'a, ()> {
        let mut i2 = it.clone();
        if i2.next() == None {
            return Ok((i2, (), None));
        }
        it.err_r(Expected::EOI)
    }
}
