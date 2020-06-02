use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::structs::my_json::Value;
use crate::structs::root_object::RootObject;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json_short};

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるかもしれない・・・？

pub fn root_to_json_new_default(obj : &RootObject) -> Result<Value> {
    let mut default = obj.default().clone();
    for (name, param) in obj.sabun(){
        if let Some(val) = default.get_mut(name){
            *val = RustValue::Param(param.clone(), val.value_type());
        }
    }
    let mut map = value_map_to_json(&btree_map(obj.default()));
    map.insert( "Old".to_string(), string_set_to_json_short(&btree_set(obj.old())));

    return Ok(Value::Map(map));
}

