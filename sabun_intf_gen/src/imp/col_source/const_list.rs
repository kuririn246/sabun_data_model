use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
use crate::imp::structs::struct_desc::StructDesc;

pub fn to_const_list_source(imp : &StructDesc) -> StructSource {
    let mut result: Vec<StrAndTab> = vec![];

    result.push(StrAndTab::new(format!("#[derive(Debug, PartialEq)]"), 0));
    result.push(StrAndTab::new(format!("pub struct {} {{", &imp.col_struct_name), 0));
    result.push(StrAndTab::new(format!("pub ptr : {},", &imp.col_ptr_type), 1));
    result.push(StrAndTab::new(format!("pub root : *mut RootItem,"), 1));
    result.push(StrAndTab::new(format!("pub proxies : HashMap<usize, {}>,", &imp.item_struct_name), 1));

    result.push(StrAndTab::new("}".to_string(), 0));

    result.push(StrAndTab::new(
        format!("impl {} {{", &imp.col_struct_name), 0));
    let mut new = format!("pub fn new(ptr : {}, root : *mut RootItem) -> {}{{ {}{{ ptr, root, ",
                          &imp.col_ptr_type, &imp.col_struct_name, &imp.col_struct_name);

    new.push_str(&format!("}} }}"));
    result.push(StrAndTab::new(new, 1));
    result.push(StrAndTab::new(format!("pub fn root(&mut self) -> *mut RootItem{{ self.root }}"), 1));

    result.push(StrAndTab::new(
        format!("pub fn from_index(&mut self, idx : usize) -> Option<&mut {}> {{", &imp.item_struct_name), 1));
    result.push(StrAndTab::new(
        format!("pub fn from_index(&mut self, idx : usize) -> Option<&mut {}> {{", &imp.item_struct_name), 2));


    // for key in &imp.keys{
    //     let key_name = if key.is_old(){ format!("{}_old", to_snake_name(key.key())) } else{to_snake_name(key.key()) };
    //     result.push(StrAndTab::new(
    //         format!("pub fn {}(&mut self) -> &mut {} {{", &key_name, &imp.item_struct_name), 1));
    //     result.push(StrAndTab::new(
    //         format!("if self.p_{}.is_some() {{", to_snake_name(key.key())), 2));
    //     result.push(StrAndTab::new(
    //         format!("return self.p_{}.as_mut().unwrap();", to_snake_name(key.key())), 3));
    //     result.push(StrAndTab::new(
    //         format!("}}"), 2));
    //
    //     result.push(StrAndTab::new(
    //         format!("let ptr = {}::get_value(self.ptr, \"{}\").unwrap();", &imp.col_mod_name, key.key()), 2));
    //     result.push(StrAndTab::new(
    //         format!("let item = {}::new(ptr, self.root());", &imp.item_struct_name), 2));
    //     result.push(StrAndTab::new(
    //         format!("self.p_{} = Some(item);", to_snake_name(key.key())), 2));
    //     result.push(StrAndTab::new(
    //         format!("self.p_{}.as_mut().unwrap()", to_snake_name(key.key())), 2));
    //     result.push(StrAndTab::new(
    //         "}".to_string(), 1));
    // }
    //
    // if imp.keys.len() != 0{
    //     result.push(StrAndTab::new(
    //         format!("pub fn from_id(&mut self, id : &str) -> Option<&mut {}> {{", &imp.item_struct_name), 1));
    //     result.push(StrAndTab::new(
    //         format!("match id{{"), 2));
    //     for key in &imp.keys {
    //         result.push(StrAndTab::new(
    //             format!("\"{}\" => Some(self.{}()),", key.key(), to_snake_name(key.key())), 3));
    //     }
    //     result.push(StrAndTab::new(
    //         format!("_ =>{{ None }},"), 3));
    //     result.push(StrAndTab::new(
    //         format!("}}"), 2));
    //     result.push(StrAndTab::new(
    //         format!("}}"), 1));
    // }
    result.push(StrAndTab::new(
        "}".to_string(), 0));

    StructSource::new(
        str_and_tabs_to_string(&result),
        imp.col_struct_name.to_string())
}