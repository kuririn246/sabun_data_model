use crate::structs::root_object::{ RefDefObj};
use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::structs::ref_value::RefValue;

pub fn adjust_mut_list_item_ref(def : &RefDefObj, old_ref : HashMap<String, RefValue>, _names : &Names) -> Result<HashMap<String, RefValue>>{
    let mut old_ref = old_ref;
    //おおむねold_sabun.len()でいいはず
    let mut result : HashMap<String, RefValue> = HashMap::with_capacity(old_ref.len());
    for (def_key, _def_v) in def.refs(){
        let sabun_v = if let Some(v) = old_ref.remove(def_key){ v } else { continue; };
        result.insert(def_key.to_string(), sabun_v);
    }
    Ok(result)
}