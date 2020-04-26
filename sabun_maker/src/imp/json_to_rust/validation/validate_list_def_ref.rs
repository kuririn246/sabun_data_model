use crate::error::Result;
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;

pub fn validate_list_def_ref(
    list_name : &str,
    list_def_ref : &IndexMap<String, RefValue>,
    root_def : &IndexMap<String, RustValue>) -> Result<()>{

    for (ref_name, _value) in list_def_ref{
        //let renamed = root_rename.get(ref_name).map(|n| n.as_str()).unwrap_or(ref_name); json時点で
        // renameに頼るのは認めないことにする

        if let Some(value) = root_def.get(ref_name){
            match value{
                RustValue::List(_l) =>{},
                _ =>{ Err(format!("list {}'s default object's ref member {} doesn't correspond to the root object", list_name, ref_name))? }
            }
        } else{
            Err(format!("list {}'s default object's ref member {} doesn't correspond to the root object", list_name, ref_name))?
        }
    }
    return Ok(())
}