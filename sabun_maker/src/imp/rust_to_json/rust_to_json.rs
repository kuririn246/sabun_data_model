use crate::imp::rust_to_json::get_renamed::get_renamed;
use crate::imp::rust_to_json::get_ref_map::get_ref_map;
use crate::imp::rust_to_json::get_new_default::{get_new_default, get_new_default_listitem};
use crate::error::Result;
use crate::imp::rust_to_json::get_include::get_include;
use crate::structs::rust_value::RustValue;
use crate::structs::my_json::Value;
use crate::structs::root_object::RootObject;
use std::collections::HashMap;

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるとは思うのだけど・・・

pub fn rust_to_json_new_default(obj : &RootObject, list_def : Option<&HashMap<String, RustValue>>, root : &RootObject) -> Result<Value> {
    let mut map_item = HashMap::new();
    let map = &mut map_item;

    if obj.include.len() != 0 {
        let include = get_include(&obj.include);
        insert(map, "Include", Value::Array(include));
    }

    let renamed = get_renamed(&obj.renamed);
    if renamed.len() != 0 {
        insert(map, "Renamed", Value::Array(renamed));
    }

    if let Some(id) = &obj.id {
        insert(map, "ID", Value::String(id.to_string()));
    }
    if obj.obsolete {
        insert(map, "Obsolete", Value::Bool(true));
    }
    if obj.refs.len() != 0 {
        let rm = get_ref_map(&obj.refs);
        insert(map, "Ref", Value::Map(rm));
    }


    let new = match list_def {
        Some(def) => get_new_default_listitem(def, &obj.default, &obj.sabun, root)?,
        None => get_new_default(&obj.default, &obj.sabun, root)?
    };
    for (k, v) in new {
        map.insert(k, v);
    }

    return Ok(Value::Map(map_item));
}

fn insert(map : &mut IndexMap<String, Value>, s : &str, v : Value){
    map.insert(s.to_string(), v);
}