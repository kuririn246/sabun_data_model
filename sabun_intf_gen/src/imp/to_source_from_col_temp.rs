use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
use crate::imp::structs::struct_temp::ColTemp;

pub fn to_source_from_col_temp(imp : &ColTemp) -> StructSource {
    let mut result: Vec<StrAndTab> = vec![];

    result.push(StrAndTab::new(format!("pub struct {} {{", &imp.struct_name), 0));
    result.push(StrAndTab::new(format!("pub ptr : {},", &imp.ptr_type), 1));
    result.push(StrAndTab::new("}".to_string(), 0));

    result.push(StrAndTab::new(
        format!("impl {} {{", &imp.struct_name), 0));

    for key in &imp.keys{
        let key_name = if key.is_old(){ format!("{}_old", key.key()) } else{ key.key().to_string() };
        result.push(StrAndTab::new(
            format!("pub fn {}(&self) -> {} {{", &key_name, &imp.item_struct_name), 1));

        result.push(StrAndTab::new(
            format!("let ptr = {}::get_value(self.ptr, \"{}\")", imp.self_mod_name, key.key()), 2));
        result.push(StrAndTab::new(
            format!("{}::new(ptr)", &imp.item_struct_name), 2));
        result.push(StrAndTab::new(
            "}".to_string(), 1));
    }
    result.push(StrAndTab::new(
        "}".to_string(), 0));

    StructSource::new(
        str_and_tabs_to_string(&result),
        imp.struct_name.to_string())
}