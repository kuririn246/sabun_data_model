use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_list_defaults::validate_list_defaults;
use crate::imp::json_to_rust::validation::validate_ref_names::validate_ref_names;
use linked_hash_map::LinkedHashMap;
use crate::imp::json_to_rust::validation::validate_ref::validate_ref;
use crate::imp::json_to_rust::validation::validate_list_def_ref::validate_list_def_ref;
use crate::structs::rust_object::RustObject;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_renamed::validate_renamed;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_list::ListDef;

pub fn validate_lists(root : &RustObject, current : &RustObject, is_inner_list : bool, names : &Names) -> Result<()>{

    for (name, value) in &current.default{
        let names = &names.append(name);

        if let RustValue::List(l) = value {
            match &l.default {
                ListDef::Def(list_def) => {
                    validate_renamed(list_def, &names.append("Default"))?;

                    validate_list_defaults(&names, &list_def.default, &l.list, &list_def.renamed)?;

                    if list_def.refs.len() != 0 {
                        let refs = &list_def.refs;
                        validate_ref_names(name, &l.list, refs)?;
                        validate_ref(name, &l.list, &root.default)?;
                        validate_list_def_ref(name, refs, &root.default)?;
                    } else {
                        if let Some(id) = check_if_items_have_ref(&l.list) {
                            Err(format!("{}'s {} has Ref", names, id))?
                        }
                    }
                },
                ListDef::Rent(list_name) =>{
                    match root.default.get(list_name){
                        Some(RustValue::List(l)) =>{

                        },
                        _ =>{
                            Err(format!("{} there's no list named {}", names, list_name))?
                        }
                    }
                },
                ListDef::InnerList =>{
                    if is_inner_list == false{
                        Err(format!("{}'s default object needs to be defined", names))?
                    }
                }
            }
        }

    }
    return Ok(());
}

fn check_if_items_have_ref(list_items : &LinkedHashMap<String, RustObject>) -> Option<&str>{
    for (id, item) in list_items{
        if item.refs.len() != 0{
            return Some(id);
        }
    }
    return None;
}