use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::btree_set;
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json};
use crate::imp::structs::def_obj::InnerMutDefObj;
use crate::imp::structs::my_json::Value;

pub fn inner_mut_def_to_json(d : &InnerMutDefObj) -> Value{
    let mut result : Vec<Value> = Vec::new();

    result.push(Value::String("InnerMutDef".to_string()));
    if d.compatible().len() != 0{
        result.push(string_set_to_json("Compatible", &btree_set(d.compatible())));
    }

    result.push(Value::Array(vec![default_to_json(d.list_def())]));


    return Value::Array(result);
}