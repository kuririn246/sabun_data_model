use crate::indexmap::IndexMap;
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;
use crate::structs::rust_value::RustValue;

///差分まで含めて全部デフォルトにしてしまう。
pub fn get_new_default(list_def : Option<&IndexMap<String, RustValue>>, def : &IndexMap<String, RustValue>, sabun : &IndexMap<String, RustValue>) -> Result<IndexMap<String, Value>>{
    let mut result = IndexMap::new();
    if let Some(map) = list_def {
        for (k, old_v) in map {
            let k: &String = k;
            let v: &RustValue =
                if let Some(sv) = sabun.get(k) {
                    //差分に保存された値を優先
                    sv
                } else  if let Some(dv) = def.get(k){
                    dv
                } else{
                    old_v
                };

            let (jv, vt) = rust_value_to_json_value(v, k)?;
            result.insert(format!("{}{}", k, vt.to_suffix()), jv);
        }
    } else{
        for (k, v) in def{
            let k : &String = k;
            let v : &RustValue = if let Some(sv) = sabun.get(k){
                sv
            } else{
                v
            };
            let (jv, vt) = rust_value_to_json_value(v, k)?;
            result.insert(format!("{}{}", k, vt.to_suffix()), jv);
        }
    }
    return Ok(result);
}