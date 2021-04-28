use bogobble::*;
use std::fmt::Display;
use std::marker::PhantomData;

pub trait SSParser<CF> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> ParseRes<'a, ()>;

    fn ss_convert<'a>(&self, s: &'a str, cf: &CF) -> Result<String, PErr<'a>> {
        let mut res = String::new();
        let it = PIter::new(s);
        match self.ss_parse(&it, &mut res, cf) {
            Ok(_) => Ok(res),
            Err(e) => Err(e),
        }
    }
}

pub struct SS<P: OParser<T>, T>(P, PhantomData<T>);

pub fn ss<P: OParser<T>, T>(p: P) -> SS<P, T> {
    SS(p, PhantomData)
}
impl<P: OParser<T>, T, CF> SSParser<CF> for SS<P, T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> ParseRes<'a, ()> {
        match self.0.parse(i) {
            Ok((i2, _, e)) => {
                res.push_str(i.str_to(i2.index()));
                Ok((i2, (), e))
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Put<T: Display>(pub T);

impl<T: Display, CF> SSParser<CF> for Put<T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String, _: &CF) -> ParseRes<'a, ()> {
        res.push_str(&self.0.to_string());
        Ok((i.clone(), (), None))
    }
}
