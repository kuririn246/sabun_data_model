use crate::structs::root_object::InnerMutDefObj;
use crate::structs::my_json::Value;
use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::btree_set;

pub fn inner_mut_def_to_json(d : &InnerMutDefObj) -> Value{
    let mut result : Vec<Value> = Vec::new();

    result.push(Value::String("InnerMutDef".to_string()));
    result.push(Value::Array(vec![default_to_json(&d.list_def)]));
    let mut compatible = vec![Value::String("Compatible".to_string())];
    let set = btree_set(d.compatible.as_ref());
    for comp in set{
        compatible.push(Value::String(comp))
    }
    result.push(Value::Array(compatible));

    return Value::Array(result);
}