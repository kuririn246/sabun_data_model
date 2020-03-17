use json5_parser::{JVal, Span};
use crate::json_to_rust::names::Names;
use crate::rust_struct::RustObject;
use crate::error::Result;
use crate::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use indexmap::IndexMap;

pub fn get_list_items(array : &[JVal], need_id : bool,  span : &Span, names : &Names) -> Result<Vec<RustObject>>{
    let mut result : Vec<RustObject> = vec![];
    for index in 0..array.len(){
        let item = &array[index];
        match item{
            JVal::Map(map, span) =>{
                result.push(get_list_item(map, need_id, index, span, names)?)
            },
            _ =>{
                Err(format!(r#"{} List's object sequence must not be interrupted {}"#, item.span().line_str(), names))?
            }
        }
    }
    return Ok(result);
}

pub fn get_list_item(map : &IndexMap<String, JVal>, need_id : bool, index : usize, span : &Span, names : &Names) -> Result<RustObject>{
    let mut obj = json_obj_to_rust(map, names)?;
    let map = obj.default.take();
    obj.default = None;
    obj.sabun = map.unwrap();

    if need_id{
        if obj.id.is_none(){
            Err(format!(r#"{} ID is missing {}"#, span.line_str(), names))?
        }
    } else{
        if obj.id.is_none(){
            obj.id = Some(index.to_string());
        } else{
            Err(format!(r#"{} ID is invalid for AutoID lists {}"#, span.line_str(), names))?
        }

    }
    if obj.renamed.is_empty() == false{
        Err(format!(r#"{} Renamed is not valid for list objects {}"#, span.line_str(), names))?
    }
    return Ok(obj)
}