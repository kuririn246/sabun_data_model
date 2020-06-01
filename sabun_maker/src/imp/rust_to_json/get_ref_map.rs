use crate::structs::qv::Qv;
use crate::structs::value_type::ValueType;
use crate::structs::ref_value::RefValue;
use crate::structs::my_json::Value;
use std::collections::HashMap;

pub fn get_ref_map(r : &HashMap<String, RefValue>) -> IndexMap<String, Value>{
    let mut map = HashMap::new();

    for (k, rv) in r{
        let rv : &RefValue = rv;
        let k : &String = k;
        let qv : &Qv<String> = rv.get_value();
        let vt : &ValueType = &rv.value_type;
        let name = format!("{}{}", k, vt.to_suffix());
        match qv{
            Qv::Val(v) => map.insert(name, Value::String(v.to_string())),
            Qv::Null => map.insert(name, Value::Null),
            Qv::Undefined => map.insert(name, Value::Undefined),
        };
        //map.insert(name, Value::String(qv_str.to_string()));
    }

    map
}