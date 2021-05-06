use crate::parser::*;
use bogobble::partial::*;
use bogobble::traits::*;

impl<CF, P: SSParser<CF>> SSParser<CF> for PPlus<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        let mut n = 0;
        loop {
            let rpos = res.len();
            if i2.eoi() {
                return Ok((i2, None));
            }
            match self.0.ss_parse(&i2, res, cf) {
                Ok((i3, _)) => {
                    if i3.index() == i2.index() {
                        return i2.err_rs("No Progression in PPlus");
                    }
                    i2 = i3;
                    n += 1;
                }
                Err(e) => {
                    if n > 0 {
                        res.truncate(rpos);
                        return Ok((i2, Some(e)));
                    }
                    return Err(e);
                }
            }
        }
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for PStar<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        loop {
            let rpos = res.len();
            if i2.eoi() {
                return Ok((i2, None));
            }
            match self.0.ss_parse(&i2, res, cf) {
                Ok((i3, _)) => {
                    i2 = i3;
                }
                Err(e) => {
                    res.truncate(rpos);
                    return Ok((i2, Some(e)));
                }
            }
        }
    }
}
