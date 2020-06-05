use crate::structs::root_object::{ListDefObj, RootObject};
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_compatible::validate_compatible;
use crate::imp::json_to_rust::validation::validate_ref_def::validate_ref_def;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_def_mem;

///ちゃんとDataDef型になっているかどうか、RefDefがちゃんとしてるかどうか、Compatibleかどうか、などDefについて調べる
/// Itemについて調べるvalidate_list/data/mut_listとは違う
pub fn validate_list_def(def : &ListDefObj, root : &RootObject, can_use_old : bool, names : &Names) -> Result<()> {
    validate_ref_def(def.refs(), names)?;
    validate_old_def_mem(def.old(), def.default(), names)?;

    for (name, val) in def.default() {
        match val {
            RustValue::InnerDataDef(d) => {
                validate_list_def(d, root, can_use_old,&names.append(name))?;
            },
            RustValue::InnerMutDef(d) => {
                let names = &names.append(name);
                validate_compatible(d.list_def(), d.compatible(), root, can_use_old, names)?;
                validate_list_def(d.list_def(), root, can_use_old, names)?;
            },
            RustValue::InnerListDef(d) => {
                validate_list_def(d, root, can_use_old,&names.append(name))?;
            },
            RustValue::Param(_,_) => {},
            _ => { Err(format!("{} {} can't be defined here", names, name))? }
        }
    }

    Ok(())
}