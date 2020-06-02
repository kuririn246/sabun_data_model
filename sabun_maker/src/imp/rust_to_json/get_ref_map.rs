use crate::structs::qv::Qv;
use crate::structs::value_type::ValueType;
use crate::structs::ref_value::RefValue;
use crate::structs::my_json::Value;
use std::collections::{HashMap, BTreeMap};
use crate::structs::root_object::RefDefObj;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;

pub fn get_ref_map(r : &HashMap<String, RefValue>) -> BTreeMap<String, Value>{
    let mut map = BTreeMap::new();

    for (k, rv) in r{
        let qv : &Qv<String> = &rv.value;
        let vt : &ValueType = &rv.value_type;
        let name = name_with_suffix(k, *vt);
        match qv{
            Qv::Val(v) => map.insert(name, Value::String(v.to_string())),
            Qv::Null => map.insert(name, Value::Null),
            Qv::Undefined => map.insert(name, Value::Undefined),
        };
        //map.insert(name, Value::String(qv_str.to_string()));
    }

    map
}

pub fn get_ref_def(r : &RefDefObj) -> BTreeMap<String, Value>{
    let mut ref_map = get_ref_map(&r.refs);
    //r.map
    todo!()
}