use crate::my_json::{Value};
use indexmap::IndexMap;
use crate::structs::qv::Qv;
use crate::structs::value_type::ValueType;
use crate::structs::ref_value::RefValue;

pub fn get_ref_map(r : &IndexMap<String, RefValue>) -> IndexMap<String, Value>{
    let mut map = IndexMap::new();

    for (k, rv) in r{
        let rv : &RefValue = rv;
        let k : &String = k;
        let qv : &Qv<String> = &rv.value;
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