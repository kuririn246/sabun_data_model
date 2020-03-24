use json5_parser::{JVal, Span};
use indexmap::IndexMap;
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;

pub fn get_refs(v : &IndexMap<String, JVal>, span : &Span, names : &Names) -> Result<IndexMap<String, RefValue>> {
    let obj = json_obj_to_rust(v, true, names)?;
    if obj.refs.is_some(){
        Err(format!(r#"{} Ref can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.id.is_some(){
        Err(format!(r#"{} ID can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.renamed.len() != 0{
        Err(format!(r#"{} Renamed can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.obsolete{
        Err(format!(r#"{} Obsolete can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if let Some(def) = obj.default.as_ref(){
        let mut map : IndexMap<String, RefValue> = IndexMap::new();
        for (_k,_v) in def{
            let k : &String = _k;
            let v : &RustValue = _v;
            match v{
                RustValue::String(v, vt) =>{
                    map.insert(k.to_string(), RefValue::new(v.clone(), vt.clone()));
                },
                _ =>{
                    Err(format!(r#"{} {} Ref's value must be string or null {}"#, span.line_str(), k, names))?
                }
            }
        }
        return Ok(map);
    } else{
        unreachable!();
    }
}

//fn get_name(name : &str) -> Option<String>{
//    let name_type = json_name(name)?;
//    match &name_type{
//        NameType::Normal => return Some(name_type),
//        _ => return None,
//    };
//}