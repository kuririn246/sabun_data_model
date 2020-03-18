use indexmap::IndexMap;
use crate::rust_struct::RustValue;
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;

pub fn get_new_default(old : &IndexMap<String, RustValue>, sabun : &IndexMap<String, RustValue>) -> IndexMap<String, Value>{
    let mut result = IndexMap::new();
    for (k,old_v) in old{
        let k : &String = k;
        let v : &RustValue =
            if let Some(sv) = sabun.get(k) {
                sv
            } else {
                old_v
            };

        let (jv,vt) = rust_value_to_json_value(v);
        result.insert(format!("{}{}",k,vt.to_suffix()), jv);
    }
    return result;
}