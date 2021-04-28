use crate::parser::*;
use bogobble::*;

impl<A: SSParser<CF>, B: SSParser<CF>, CF> SSParser<CF> for (A, B) {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let (i2, _, e1) = self.0.ss_parse(i, res, cf)?;
        self.1.ss_parse(&i2, res, cf).join_err_op(e1)
    }
}

impl<A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>, CF> SSParser<CF> for (A, B, C) {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let (i2, _, e1) = self.0.ss_parse(i, res, cf)?;
        let (i3, _, e2) = self.1.ss_parse(&i2, res, cf).join_err_op(e1)?;
        self.2.ss_parse(&i3, res, cf).join_err_op(e2)
    }
}

impl<A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>, D: SSParser<CF>, CF> SSParser<CF>
    for (A, B, C, D)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let (i2, _, e1) = self.0.ss_parse(i, res, cf)?;
        let (i3, _, e2) = self.1.ss_parse(&i2, res, cf).join_err_op(e1)?;
        let (i4, _, e3) = self.2.ss_parse(&i3, res, cf).join_err_op(e2)?;
        self.3.ss_parse(&i4, res, cf).join_err_op(e3)
    }
}

impl<CF, A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>, D: SSParser<CF>, E: SSParser<CF>>
    SSParser<CF> for (A, B, C, D, E)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let (i2, _, e) = self.0.ss_parse(i, res, cf)?;
        let (i2, _, e) = self.1.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, _, e) = self.2.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, _, e) = self.3.ss_parse(&i2, res, cf).join_err_op(e)?;
        self.4.ss_parse(&i2, res, cf).join_err_op(e)
    }
}

impl<
        CF,
        A: SSParser<CF>,
        B: SSParser<CF>,
        C: SSParser<CF>,
        D: SSParser<CF>,
        E: SSParser<CF>,
        F: SSParser<CF>,
    > SSParser<CF> for (A, B, C, D, E, F)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let (i2, _, e) = self.0.ss_parse(i, res, cf)?;
        let (i2, _, e) = self.1.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, _, e) = self.2.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, _, e) = self.3.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, _, e) = self.4.ss_parse(&i2, res, cf).join_err_op(e)?;
        self.5.ss_parse(&i2, res, cf).join_err_op(e)
    }
}

impl<CF, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for Or<A, B> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
        let p = res.len();
        match self.a.ss_parse(i, res, cf) {
            Err(e) if e.is_break => Err(e),
            Err(e) => {
                res.replace_range(p.., "");
                match self.b.ss_parse(i, res, cf) {
                    Err(e2) if e2.is_break => Err(e2),
                    Err(e2) => Err(e2.longer(e)),
                    v => v,
                }
            }
            v => v,
        }
    }
}
