use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_table_type_name, to_item_type_name, to_ids_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::structs::item_source::ItemSource;
use sabun_maker::intf::member_desc::{MemberDesc, KeyItem};

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
    pub fn new(key : String, is_old : bool) -> KeySource{ KeySource{ key, is_old }}
    pub fn from(key : &KeyItem) -> KeySource{
        KeySource::new(key.key().to_string(), key.is_old())
    }
    pub fn key_name(&self) -> String{
        if self.is_old{ format!("{}_old", to_snake_name(&self.key)) } else{ to_snake_name(&self.key) }
    }
}
impl DataSource{
    pub fn new(stem : String, is_old : bool, keys : Vec<KeySource>, item_source : ItemSource) -> DataSource{
        DataSource{ stem, is_old, keys, item_source }
    }
    pub fn from(desc : &MemberDesc) -> DataSource {
        let mut keys: Vec<KeySource> = vec![];
        let cs = desc.child_descs().unwrap();
        for key in cs.keys() {
            keys.push(KeySource::from(key))
        }
        DataSource::new(
            desc.name().to_string(),
            desc.is_old(),
            keys,
            ItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn keys(&self) -> &[KeySource]{ &self.keys }
    pub fn item_source(&self) -> &ItemSource{ &self.item_source }

    pub fn get(&self, mod_name : &str, ptr_exp : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let data_type_name = to_table_type_name(id);
        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), &data_type_name));
        sb.push(1,&format!("let ans = {}::get_data({}, \"{}\").unwrap();", &mod_name, ptr_exp, id));
        sb.push(1,&format!("{}::new(ans)", &data_type_name));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        let id = self.stem();
        let data_type_name = to_table_type_name(id);
        let item_type_name = to_item_type_name(id);
        let ids_type_name = to_ids_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &data_type_name));
        sb.push(1,"ptr : ConstDataPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &data_type_name));
        sb.push(1, &format!("pub fn new(ptr : ConstDataPtr) -> {}{{ {}{{ ptr }} }} ",
                           &data_type_name, &data_type_name));

        for key in &self.keys {
            let key_name = key.key_name();
            sb.push(1,&format!("pub fn {}(&self) -> {} {{", &key_name, &item_type_name));
            sb.push(2,&format!("let ptr = data::get_value(self.ptr, \"{}\").unwrap();", &key.key));

            sb.push(2, &format!("{}::new(ptr)", &item_type_name));
            sb.push(1, "}");
        }

        if self.keys.len() != 0 {
            sb.push(1,&format!("pub fn from_id(&self, id : &str) -> Option<{}>{{", &item_type_name));
            sb.push(2,"match id{");

            for key in &self.keys {
                 sb.push(3,&format!("\"{}\" => Some(self.{}()),", &key.key, &key.key_name()));
            }

            sb.push(3, &format!("_ =>{{ None }},"));
            sb.push(2, "}");
        }
        sb.push(1, "}");
        sb.push(0, "}");

        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}