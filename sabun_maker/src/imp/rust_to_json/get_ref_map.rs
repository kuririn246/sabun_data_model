use crate::rust_struct::{RefMap, Qv, ValueType};
use crate::my_json::{Value};
use indexmap::IndexMap;

pub fn get_ref_map(r : &RefMap) -> IndexMap<String, Value>{
    let mut map = IndexMap::new();

    for (k, (qv, vt)) in r{
        let k : &String = k;
        let qv : &Qv<String> = qv;
        let vt : &ValueType = vt;
        let name = format!("{}{}", k, vt.to_suffix());
        let qv_str = match qv{
            Qv::Val(v) => v,
            Qv::Null => "null",
            Qv::Undefined => "undefined",
        };
        map.insert(name, Value::String(qv_str.to_string()));
    }

    map
}