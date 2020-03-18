use indexmap::IndexMap;
use crate::rust_struct::RustValue;
use serde_json::Value;

pub fn get_new_default(old : &IndexMap<String, RustValue>, sabun : &IndexMap<String, RustValue>) -> IndexMap<String, Value>{
    let mut result = IndexMap::new();
    for (k,v) in old{
        let k : &String = k;
        let v : &RustValue = v;

        if let Some(sv) = sabun.get(k){
            let sv : RustValue = sv;
            match sv{

            }
        }
    }
    return result;
}