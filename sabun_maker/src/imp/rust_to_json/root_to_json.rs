use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::structs::my_json::Value;
use crate::structs::root_object::RootObject;
use std::collections::{HashMap, BTreeMap};

///本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
///デフォルト値も差分も全部Json化したいユースケースもあるとは思うのだけど・・・

pub fn root_to_json_new_default(obj : &RootObject, list_def : Option<&HashMap<String, RustValue>>, root : &RootObject) -> Result<Value> {
    let mut map_item = BTreeMap::new();
    let map = &mut map_item;





    // let new = match list_def {
    //     Some(def) => get_new_default_listitem(def, &obj.default, &obj.sabun, root)?,
    //     None => get_new_default(&obj.default, &obj.sabun, root)?
    // };
    // for (k, v) in new {
    //     map.insert(k, v);
    // }

    return Ok(Value::Map(map_item));
}

fn insert(map : &mut BTreeMap<String, Value>, s : &str, v : Value){
    map.insert(s.to_string(), v);
}