use std::collections::HashMap;
use crate::rust_struct::{RustValue, RustObject, ValueType, Qv, ArrayType};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{json_name, NameType};

pub fn validate_and_final_touch(root : RustObject, map : HashMap<String, RustValue>) -> Result<RustObject>{
    let mut root = root;
    for (key, value) in map{
        let name = json_name(&key).ok_or_else(|| Err(format!("{} is not a valid name", &key)))?;
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

fn validate_refs(root : &RustObject) -> Result<()>{
    root.default.is_none(){ return Ok(()); }

    let root_def = root.default.as_ref().unwrap();
    for (name, value) in def{
        let name : &str = name;
        let value : &RustValue = value;

        if let RustValue::List(l, vt) = value {
            if let Qv::Val(l) = l {
                let list_def = &l.default;
                //unwrapは絶対に成功するはずだが、データ型はそう言ってないのでデータ型に従ってコーディングする。
                validate_list_sabuns(name, list_def.default.as_ref().unwrap_or_else(|| IndexMap::new()), &l.list);


            }
        }

    }
    return Ok(());
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}

fn validate_list_sabuns(list_name : &str, list_def : &HashMap<String, RustValue>, list_items : &[RustObject]) -> Result<()>{
    for item in list_items{
        for (name, val) in &item.sabun{
            let name : &str = name;
            let sabun_val : &RustValue = val;

            let def_val = list_def.get(name).ok_or_else(|| format!("{}'s default obj doesn't have {} .{}", list_name, name, get_id(item)))?;

            if val_type_check(sabun_val, def_val) == false{
                Err(format!("list {}'s default value's type doesn't correspond to {}'s {}", list_name, item.id, name))?
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

fn validate_ref_maps(root : HashMap<String, RustValue>, sabun_ref : HashMap<String, (Qv<String>, ValueType)>, list_ref : HashMap<String, (Qv<String>, ValueType)>) -> Result<()>{
    todo!();
}