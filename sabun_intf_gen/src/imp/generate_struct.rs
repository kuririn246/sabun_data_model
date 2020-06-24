use crate::imp::fun_to_string::fun_to_string;
use crate::imp::structs::fun::Impl;
use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};

pub fn generate_struct(imp : &Impl) -> StructSource {
    let mut result: Vec<StrAndTab> = vec![];

    result.push(StrAndTab::new(format!("pub struct {} {{", &imp.struct_name), 0));
    result.push(StrAndTab::new(format!("pub ptr : *mut {},", &imp.ptr_type), 1));
    for proxy in &imp.proxies {
        result.push(StrAndTab::new(format!("{} : Option<{}>,", &proxy.name, &proxy.value_type), 1));
    }
    result.push(StrAndTab::new("}".to_string(), 0));

    result.push(StrAndTab::new(
        format!("impl {} {{", &imp.struct_name), 0));
    for fun in &imp.funs {
        let fun_str_vec = fun_to_string(fun, &imp.self_mod_name);
        for mut f in fun_str_vec {
            f.tab += 1;
            result.push(f);
        }
    }
    result.push(StrAndTab::new(
        "}".to_string(), 0));

    StructSource::new(
        str_and_tabs_to_string(&result),
       imp.struct_name.to_string())
}