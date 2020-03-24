use json5_parser::{JVal, Span};
use super::super::names::Names;
use super::super::json_obj_to_rust::json_obj_to_rust;
use crate::error::Result;
use indexmap::IndexMap;
use crate::structs::rust_object::RustObject;

pub fn get_default(array : &[JVal], span : &Span, names : &Names) -> Result<RustObject>{
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

fn get_default_obj(map : &IndexMap<String, JVal>, span : &Span, names : &Names) -> Result<RustObject>{
    let names = &names.append("default");
    let obj = json_obj_to_rust(map, false, names)?;
    if (&obj).id.is_none() == false{
        Err(format!("{} ID is not valid for default objects {}", span.line_str(), names))?
    }

    if obj.default.is_none(){
        Err(format!("{} no default obj {}", span.line_str(), names))?
    }

    return Ok(obj);
}