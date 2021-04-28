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
