use crate::imp::fun::Impl;
use crate::imp::fun_to_string::fun_to_string;
use crate::imp::str_and_tab::StrAndTab;

pub fn generate_struct(imp : &Impl) -> Vec<StrAndTab>{
    let mut result : Vec<StrAndTab> = vec![];
    result.push(StrAndTab::new("use sabun_maker::intf::*;".to_string(), 0));
    result.push(StrAndTab::new("use sabun_maker::structs::*;".to_string(), 0));

    let s = format!("pub struct {} {{ pub ptr : *mut {} }}", &imp.struct_name, &imp.ptr_type);
    result.push(StrAndTab::new(s, 0));
    let s = format!("impl {} {{", &imp.struct_name);
    result.push(StrAndTab::new(s, 0));
    for fun in &imp.funs{
        let fun_str_vec = fun_to_string(fun, &imp.self_mod_name);
        for mut f in fun_str_vec{
            f.tab += 1;
            result.push(f);
        }
    }
    result.push(StrAndTab::new("}".to_string(), 0));

    result
}