use json5_parser::{JVal, Span};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use linked_hash_map::LinkedHashMap;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;

pub fn get_list_items(array : &[JVal], need_id : bool,  _span : &Span, names : &Names) -> Result<Vec<TmpObj>>{
    let mut result : Vec<TmpObj> = vec![];
    for index in 0..array.len(){
        let item = &array[index];
        match item{
            JVal::Map(map, span) =>{
                let (id,item) = get_list_item(map, need_id, index, span, names)?;
                result.insert(id, item);
            },
            _ =>{
                Err(format!(r#"{} List's object sequence must not be interrupted {}"#, item.span().line_str(), names))?
            }
        }
    }
    return Ok(result);
}

pub fn get_list_item(map : &LinkedHashMap<String, JVal>, need_id : bool, index : usize, span : &Span, names : &Names) -> Result<(String, TmpObj)>{
    let mut obj = json_obj_to_rust(map, false, names)?;

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
    return Ok((obj.id.as_ref().unwrap().to_string(), obj))
}