use crate::error::Result;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json_short};
use std::collections::HashMap;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::root_value::RootValue;

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるかもしれない・・・？

pub fn root_to_json_new_default(obj : &RootObject) -> Result<Value> {
    let mut result : HashMap<String,RustValue> = HashMap::with_capacity(obj.default().len());
    let sabun = obj.sabun();
    for (name, val) in obj.default(){
        if let RootValue::Param(_p,v) = val{
            if let Some(ps) = sabun.get(name){
                result.insert(name.to_string(), RustValue::Param(ps.clone(), v.clone()));
            } else{
                result.insert(name.to_string(), val.clone().into_rust_value());
            }
        }
    }

    let mut map = value_map_to_json(&btree_map(&result));
    map.insert( "Old".to_string(), string_set_to_json_short(&btree_set(obj.old())));

    return Ok(Value::Map(map));
}

