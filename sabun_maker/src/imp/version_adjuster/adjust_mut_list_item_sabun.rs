use crate::structs::root_object::ListDefObj;
use crate::structs::rust_value::RustValue;
use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_mut_list::adjust_inner_mut_list;

pub fn adjust_mut_list_item_sabun(def : &ListDefObj, old_sabun : HashMap<String, RustValue>, names : &Names) -> Result<HashMap<String, RustValue>>{
    let mut old_sabun = old_sabun;
    //おおむねold_sabun.len()でいいはず
    let mut result : HashMap<String, RustValue> = HashMap::with_capacity(old_sabun.len());
    for (def_key, _def_v) in def.default(){
        let sabun_v = if let Some(v) = old_sabun.remove(def_key){ v } else { continue; };
        match sabun_v{
            RustValue::Param(p, v) =>{
                //型チェックめんどいからなしでいいかな・・・？
                result.insert(def_key.to_string(), RustValue::Param(p, v));
            },
            RustValue::InnerMut(op) =>{
                match op{
                    Some(im) =>{
                        let new = adjust_inner_mut_list(def, im, &names.append(def_key))?;
                        result.insert(def_key.to_string(), RustValue::InnerMut(Some(new)));
                    },
                    None =>{
                        result.insert(def_key.to_string(), RustValue::InnerMut(None));
                    }
                }
            },
            _ =>{ Err(format!("{} {} mut list items can only have Param or InnerMut", names, def_key))? }
        }
    }
    Ok(result)
}