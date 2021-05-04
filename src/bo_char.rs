use crate::parser::*;
use bogobble::charbool::*;
use bogobble::traits::*;

impl<CF, CB: CharBool + Sized> SSParser<CF> for OneChar<CB> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        match i2.next() {
            Some(c) if self.cb.char_bool(c) => Ok((i2, None)),
            _ => it.err_r(self.cb.expected()),
        }
    }
}

impl<CF, CB: CharBool> SSParser<CF> for ICharStar<CB> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        let mut i1 = it.clone();
        let mut i2 = it.clone();
        while let Some(c) = i2.next() {
            if self.0.char_bool(c) {
                i1 = i2;
                i2 = it.clone();
            } else {
                let e = i1.err(self.0.expected());
                return Ok((i1, Some(e)));
            }
        }
        Ok((i2, Some(i1.err(self.0.expected()))))
    }
}

impl<CF, CB: CharBool> SSParser<CF> for ICharPlus<CB> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        let mut i1 = it.clone();
        let mut i2 = it.clone();
        while let Some(c) = i2.next() {
            if self.0.char_bool(c) {
                i1 = i2;
                i2 = it.clone();
            } else {
                let e = i1.err(self.0.expected());
                return Ok((i1, Some(e)));
            }
        }
        Ok((i2, Some(i1.err(self.0.expected()))))
    }
}
