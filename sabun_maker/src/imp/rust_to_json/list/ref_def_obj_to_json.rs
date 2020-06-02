use crate::structs::root_object::RefDefObj;
use std::collections::BTreeMap;
use crate::structs::my_json::Value;
use crate::structs::qv::Qv;
use crate::structs::value_type::ValueType;

pub fn ref_def_obj_to_json(obj : &RefDefObj) -> BTreeMap<String, Value>{
    let mut result : BTreeMap<String, Value> = BTreeMap::new();

    for (key,value) in obj.refs.as_ref() {
        let (key, value) = reconstruct_ref_value(key, &value.value, value.value_type);
        result.insert(key, value);
    }

    return result;
}

pub fn reconstruct_ref_value(name : &String, value : &Qv<String>, value_type : ValueType) -> (String, Value){
    (format!("{}{}", name.to_string(), value_type.to_suffix()), match value{
        Qv::Val(v) => Value::String(v.to_string()),
        Qv::Null => Value::Null,
        Qv::Undefined => Value::Undefined,
    })
}
