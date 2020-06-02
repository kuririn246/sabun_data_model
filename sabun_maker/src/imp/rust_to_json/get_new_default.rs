// use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
// use crate::error::Result;
// use crate::structs::rust_value::{RustValue, RustParam};
// use crate::structs::my_json::Value;
// use crate::structs::root_object::RootObject;
// use std::collections::HashMap;
//
// ///差分まで含めて全部デフォルトにしてしまう。
// /// listの場合はlist_defが必ずあるものと想定。Noneの場合はdefがしっかりあって、sabunはdefで定義された名前のものしかないと想定。
// /// list_defは実際はメンバの定義順と合わせるためにしか使っていない
// pub fn get_new_default_listitem(list_def : &HashMap<String, RustValue>, def : &HashMap<String, RustValue>, sabun : &HashMap<String, RustParam>, root : &RootObject) -> Result<IndexMap<String, Value>> {
//     let mut result = HashMap::new();
//
//     for (k, v) in list_def {
//         let v: &RustValue =
//             if let Some(sv) = sabun.get(k) {
//
//                 //差分に保存された値を優先
//                 &RustValue::Param(sv.clone(), v.value_type())
//             } else if let Some(dv) = def.get(k) {
//                 dv
//             } else {
//                 continue; //defにあるものは書かなくて良い
//             };
//
//         let (jv, vt) = rust_value_to_json_value(v, root, k)?;
//         result.insert(format!("{}{}", k, vt.to_suffix()), jv);
//     }
//
//     return Ok(result);
// }
//
// pub fn get_new_default(def : &HashMap<String, RustValue>, sabun : &HashMap<String, RustParam>, root : &RootObject) -> Result<HashMap<String, Value>> {
//     let mut result = HashMap::new();
//
//     for (k, v) in def {
//         let v: &RustValue = if let Some(sv) = sabun.get(k) {
//             &RustValue::Param(sv.clone(), v.value_type())
//         } else {
//             v
//         };
//         let (jv, vt) = rust_value_to_json_value(v, root, k)?;
//         result.insert(format!("{}{}", k, vt.to_suffix()), jv);
//     }
//
//     return Ok(result);
// }