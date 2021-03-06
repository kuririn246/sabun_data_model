use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::super::json_obj_to_rust::json_obj_to_rust;
use crate::error::Result;
use crate::imp::structs::list_def_obj::ListDefObj;
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
    let obj = json_obj_to_rust(map, false, span, names)?;

    return obj.into_list_def_obj();
}