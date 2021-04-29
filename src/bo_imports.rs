use crate::parser::*;
use bogobble::*;

impl<CF> SSParser<CF> for EOI {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) {
        let i2 = it.clone();
        if i2.next() == None {
            return Ok((i2, (), None));
        }
        i.err_r(Expected::EOI)
    }
}
