use crate::error::Result;
use indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;
use std::collections::BTreeMap;
use crate::structs::rust_object::RustObject;
use crate::structs::ref_value::RefValue;

///list_itemsのRefの名前と型がDefault objectのRefと一致してるか調べる
///Renamedされていれば自動で追跡するのだが、ListのデフォルトオブジェクトのRefは自動で追跡してくれないので、DefaultのRefだけはちゃんと最新のに書き直さないとエラーになってしまうことになっている
pub fn validate_ref_names(list_name : &str, list_items : &LinkedHashMap<String, RustObject>, list_def_ref : &IndexMap<String, RefValue>, root_renamed : &BTreeMap<String, String>) -> Result<()>{
    for (id, item) in list_items {
        if let Some(sabun_ref) = &item.refs {
            for (name, sab_rv) in sabun_ref {
                //renameされていれば直す
                let name = root_renamed.get(name).map(|n| n.as_str()).unwrap_or(name);
                if let Some(rv) = list_def_ref.get(name) {
                    let rv : &RefValue = rv;
                    if rv.value_type.acceptable(&sab_rv.get_value().qv_type()) == false{
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