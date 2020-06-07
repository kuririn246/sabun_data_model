use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::def_obj::RefDefObj;
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::structs::qv::Qv;

pub fn adjust_mut_list_item_ref(def : &RefDefObj, old_ref : HashMap<String, RefSabValue>, _names : &Names) -> Result<HashMap<String, RefSabValue>>{
    let mut old_ref = old_ref;

    //事前に大きさが決定できないが、refのusecaseだとundefinedは少なく、default値のままが多いと思うので、sabunのlenを使う
    let mut result : HashMap<String, RefSabValue> = HashMap::with_capacity(old_ref.len());

    for (def_key, def_v) in def.refs(){
        let sabun_v = if let Some(v) = old_ref.remove(def_key){ v } else {
            if def_v.value_type().undefiable(){
                RefSabValue::new(Qv::Undefined)
            } else{
                continue;
            }
        };
        result.insert(def_key.to_string(), sabun_v);
    }
    Ok(result)
}