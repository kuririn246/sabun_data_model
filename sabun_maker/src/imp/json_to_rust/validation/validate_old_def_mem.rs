use crate::{HashM, HashS};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_def_obj::ListDefMap;
use crate::imp::structs::ref_def_obj::RefDefMap;

pub fn validate_old_root_def_mem(old : &HashS<String>, map : &HashM<String, RootValue>, names : &Names) -> Result<()>{
    for name in old{
        if map.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Default", names, name))?
        }
    }
    Ok(())
}

pub fn validate_old_list_def_mem(old : &HashS<String>, map : &ListDefMap, names : &Names) -> Result<()>{
    for name in old{
        if map.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Default", names, name))?
        }
    }
    Ok(())
}

pub fn validate_old_ref_def(old : &HashS<String>, ref_def : &RefDefMap, names : &Names) -> Result<()>{
    for name in old{
        if ref_def.contains_key(name) == false{
            Err(format!("{} Old {} does not exist in the Ref", names, name))?
        }
    }
    Ok(())
}

pub fn validate_old_data_id(old : &HashS<String>, data_map : &HashM<String, ConstItem>, names : &Names) -> Result<()> {
    for name in old{
        if data_map.contains_key(name) == false{
            Err(format!("{} Old's ID {} does not exist in the Data", names, name))?
        }
    }
    Ok(())
}