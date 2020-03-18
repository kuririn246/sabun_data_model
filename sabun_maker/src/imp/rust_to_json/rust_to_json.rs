use crate::rust_struct::{RustObject, RustValue};
use serde_json::{Value, Map};
use crate::imp::rust_to_json::get_renamed::get_renamed;
use indexmap::IndexMap;

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるとは思うのだけど・・・
pub fn rust_to_json_new_default(obj : &RustObject, def : Option<IndexMap<String, RustValue>>) -> Value{

    let mut map_item = Map::new();
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

    }

    todo!()
}

fn insert(map : &mut Map<String, Value>, s : &str, v : Value){
    map.insert(s.to_string(), v);
}