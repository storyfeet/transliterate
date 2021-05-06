use crate::parser::*;
use bogobble::common::*;
use bogobble::*;

impl<CF> SSParser<CF> for Ident {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        (ss(Alpha.iplus()), (Alpha, NumDigit, '_').star())
            .ss_parse(it, res, cf)
            .join_err(it.err_s("Ident"))
    }
}
