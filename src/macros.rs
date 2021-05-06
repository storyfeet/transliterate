#[macro_export]
macro_rules! ss_parser {
    ($id:ident , $exp:expr $(,)?) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $id;
        impl<CF> SSParser<CF> for $id {
            ///Parse run the main parser
            fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &CF) -> SSRes<'a> {
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
    (($id:ident,$tp:ident) , $exp:expr $(,)?) => {
        #[derive(Copy, Clone)]
        pub struct $id;
        impl SSParser<$tp> for $id {
            ///Parse run the main parser
            fn ss_parse<'a>(&self, it: &PIter<'a>, res: &mut String, cf: &$tp) -> SSRes<'a> {
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
    use crate::parser::*;
    #[test]
    pub fn can_ss_or() {
        assert_eq!("man".ss_convert("mandothing", &()), Ok("man".to_string()));
        assert!("do".ss_convert("mandothing", &()).is_err());

        assert_eq!(
            ss_or!("cat", "doughnut", "car",).ss_convert("catdoman ", &()),
            Ok("cat".to_string())
        );
    }
}
