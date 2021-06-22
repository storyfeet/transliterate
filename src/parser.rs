use crate::tuple::PartThen;
use bogobble::*;
use std::fmt::Display;
use std::marker::PhantomData;

pub type SSRes<'a> = Result<(PIter<'a>, Option<PErr<'a>>), PErr<'a>>;

pub trait SSErr<'a>: Sized {
    fn join_err(self, e2: PErr<'a>) -> Self;
    fn join_err_op(self, e: Option<PErr<'a>>) -> Self {
        match e {
            Some(e) => self.join_err(e),
            None => self,
        }
    }
    fn put(self, res: &mut String, it: &PIter) -> Self;
}

pub trait BackTo {
    fn back(&self, _: usize) {}
}

impl BackTo for () {}

impl<'a> SSErr<'a> for SSRes<'a> {
    fn join_err(self, e2: PErr<'a>) -> Self {
        self.map_err(|e| e.join(e2))
    }
    fn put(self, res: &mut String, it: &PIter) -> Self {
        if let Ok((i, _)) = &self {
            res.push_str(it.str_to(i.index()));
        }
        self
    }
}

pub trait SSParser<CF: BackTo>: Sized {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a>;

    fn ss_convert<'a>(&self, s: &'a str, cf: &CF) -> Result<String, PErr<'a>> {
        let mut res = String::new();
        let it = PIter::new(s);
        match self.ss_parse(&it, &mut res, cf) {
            Ok(_) => Ok(res),
            Err(e) => Err(e),
        }
    }
}

pub trait SSOrer: Sized {
    fn ss_or<B>(self, b: B) -> SSOR<Self, B> {
        SSOR(self, b)
    }
    fn p_then<B>(self, b: B) -> PartThen<Self, B> {
        PartThen(self, b)
    }
}

impl<T: Sized> SSOrer for T {}

pub struct SS<P: OParser<T>, T>(P, PhantomData<T>);

pub fn ss<P: OParser<T>, T>(p: P) -> SS<P, T> {
    SS(p, PhantomData)
}

impl<P: OParser<T>, T, CF: BackTo> SSParser<CF> for SS<P, T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        match self.0.parse(i) {
            Ok((i2, _, e)) => {
                res.push_str(i.str_to(i2.index()));
                Ok((i2, e))
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Put<T: Display>(pub T);

impl<T: Display, CF: BackTo> SSParser<CF> for Put<T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        res.push_str(&self.0.to_string());
        Ok((i.clone(), None))
    }
}

pub struct SSkip<P: OParser<T>, T>(P, PhantomData<T>);

pub fn sskip<P: OParser<T>, T>(p: P) -> SSkip<P, T> {
    SSkip(p, PhantomData)
}

impl<P: OParser<T>, T, CF: BackTo> SSParser<CF> for SSkip<P, T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        match self.0.parse(i) {
            Ok((i2, _, e)) => Ok((i2, e)),
            Err(e) => Err(e),
        }
    }
}

pub struct SSOR<A, B>(A, B);

impl<CF: BackTo, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for SSOR<A, B> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        let pos = res.len();
        match self.0.ss_parse(it, res, cf) {
            Ok((r, e)) => Ok((r, e)),
            Err(e) if e.is_break => Err(e),
            Err(e) => {
                cf.back(pos);
                res.truncate(pos);
                match self.1.ss_parse(it, res, cf) {
                    Ok((r, ex)) => Ok((r, ex)),
                    Err(e2) if e2.is_break => Err(e2),
                    Err(e2) => Err(e.longer(e2)),
                }
            }
        }
    }
}

pub struct SSDebug(pub &'static str);

impl<CF: BackTo> SSParser<CF> for SSDebug {
    fn ss_parse<'a>(&self, it: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        let (l, c) = it.lc();
        println!("SS DEBUG (l{},c{}): {}, ", l, c, self.0);
        Ok((it.clone(), None))
    }
}
