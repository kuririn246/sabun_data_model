use crate::structs::root_object::RustObject;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;

pub fn validate_renamed(obj : &RustObject, names : &Names) -> Result<()>{
    for (key, value) in &obj.renamed{
        if obj.default.contains_key(value) == false{
            Err(format!(r#"{} Renamed "{}->{}": {} doesn't exist"#, names, key, value, value))?
        }
    }
    return Ok(());
}