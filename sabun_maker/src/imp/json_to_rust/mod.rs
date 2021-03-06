pub mod names;
pub mod json_obj_to_rust;
pub mod json_name;
pub mod tmp;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod array_null;
pub mod list;
pub mod get_old;
pub mod get_compatible;
pub mod get_id;
pub mod get_refs;
pub mod json_dir_to_rust;
pub mod construct_root;
pub mod validation;
use json5_parser::JVal;
use names::Names;
use crate::error::Result;
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::root_obj::RootObject;

pub fn json_root_to_rust(json : &str) -> Result<RootObject>{
    let jval = json5_parser::from_str(json)?;

    return match jval{
        JVal::Map(map, span) =>{
            let tmp = json_obj_to_rust::json_obj_to_rust(&map, false, &span, &Names::new(""))?;
            Ok(tmp.into_root_obj()?)
        },
        _ =>{
            Err(format!(r#"root object is not found"#))?
        }
    };
}

pub fn json_item_str_to_rust(json : &str, item_name : &str) -> Result<RustValue>{
    let jval = json5_parser::from_str(json)?;

    json_item_to_rust::json_item_to_rust(item_name, VarType::Normal, &jval, &Names::new(""))
}
