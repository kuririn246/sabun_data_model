use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_type_name, to_data_type_name, to_item_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::structs::item_source::ItemSource;

#[derive(Debug, PartialEq)]
pub struct DataSource{
    stem : String,
    is_old : bool,
    keys : Vec<KeySource>,
    item_source : ItemSource,
}
#[derive(Debug, PartialEq)]
pub struct KeySource{
    key : String,
    is_old : bool,
}
impl KeySource{
    pub fn key_name(&self) -> String{
        if key.is_old(){ format!("{}_old", to_snake_name(&key.key)) } else{ to_snake_name(&key.key) }
    }
}
impl DataSource{
    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn keys(&self) -> &[KeySource]{ &self.keys }
    pub fn item_srouce(&self) -> &ItemSource{ &self.item_source }

    pub fn get(&self, mod_name : &str, ptr_exp : &str, root_exp : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let data_type_name = to_data_type_name(id);
        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), &data_type_name));
        sb.push(1,&format!("let ans = {}::get_data({}, \"{}\").unwrap();", &mod_name, ptr_exp, id));
        sb.push(1,&format!("{}::new(ans, {})", &data_type_name, root_exp));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let data_type_name = to_data_type_name(id);
        let item_type_name = to_item_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &data_type_name);
        sb.push(1,"ptr : ConstDataPtr,");
        sb.push(1,"root : *mut RootItem,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &data_type_name));
        sb.push(1, &format!("pub fn new(ptr : ConstDataPtr, root : *mut RootItem) -> {}{{ {}{{ ptr, root }} }} ",
                           &data_type_name, &data_type_name));

        for key in &self.keys {
            let key_name = key.key_name();
            sb.push(1,&format!("pub fn {}(&self) -> {} {{", &key_name, &item_type_name));
            sb.push(2,&format!("let ptr = data::get_value(self.ptr, \"{}\").unwrap();", &key.key));

            sb.push(2, &format!("{}::new(ptr, self.root);", &item_type_name));
            sb.push(1, "}");
        }

        if self.keys.len() != 0 {
            sb.push(1,&format!("pub fn from_id(&self, id : &str) -> {}{{", &item_type_name));
            sb.push(2,"match id{");

            for key in &self.keys {
                 sb.push(3,&format!("\"{}\" => self.{}(),", &key.key, &key.key_name()));
            }

            sb.push(3, &format!("_ =>{{ None }},"));
            sb.push(2, "}");
        }
        sb.push(1, "}");
        sb.to_string()
    }
}