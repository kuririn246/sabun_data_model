use json5_parser::{JVal, Span};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::structs::rust_value::{RustValue, RustParam};
use crate::structs::ref_value::RefValue;
use crate::imp::json_to_rust::json_name::json_simple_name;
use crate::structs::qv::Qv;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpRefs;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

pub fn get_ref(v : &LinkedHashMap<String, JVal>, span : &Span, names : &Names) -> Result<TmpRefs> {
    let obj = json_obj_to_rust(v, true, span, names)?;
    if obj.refs.map.len() != 0 {
        Err(format!(r#"{} Ref can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.id.is_some() {
        Err(format!(r#"{} ID can't be declared in a Ref object {}"#, span.line_str(), names))?
    }
    if obj.include.len() != 0{
        Err(format!(r#"{} Include can't be declared in a Ref object {}"#, span.line_str(), names))?
    }


    let mut map: HashMap<String, RefValue> = HashMap::with_capacity(obj.default.len());
    for (k, v) in &obj.default {
        match v {
            RustValue::Param(RustParam::String(v), vt) => {
                match v {
                    Qv::Val(s) =>{
                        if json_simple_name(s).is_none() && s.is_empty() == false{
                            //undefinedは勝手にいれちゃいけないから、エラーメッセージには表示しないが、別に入れられる
                            Err(format!(r#"{} {} Ref's value must be a simple name, null or empty {}"#, span.line_str(), s, names))?
                        }
                    },
                    _ =>{},
                }
                map.insert(k.to_string(), RefValue::new(v.clone(), vt.clone()));

            },
            _ => {
                Err(format!(r#"{} {} Ref's value must be a string or null {}"#, span.line_str(), k, names))?
            }
        }
    }
    return Ok(TmpRefs{ map, old : obj.old, is_enum : false, span : span.clone() });
}


//fn get_name(name : &str) -> Option<String>{
//    let name_type = json_name(name)?;
//    match &name_type{
//        NameType::Normal => return Some(name_type),
//        _ => return None,
//    };
//}