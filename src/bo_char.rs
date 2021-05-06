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

fn do_char_star<'a, CB: CharBool>(p: &CB, it: &PIter<'a>) -> SSRes<'a> {
    let mut i1 = it.clone();
    let mut i2 = it.clone();
    while let Some(c) = i2.next() {
        if p.char_bool(c) {
            i1 = i2;
            i2 = it.clone();
        } else {
            let e = i1.err(p.expected());
            return Ok((i1, Some(e)));
        }
    }
    Ok((i2, Some(i1.err(p.expected()))))
}

impl<CF, CB: CharBool> SSParser<CF> for CharStar<CB> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        do_char_star(&self.0, it).put(res, it)
    }
}

impl<CF, CB: CharBool> SSParser<CF> for CharPlus<CB> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        let mut i1 = it.clone();
        let mut i2 = match i1.next() {
            Some(c) if self.0.char_bool(c) => i1.clone(),
            _ => return it.err_r(self.0.expected()),
        };

        while let Some(c) = i2.next() {
            if self.0.char_bool(c) {
                i1 = i2.clone();
            } else {
                res.push_str(it.str_to(i1.index()));
                let e = i1.err(self.0.expected());
                return Ok((i1, Some(e)));
            }
        }
        res.push_str(it.str_to(None));
        Ok((i2, Some(i1.err(self.0.expected()))))
    }
}

impl<CF> SSParser<CF> for WS {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        do_char_star(self, it).put(res, it)
    }
}
