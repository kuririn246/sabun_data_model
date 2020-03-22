use std::collections::HashMap;
use crate::rust_struct::{RustValue, RustObject, ValueType, Qv, ArrayType};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use indexmap::IndexMap;

pub fn validate_and_final_touch(root : RustObject, map : HashMap<String, RustValue>) -> Result<RustObject>{
    let mut root = root;
    for (key, value) in map{
        let name = json_name(&key).ok_or_else(|| format!("{} is not a valid name", &key))?;
        match name {
            NameType::Name(name, ValueType::Normal) => {
                root.include.push(key);
                root.insert_default(name, value);
            },
            _=>{ Err(format!("{} is not a valid name", &key))?; }
        }
    }

    return Ok(root);
}

fn validate_lists(root : &RustObject) -> Result<()>{
    if root.default.is_none(){ return Ok(()); }

    let root_def = root.default.as_ref().unwrap();
    for (name, value) in root_def{
        let name : &str = name;
        let value : &RustValue = value;

        if let RustValue::List(l) = value {
            let list_def = &l.default;
            //unwrapは絶対に成功するはずだが、データ型はそう言ってないのでデータ型に従ってコーディングする。
            validate_list_sabuns(name, list_def.default.as_ref().ok_or_else(|| format!("list {} doesn't have default obj", name))?, &l.list);
            if let Some(refs) = &list_def.refs {
                validate_ref_names(name, &l.list, refs);
                //validate_refs()
            } else{
                if let Some(id) = check_if_items_have_ref(&l.list){
                    Err(format!("{}'s {} has Ref", name, id))?
                }
            }
        }

    }
    return Ok(());
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}

fn validate_list_sabuns(list_name : &str, list_def : &IndexMap<String, RustValue>, list_items : &[RustObject]) -> Result<()>{
    for item in list_items{
        for (name, val) in &item.sabun{
            let name : &str = name;
            let sabun_val : &RustValue = val;

            let def_val = list_def.get(name).ok_or_else(|| format!("{}'s default obj doesn't have {} .{}", list_name, name, get_id(item)))?;

            if val_type_check(sabun_val, def_val) == false{
                Err(format!("list {}'s default value's type doesn't correspond to {}'s {}", list_name, get_id(item), name))?
            }
        }
    }
    return Ok(());
}

fn val_type_check(l : &RustValue, r : &RustValue) ->bool{
    if l.type_num() != r.type_num(){
        return false
    }
    if l.value_type().type_num() != r.value_type().type_num(){
        return false
    }

    if let RustValue::Array(_, l_at, _) = l {
        if let RustValue::Array(_, r_at, _) = r {
            if l_at.type_num() != r_at.type_num() {
                return false
            }
        }
    }
    return true;
}

///list_itemsのRefの名前と型がDefault objectのRefと一致してるか調べる
fn validate_ref_names(list_name : &str, list_items : &[RustObject], list_def_ref : &IndexMap<String, (Qv<String>, ValueType)>) -> Result<()>{
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

fn check_if_items_have_ref(list_items : &[RustObject]) -> Option<&str>{
    for item in list_items{
        if item.refs.is_some(){
            return Some(item.id.as_ref().unwrap());
        }
    }
    return None;
}

///参照先が存在し、Obsoleteされてないか調べる
fn validate_ref(list_name : &str, list_items : &[RustObject], root_def : &HashMap<String, RustValue>, list_def_ref : &HashMap<String, (Qv<String>, ValueType)>) -> Result<()> {
    for item in list_items{
        let sabun = item.refs;
        for (name, (qv, vt)) in list_def_ref{

        }
    }
}
