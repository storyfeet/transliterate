use crate::parser::*;
use bogobble::partial::*;
use bogobble::traits::*;

impl<CF: BackTo, P: SSParser<CF>> SSParser<CF> for PPlus<P> {
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

impl<CF: BackTo, P: SSParser<CF>> SSParser<CF> for PStar<P> {
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

pub struct PKeyWord(pub &'static str);

impl<CF: BackTo> SSParser<CF> for PKeyWord {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        for c in self.0.chars() {
            match i2.next() {
                Some(n) if c == n => {}
                Some(_) => return it.err_rs(self.0),
                None => {
                    res.push_str(it.str_to(None));
                    return Ok((i2, None));
                }
            }
        }
        let mut i3 = i2.clone();
        use bogobble::*;
        match i3.next() {
            Some(c) if (Alpha, NumDigit, '_').char_bool(c) => {
                return i2.err_rs(self.0);
            }
            _ => {
                res.push_str(it.str_to(i2.index()));
                Ok((i2, None))
            }
        }
    }
}

pub struct PStarUntil<A, B>(pub A, pub B);

impl<CF: BackTo, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for PStarUntil<A, B> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        loop {
            if i2.eoi() {
                return Ok((i2, None));
            }
            let rpos = res.len();
            let e = match self.1.ss_parse(&i2, res, cf) {
                Ok(v) => return Ok(v),
                Err(e) => e,
            };
            res.truncate(rpos);
            match self.0.ss_parse(&i2, res, cf) {
                Ok((v, _)) => i2 = v,
                Err(ea) => return Err(ea.join(e)),
            }
        }
    }
}

pub struct PSepPlus<A, B>(pub A, pub B);
impl<CF: BackTo, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for PSepPlus<A, B> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        loop {
            if i2.eoi() {
                return Ok((i2, None));
            }
            match self.0.ss_parse(&i2, res, cf) {
                Ok((nx, _)) => i2 = nx,
                Err(e) => return Err(e),
            }
            if i2.eoi() {
                return Ok((i2, None));
            }
            match self.1.ss_parse(&i2, res, cf) {
                Ok((nx, _)) => i2 = nx,
                Err(e) => return Ok((i2, Some(e))),
            }
        }
    }
}

pub struct PSepUntil<A, B, C>(pub A, pub B, pub C);
impl<CF: BackTo, A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>> SSParser<CF>
    for PSepUntil<A, B, C>
{
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        let mut e_end = Some(match self.2.ss_parse(&i2, res, cf) {
            Ok(v) => return Ok(v),
            Err(e) => e,
        });
        loop {
            if i2.eoi() {
                return Ok((i2, None));
            }
            match self.0.ss_parse(&i2, res, cf) {
                Ok((v, _)) => i2 = v,
                Err(e) => {
                    return match e_end {
                        Some(ee) => Err(e.join(ee)),
                        None => Err(e),
                    }
                }
            }
            if i2.eoi() {
                return Ok((i2, None));
            }
            e_end = match self.2.ss_parse(&i2, res, cf) {
                Ok(v) => return Ok(v),
                Err(e) => Some(e),
            };
            if i2.eoi() {
                return Ok((i2, None));
            }
            e_end = match self.1.ss_parse(&i2, res, cf) {
                Ok((v, e)) => {
                    i2 = v;
                    e
                }
                Err(e) => {
                    return match e_end {
                        Some(ee) => Err(e.join(ee)),
                        None => Err(e),
                    }
                }
            }
        }
    }
}
