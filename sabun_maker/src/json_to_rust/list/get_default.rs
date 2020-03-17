use json5_parser::{JVal, Span};
use crate::json_to_rust::names::Names;
use crate::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::rust_struct::RustObject;
use crate::error::Result;
use indexmap::IndexMap;

pub fn get_default(array : &[JVal], span : &Span, names : &Names) -> Result<RustObject>{
    let error_message = r#"["Default", \{ default_obj \}] is valid"#;
    if array.len() != 1{
        Err(format!(r#"{} {} {} {}"#, span.line_col_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::Map(map, _) =>{
            Ok(get_default_obj(map, span, names)?)
        },
        _ => Err(format!(r#"{} {} {} {}"#, span.line_col_str(), span.slice(), error_message, names))?,
    }
}

fn get_default_obj(map : &IndexMap<String, JVal>, span : &Span, names : &Names) -> Result<RustObject>{
    let names = &names.append("default");
    let obj = json_obj_to_rust(map, names)?;
    if (&obj).id.is_none() == false{
        Err(format!("{} ID is not valid for default objects {}", span.line_col_str(), names))?
    }


    if obj.default.is_none() == false{
        unreachable!();
    }
    for (name, val) in obj.default.as_ref().unwrap(){
        
    }
    return Ok(obj);
}