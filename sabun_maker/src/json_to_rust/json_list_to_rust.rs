use json5_parser::{JVal, Span};
use crate::rust_struct::{ValueType, RustValue, RustList, Qv, RustObject};
use crate::json_to_rust::names::Names;
use crate::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::error::Result;

pub fn json_list_to_rust(array : &[JVal], value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue> {
    let mut result = RustList::new();
    for item in array {
        let val = match item {
            JVal::Array(a2, _) => {
            },
            //JVal::Map(_, _) => unreachable!(),
            //JVal::Bool(_, _) => unreachable!(),
            _ =>{ todo!() }
        };
    }
    return Ok(RustValue::List(Qv::Val(result), value_type));
}

enum ListArrayItem{
    DefaultID(String),
    DefaultObj(RustObject),
    ListID(String),
    AutoID,
    RefListID(String),
    RefListIDs(RustObject),
}

fn list_item_array(array : &Vec<JVal>, span : &Span, names : &Names) -> Result<ListArrayItem>{
    let error_message = "List's array must be AutoID, RefListID, RefListIDs, Default or ListID";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(s, _) =>{
            match s.as_str(){
                "AutoID" =>{ Ok(ListArrayItem::AutoID) },
                "Default" =>{ get_default(&array[1..], span, names) },
                "RefListIDs" =>{ todo!() },
                "RefListID" =>{ todo!() },
                "ListID" =>{ todo!() },
                _ =>{
                    Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
                }
            }
        },
        _ =>{
            Err(format!("{} {} {} {}", span.line_col_str(), span.slice(), error_message, names))?
        }
    };
}

fn get_default(array : &[JVal], span : &Span, names : &Names) -> Result<ListArrayItem>{
    let error_message = r#"["Default", "id"] or ["Default", \{ default_obj \}] is valid"#;
    if array.len() != 1{
        Err(format!(r#"{} {} {} {}"#, span.line_col_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(id, _) => Ok(ListArrayItem::DefaultID(id.to_string())),
        JVal::Map(map, _) =>{
            Ok(ListArrayItem::DefaultObj(json_obj_to_rust(map, names)?))
        },
        _ => Err(format!(r#"{} {} {} {}"#, span.line_col_str(), span.slice(), error_message, names))?,
    }
}
