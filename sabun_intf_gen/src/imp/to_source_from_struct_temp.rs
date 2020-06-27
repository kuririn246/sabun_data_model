use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
use crate::imp::structs::struct_temp::StructTemp;

pub fn to_source_from_struct_temp(imp : &StructTemp) -> StructSource {
    let mut result: Vec<StrAndTab> = vec![];

    result.push(StrAndTab::new(format!("#[derive(Debug, PartialEq, Clone)]"), 0));
    result.push(StrAndTab::new(format!("pub struct {} {{", &imp.struct_name), 0));
    result.push(StrAndTab::new(format!("pub ptr : {},", &imp.ptr_type), 1));
    for proxy in &imp.proxies {
        result.push(StrAndTab::new(proxy.to_string(), 1));
    }
    result.push(StrAndTab::new("}".to_string(), 0));

    result.push(StrAndTab::new(
        format!("impl {} {{", &imp.struct_name), 0));

    for line in imp.new.split('\n'){
        result.push(StrAndTab::new(line.to_string(), 1));
    }

    for fun in &imp.funs {
        for line in fun.split('\n') {
            result.push(StrAndTab::new(line.to_string(), 1));
        }
    }
    result.push(StrAndTab::new(
        "}".to_string(), 0));

    StructSource::new(
        str_and_tabs_to_string(&result),
       imp.struct_name.to_string())
}