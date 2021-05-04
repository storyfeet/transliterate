use crate::parser::*;
use bogobble::*;

impl<CF> SSParser<CF> for &'static str {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        for x in self.chars() {
            if i2.next() != Some(x) {
                return i2.err_rs(self);
            }
        }
        res.push_str(self);
        Ok((i2, None))
    }
}

impl<CF> SSParser<CF> for char {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        match i2.next().map(|c| *self == c) {
            Some(true) => {
                res.push(*self);
                Ok((i2, None))
            }
            _ => Err(it.err(Expected::Char(*self))),
        }
    }
}
