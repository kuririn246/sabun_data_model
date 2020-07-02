use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
use crate::imp::structs::struct_desc::StructDesc;
use crate::imp::util::to_type_name::to_item_type_name;

pub fn to_source_from_col_temp(imp : &StructDesc) -> StructSource {
    let mut result: Vec<StrAndTab> = vec![];

    result.push(StrAndTab::new(format!("#[derive(Debug, PartialEq)]"), 0));
    result.push(StrAndTab::new(format!("pub struct {} {{", &imp.col_struct_name), 0));
    result.push(StrAndTab::new(format!("pub ptr : {},", &imp.col_ptr_type), 1));
    result.push(StrAndTab::new(format!("root : *const RootItem,"), 1));
    for key in &imp.keys{
        result.push(StrAndTab::new(format!("p_{} : *const {},", key.key(), to_item_type_name(key.key())), 1));
    }
    result.push(StrAndTab::new("}".to_string(), 0));

    result.push(StrAndTab::new(
        format!("impl {} {{", &imp.col_struct_name), 0));
    let mut new = format!("pub fn new(ptr : {}, root : *const RootItem) -> {}{{ {}{{ ptr, root, ",
                          &imp.col_ptr_type, &imp.col_struct_name, &imp.col_struct_name);

    for key in &imp.keys{
        new.push_str(&format!("p_{} : None, ", key.key()));
    }
    new.push_str(&format!("}} }}"));
    result.push(StrAndTab::new(new, 1));

    for key in &imp.keys{
        let key_name = if key.is_old(){ format!("{}_old", key.key()) } else{ key.key().to_string() };
        result.push(StrAndTab::new(
            format!("pub fn {}(&mut self) -> &{} {{", &key_name, &imp.item_struct_name), 1));
        result.push(StrAndTab::new(
            format!("if let Some(item) = &self.p_{} {{", key.key()), 2));
        result.push(StrAndTab::new(
            format!("return item;"), 3));
        result.push(StrAndTab::new(
            format!("}}"), 2));

        result.push(StrAndTab::new(
            format!("let ptr = {}::get_value(self.ptr, \"{}\").unwrap();", &imp.col_mod_name, key.key()), 2));
        result.push(StrAndTab::new(
            format!("let item = {}::new(ptr, root);", &imp.item_struct_name), 2));
        result.push(StrAndTab::new(
            format!("self.p_{} = Some(item);", &imp.item_struct_name), 2));
        result.push(StrAndTab::new(
            format!("self.p_{}.as_ref().unwrap()", &imp.item_struct_name), 2));
        result.push(StrAndTab::new(
            "}".to_string(), 1));
    }

    if imp.keys.len() != 0{
        result.push(StrAndTab::new(
            format!("pub fn from_id(&mut self, id : &str) -> &{} {{", &imp.item_struct_name), 1));
        result.push(StrAndTab::new(
            format!("match id{{"), 2));
        for key in &imp.keys {
            result.push(StrAndTab::new(
                format!("\"{}\" => self.{}(),", key.key(), key.key()), 3));
        }
        result.push(StrAndTab::new(
            format!("}}"), 2));
    }
    result.push(StrAndTab::new(
        "}".to_string(), 0));

    StructSource::new(
        str_and_tabs_to_string(&result),
        imp.col_struct_name.to_string())
}