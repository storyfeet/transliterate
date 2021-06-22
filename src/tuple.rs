use crate::parser::*;
use bogobble::*;

impl<A: SSParser<CF>, B: SSParser<CF>, CF: BackTo> SSParser<CF> for (A, B) {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e1) = self.0.ss_parse(i, res, cf)?;
        self.1.ss_parse(&i2, res, cf).join_err_op(e1)
    }
}

impl<A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>, CF: BackTo> SSParser<CF> for (A, B, C) {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e1) = self.0.ss_parse(i, res, cf)?;
        let (i3, e2) = self.1.ss_parse(&i2, res, cf).join_err_op(e1)?;
        self.2.ss_parse(&i3, res, cf).join_err_op(e2)
    }
}

impl<A: SSParser<CF>, B: SSParser<CF>, C: SSParser<CF>, D: SSParser<CF>, CF: BackTo> SSParser<CF>
    for (A, B, C, D)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e1) = self.0.ss_parse(i, res, cf)?;
        let (i3, e2) = self.1.ss_parse(&i2, res, cf).join_err_op(e1)?;
        let (i4, e3) = self.2.ss_parse(&i3, res, cf).join_err_op(e2)?;
        self.3.ss_parse(&i4, res, cf).join_err_op(e3)
    }
}

impl<
        CF: BackTo,
        A: SSParser<CF>,
        B: SSParser<CF>,
        C: SSParser<CF>,
        D: SSParser<CF>,
        E: SSParser<CF>,
    > SSParser<CF> for (A, B, C, D, E)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e) = self.0.ss_parse(i, res, cf)?;
        let (i2, e) = self.1.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, e) = self.2.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, e) = self.3.ss_parse(&i2, res, cf).join_err_op(e)?;
        self.4.ss_parse(&i2, res, cf).join_err_op(e)
    }
}

impl<
        CF: BackTo,
        A: SSParser<CF>,
        B: SSParser<CF>,
        C: SSParser<CF>,
        D: SSParser<CF>,
        E: SSParser<CF>,
        F: SSParser<CF>,
    > SSParser<CF> for (A, B, C, D, E, F)
{
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e) = self.0.ss_parse(i, res, cf)?;
        let (i2, e) = self.1.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, e) = self.2.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, e) = self.3.ss_parse(&i2, res, cf).join_err_op(e)?;
        let (i2, e) = self.4.ss_parse(&i2, res, cf).join_err_op(e)?;
        self.5.ss_parse(&i2, res, cf).join_err_op(e)
    }
}

pub struct PartThen<A, B>(pub A, pub B);

impl<A: SSParser<CF>, B: SSParser<CF>, CF: BackTo> SSParser<CF> for PartThen<A, B> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let (i2, e) = self.0.ss_parse(i, res, cf)?;
        if i2.eoi() {
            return Ok((i2, e));
        }
        self.1.ss_parse(&i2, res, cf).join_err_op(e)
    }
}
