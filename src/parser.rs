use bogobble::*;
use std::fmt::Display;
use std::marker::PhantomData;

pub trait SSParser {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String) -> ParseRes<'a, ()>;
}

impl<P: OParser<String>> SSParser for P {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String) -> ParseRes<'a, ()> {
        match self.parse(i) {
            Ok((i, s, e)) => {
                res.push_str(&s);
                Ok((i, (), e))
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Asis<P: OParser<T>, T>(P, PhantomData<T>);

pub fn asis<P: OParser<T>, T>(p: P) -> Asis<P, T> {
    Asis(p, PhantomData)
}
impl<P: OParser<T>, T> SSParser for Asis<P, T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String) -> ParseRes<'a, ()> {
        match self.0.parse(i) {
            Ok((i2, _, e)) => {
                res.push_str(i.str_to(i2.index()));
                Ok((i2, (), e))
            }
            Err(e) => Err(e),
        }
    }
}

pub struct Put<T: Display>(T);

impl<T: Display> SSParser for Put<T> {
    fn ss_parse<'a>(&self, i: &PIter<'a>, res: &mut String) -> ParseRes<'a, ()> {
        res.push_str(&self.0.to_string());
        Ok((i.clone(), (), None))
    }
}
