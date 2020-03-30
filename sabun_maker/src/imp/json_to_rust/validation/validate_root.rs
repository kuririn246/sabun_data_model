use crate::structs::rust_object::RustObject;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_lists::validate_lists;
use crate::imp::json_to_rust::validation::validate_renamed::validate_renamed;
use crate::imp::json_to_rust::names::Names;

pub fn validate_root(root : &RustObject) -> Result<()>{
    validate_lists(&root)?;
    validate_renamed(&root, &Names::new("root"))?;
    return Ok(());
}