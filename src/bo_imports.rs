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

impl<CF> SSParser<CF> for &'static str {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> ParseRes<'a, ()> {
        let mut i2 = it.clone();
        for x in self.chars() {
            if i2.next() == Some(x) {
                return i2.err_rs(self);
            }
        }
        res.push_str(self);
        Ok((i2, (), None))
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for KeyWord<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> ParseRes<'a, ()> {}
}
