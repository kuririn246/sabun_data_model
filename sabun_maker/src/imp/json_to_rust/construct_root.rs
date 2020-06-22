use crate::imp::json_to_rust::json_name::{json_name, NameType};
use crate::HashM;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::structs::value_type::VarType;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;

pub fn construct_root(root : RootObject, map : HashM<String, RootValue>, validation : bool) -> Result<RootObject>{
    let mut default : HashM<String, RootValue> = root.default().clone();
    for (key, value) in map{
        let name = json_name(&key).ok_or_else(|| format!("filename:{} is not a valid name", &key))?;
        match name {
            NameType::Name(name, VarType::Normal) => {
                default.insert(name, value);
            },
            _=>{ Err(format!("{} is not a valid name", &key))?; }
        }
    }
    let root = RootObject::new(default, root.sabun().clone(), root.old().clone());
    if validation{
        validate_root(&root, false)?
    }

    return Ok(root);
}