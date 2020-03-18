use crate::rust_struct::{RustObject, RustValue};
use crate::my_json::{Value};
use crate::imp::rust_to_json::get_renamed::get_renamed;
use indexmap::IndexMap;
use crate::imp::rust_to_json::get_ref_map::get_ref_map;
use crate::imp::rust_to_json::get_new_default::get_new_default;

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるとは思うのだけど・・・
pub fn rust_to_json_new_default(obj : &RustObject, def : Option<IndexMap<String, RustValue>>) -> Value{

    let mut map_item = IndexMap::new();
    let map = &mut map_item;

    let renamed = get_renamed(&obj.renamed);
    insert(map,"Renamed", Value::Array(renamed));

    if let Some(id) = &obj.id{
        insert(map,"ID", Value::String(id.to_string()));
    }
    if obj.obsolete{
        insert(map, "Obsolete", Value::Bool(true));
    }
    if let Some(refs) = &obj.refs{
        let rm = get_ref_map(refs);
        insert(map, "Ref", Value::Map(rm));
    }

    let def = obj.default.as_ref().unwrap_or(def.as_ref().unwrap());

    let new = get_new_default(def, &obj.sabun);

    for (k,v) in new{
        map.insert(k, v);
    }

    return Value::Map(map_item);
}

fn insert(map : &mut IndexMap<String, Value>, s : &str, v : Value){
    map.insert(s.to_string(), v);
}