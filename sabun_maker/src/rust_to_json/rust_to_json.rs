use crate::rust_struct::RustObject;
use serde_json::{Value, Map};
use crate::rust_to_json::get_renamed::get_renamed;

pub fn rust_to_json(obj : &RustObject) -> Value{

    let mut map_item = Map::new();
    let map = &mut map_item;

    let renamed = get_renamed(&obj.renamed);
    insert(map,"Renamed", Value::Array(renamed));

    todo!()
}

fn insert(map : &mut Map<String, Value>, s : &str, v : Value){
    map.insert(s.to_string(), v);
}