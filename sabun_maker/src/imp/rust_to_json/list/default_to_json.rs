use crate::structs::my_json::Value;
use crate::structs::root_object::ListDefObj;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::ref_def_obj_to_json::ref_def_obj_to_json;
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json_short};
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};
use crate::structs::rust_value::ListType;
use crate::imp::rust_to_json::list::list_type_to_string::list_type_to_string;

pub fn default_to_json(obj : &ListDefObj) -> Value{
    let mut def = value_map_to_json(&btree_map(&obj.default().iter().map(|(k,v)| (k.to_string(), v.to_rust_value())).collect()));
    if obj.refs().refs().len() != 0 {
        if obj.refs().is_enum() {
            def.insert("Enum".to_string(), Value::Map(ref_def_obj_to_json(obj.refs())));
        } else {
            def.insert("Ref".to_string(), Value::Map(ref_def_obj_to_json(obj.refs())));
        }
    }
    if obj.old().len() != 0 {
        def.insert("Old".to_string(), string_set_to_json_short(&btree_set(obj.old())));
    }
    return Value::Map(def);
}

pub fn inner_def_to_json(obj : &ListDefObj, lt : ListType) -> Value{
    let mut result = vec![];
    result.push(Value::String(list_type_to_string(&lt, false)));
    result.push(Value::Array(vec![default_to_json(obj)]));
    return Value::Array(result);
}