use crate::structs::root_object::RootObject;
use crate::error::Result;
use crate::imp::json_to_rust::json_name::dot_chained_name;
use crate::imp::json_to_rust::names::Names;

pub fn validate_ref_from_root(dot_chained : &str, root : &RootObject, names : &Names) -> Result<()>{
    if dot_chained.is_empty(){
        Err(format!("{} is empty", names))?
    }
    match dot_chained_name(dot_chained){
        None =>{ Err(format!("{} {} is not a dot-chained name", names, dot_chained))? }
    }
}