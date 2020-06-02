use crate::structs::my_json::Value;
use crate::structs::root_object::ListDefObj;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::ref_def_obj_to_json::ref_def_obj_to_json;
use crate::imp::rust_to_json::string_set_to_json::string_set_to_json_short;
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};

pub fn default_to_json(obj : &ListDefObj) -> Value{
    let mut result : Vec<Value> = vec![];
    //result.push(Value::String("Default".to_string())); too noisy
    let mut nd = value_map_to_json(&btree_map(&obj.default));
    if obj.refs.is_enum {
        nd.insert("Enum".to_string(), Value::Map(ref_def_obj_to_json(obj.refs.as_ref())));
    } else{
        nd.insert("Ref".to_string(), Value::Map(ref_def_obj_to_json(obj.refs.as_ref())));
    }
    if obj.old.len() != 0 {
        nd.insert("Old".to_string(), string_set_to_json_short(&btree_set(&obj.old)));
    }
    result.push(Value::Map(nd));

    return Value::Array(result);
}