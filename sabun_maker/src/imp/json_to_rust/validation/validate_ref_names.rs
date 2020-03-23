use crate::rust_struct::{RustObject, Qv, ValueType};
use crate::error::Result;
use indexmap::IndexMap;

///list_itemsのRefの名前と型がDefault objectのRefと一致してるか調べる
pub fn validate_ref_names(list_name : &str, list_items : &[RustObject], list_def_ref : &IndexMap<String, (Qv<String>, ValueType)>) -> Result<()>{
    for item in list_items {
        if let Some(sabun_ref) = &item.refs {
            for (name, (qv, def_vt)) in sabun_ref {
                if let Some((sab, sab_vt)) = list_def_ref.get(name) {
                    if def_vt.type_num() != sab_vt.type_num() {
                        Err(format!("{}'s ref member {}'s type doesn't correspond to list {}'s default object", get_id(item), name, list_name))?
                    }
                } else {
                    Err(format!("{}'s ref member {} doesn't correspond to list {}'s default object", get_id(item), name, list_name))?
                }
            }
        }
    }
    return Ok(());
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}