#[macro_export]
macro_rules! ss_parser {
    ($id:ident , $exp:expr $(,)?) => {
        #[derive(Copy, Clone)]
        pub struct $id;
        impl<CF> SSParser<CF> for $id {
            ///Parse run the main parser
            fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> ParseRes<'a, ()> {
                match $exp.ss_parse(it, res, cf) {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        let name_e = it.err_s(stringify!($id));
                        match (e.index, name_e.index) {
                            (Some(ei), Some(ii)) if (ii == ei) => Err(it.err_s(stringify!($id))),
                            _ => Err(e.join(name_e)),
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ss_or{
    ($s:expr,$($x:expr),* $(,)?) => { $s$(.ss_or($x))*;};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    #[test]
    pub fn can_ss_or() {
        let a = ss_or!("cat", "dog", "car");
        /*       assert_eq!(
            ss_or!("cat", "dog", "car",).ss_convert("catdogman ", &()),
            Ok("cat".to_string())
        );*/
    }
}
