use indexmap::IndexMap;
use crate::rust_struct::RustValue;
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;

pub fn get_new_default(def : Option<&IndexMap<String, RustValue>>, sabun : &IndexMap<String, RustValue>) -> Result<IndexMap<String, Value>>{
    let mut result = IndexMap::new();
    if let Some(map) = def {
        for (k, old_v) in map {
            let k: &String = k;
            let v: &RustValue =
                if let Some(sv) = sabun.get(k) {
                    //差分に保存された値を優先
                    sv
                } else {
                    old_v
                };

            let (jv, vt) = rust_value_to_json_value(v, k)?;
            result.insert(format!("{}{}", k, vt.to_suffix()), jv);
        }
    } else{
        for (k, v) in sabun{
            let k : &String = k;
            let v : &RustValue = v;
            let (jv, vt) = rust_value_to_json_value(v, k)?;
            result.insert(format!("{}{}", k, vt.to_suffix()), jv);
        }
    }
    return Ok(result);
}