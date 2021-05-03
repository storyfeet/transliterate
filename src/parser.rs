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
}

impl<'a> SSErr<'a> for SSRes<'a> {
    fn join_err(self, e2: PErr<'a>) -> Self {
        self.map_err(|e| e.join(e2))
    }
}

pub trait SSParser<CF>: Sized {
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
}

impl<T: Sized> SSOrer for T {}

pub struct SS<P: OParser<T>, T>(P, PhantomData<T>);

pub fn ss<P: OParser<T>, T>(p: P) -> SS<P, T> {
    SS(p, PhantomData)
}
impl<P: OParser<T>, T, CF> SSParser<CF> for SS<P, T> {
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

impl<T: Display, CF> SSParser<CF> for Put<T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> SSRes<'a> {
        res.push_str(&self.0.to_string());
        Ok((i.clone(), None))
    }
}

pub struct SSkip<P: OParser<T>, T>(P, PhantomData<T>);

pub fn sskip<P: OParser<T>, T>(p: P) -> SSkip<P, T> {
    SSkip(p, PhantomData)
}

impl<P: OParser<T>, T, CF> SSParser<CF> for SSkip<P, T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, _: &mut String, _: &CF) -> SSRes<'a> {
        match self.0.parse(i) {
            Ok((i2, _, e)) => Ok((i2, e)),
            Err(e) => Err(e),
        }
    }
}

pub struct SSOR<A, B>(A, B);

impl<CF, A: SSParser<CF>, B: SSParser<CF>> SSParser<CF> for SSOR<A, B> {
    fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
        match self.0.ss_parse(it, res, cf) {
            Ok((r, e)) => Ok((r, e)),
            Err(e) if e.is_break => Err(e),
            Err(e) => match self.1.ss_parse(it, res, cf) {
                Ok((r, ex)) => Ok((r, ex)),
                Err(e2) if e2.is_break => Err(e2),
                Err(e2) => Err(e.longer(e2)),
            },
        }
    }
}
