use crate::rust_struct::{RefMap, Qv, ValueType};
use serde_json::{Map, Value};

pub fn get_ref_map(r : &RefMap) -> Map<String, Value>{
    let mut map = Map::new();

    for (k, (qv, vt)) in r{
        let k : &String = k;
        let qv : &Qv<String> = qv;
        let vt : &ValueType = vt;
        let name = format!("{}{}", k, vt.to_suffix());
        let qv_str = match qv{
            Qv::Val(v) => v,
            Qv::Null => "null",
            Qv::Incompatible => "undefined",
        };
        map.insert(name, Value::String(qv_str.to_string()));
    }

    map
}