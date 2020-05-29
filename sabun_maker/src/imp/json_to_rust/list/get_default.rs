use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::super::json_obj_to_rust::json_obj_to_rust;
use crate::error::Result;
use crate::structs::root_object::ListDefObj;
use linked_hash_map::LinkedHashMap;

pub fn get_default(array : &[JVal], span : &Span, names : &Names) -> Result<ListDefObj>{
    let error_message = r#"["Default", \{ default_obj \}] is valid"#;
    if array.len() != 1{
        Err(format!(r#"{} {} {} {}"#, span.line_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::Map(map, _) =>{
            Ok(get_default_obj(map, span, names)?)
        },
        _ => Err(format!(r#"{} {} {} {}"#, span.line_str(), span.slice(), error_message, names))?,
    }
}

fn get_default_obj(map : &LinkedHashMap<String, JVal>, span : &Span, names : &Names) -> Result<ListDefObj> {
    let names = &names.append("default");
    let obj = json_obj_to_rust(map, span, names)?;
    if obj.id.is_some() {
        Err(format!("{} ID is not valid for Default {}", span.line_str(), names))?
    }
    if obj.include.len() != 0 {
        Err(format!("{} Include is not valid for Default {}", span.line_str(), names))?
    }


    return Ok(ListDefObj { default: obj.default, refs: obj.refs.to_ref_def(), old: obj.old })
}