use crate::parser::*;
use bogobble::*;

impl<CF> SSParser<CF> for EOI {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        let mut i2 = it.clone();
        if i2.next() == None {
            return Ok((i2, None));
        }
        it.err_r(Expected::EOI)
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for KeyWord<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        (BRP(self), FailOn(ss((Alpha, NumDigit, '_').one()))).ss_parse(it, res, cf)
    }
}

impl<'b, CF, P: SSParser<CF>> SSParser<CF> for BRP<'b, P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        self.0.ss_parse(it, res, cf)
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for FailOn<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let rpos = res.len();
        match self.0.ss_parse(&it, res, cf) {
            Ok((_, _)) => it.err_rs("Failon Succeeded"),
            Err(_) => {
                res.replace_range(rpos.., "");
                Ok((it.clone(), None))
            }
        }
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for WS_<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        (WS.istar(), BRP(&self.0)).ss_parse(it, res, cf)
    }
}
impl<CF, P: SSParser<CF>> SSParser<CF> for WS__<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        (WS.istar(), BRP(&self.0), WS.istar()).ss_parse(it, res, cf)
    }
}

impl<CF, P: SSParser<CF>> SSParser<CF> for Maybe<P> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let rp = res.len();
        match self.0.ss_parse(it, res, cf) {
            Ok((i2, e)) => Ok((i2, e)),
            Err(_) => {
                res.replace_range(rp.., "");
                Ok((it.clone(), None))
            }
        }
    }
}

impl<CF, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for PlusUntil<A, B> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let n = 0;
        let (i2, _) = self.0.ss_parse(it, res, cf)?;
        loop {
            let rpos = res.len();
            let exp = match self.1.ss_parse(it, res, cf)? {
                Ok(v) => return Ok(v),
                Err(c) => c,
            };
            res.truncate(rpos);
            match self.0.ss_parse(it,res,cf) {
                //TODO
            }
        }
    }
}
