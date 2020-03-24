use crate::rust_struct::{RustObject, Qv, ValueType};
use crate::error::Result;
use indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;
use std::collections::BTreeMap;

///list_itemsのRefの名前と型がDefault objectのRefと一致してるか調べる
///Renamedされていれば自動で追跡するのだが、ListのデフォルトオブジェクトのRefは自動で追跡してくれないので、DefaultのRefだけはちゃんと最新のに書き直さないとエラーになってしまうことになっている
pub fn validate_ref_names(list_name : &str, list_items : &LinkedHashMap<String, RustObject>, list_def_ref : &IndexMap<String, (Qv<String>, ValueType)>, root_renamed : &BTreeMap<String, String>) -> Result<()>{
    for (id, item) in list_items {
        if let Some(sabun_ref) = &item.refs {
            for (name, (_qv, def_vt)) in sabun_ref {
                //renameされていれば直す
                let name = root_renamed.get(name).map(|n| n.as_str()).unwrap_or(name);
                if let Some((_sab, sab_vt)) = list_def_ref.get(name) {
                    if def_vt.type_num() != sab_vt.type_num() {
                        Err(format!("{}'s ref member {}'s type doesn't correspond to list {}'s default object", id, name, list_name))?
                    }
                } else {
                    //renamedされていれば自動で追跡するのだが、デフォルトオブジェクトのRefは自動で追跡してくれないので、ちゃんと最新のに書き直さないとエラーになってしまう。
                    Err(format!("{}'s ref member {} doesn't correspond to list {}'s default object", id, name, list_name))?
                }
            }
        }
    }
    return Ok(());
}