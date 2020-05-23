use crate::indexmap::IndexMap;
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::structs::root_object::RustObject;

///差分まで含めて全部デフォルトにしてしまう。
/// listの場合はlist_defが必ずあるものと想定。Noneの場合はdefがしっかりあって、sabunはdefで定義された名前のものしかないと想定。
/// list_defは実際はメンバの定義順と合わせるためにしか使っていない
pub fn get_new_default_listitem(list_def : &IndexMap<String, RustValue>, def : &IndexMap<String, RustValue>, sabun : &IndexMap<String, RustValue>, root : &RustObject) -> Result<IndexMap<String, Value>> {
    let mut result = IndexMap::new();

    for (k, _) in list_def {
        let v: &RustValue =
            if let Some(sv) = sabun.get(k) {
                //差分に保存された値を優先
                sv
            } else if let Some(dv) = def.get(k) {
                dv
            } else {
                continue; //defにあるものは書かなくて良い
            };

        let (jv, vt) = rust_value_to_json_value(v, root, k)?;
        result.insert(format!("{}{}", k, vt.to_suffix()), jv);
    }

    return Ok(result);
}

pub fn get_new_default(def : &IndexMap<String, RustValue>, sabun : &IndexMap<String, RustValue>, root : &RustObject) -> Result<IndexMap<String, Value>> {
    let mut result = IndexMap::new();

    for (k, v) in def {
        let v: &RustValue = if let Some(sv) = sabun.get(k) {
            sv
        } else {
            v
        };
        let (jv, vt) = rust_value_to_json_value(v, root, k)?;
        result.insert(format!("{}{}", k, vt.to_suffix()), jv);
    }

    return Ok(result);
}