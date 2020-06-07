use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_compatible::validate_compatible;
use crate::imp::json_to_rust::validation::validate_ref_def::validate_ref_def;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_def_mem;
use crate::imp::structs::def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::list_value::ListDefValue;

///ちゃんとDataDef型になっているかどうか、RefDefがちゃんとしてるかどうか、Compatibleかどうか、などDefについて調べる
/// Itemについて調べるvalidate_list/data/mut_listとは違う
/// is_mutの場合、InnerMutしか認めない
pub(crate) fn validate_list_def(def : &ListDefObj, root : &RootObject, can_use_old : bool, is_mut : bool, names : &Names) -> Result<()> {
    validate_ref_def(def.refs(), names)?;
    validate_old_def_mem(def.old(), def.default(), names)?;

    for (name, val) in def.default() {
        match val {
            ListDefValue::InnerDataDef(d) => {
                if is_mut{
                    Err(format!("{} {} MutList can't have InnerData", names, name))?;
                }
                validate_list_def(d, root, can_use_old, is_mut,&names.append(name))?;
            },
            ListDefValue::InnerMutDef(d) => {
                if is_mut == false{
                    Err(format!("{} {} Data/List can't have InnerMutList", names, name))?;
                }
                let names = &names.append(name);
                validate_compatible(d.list_def(), d.compatible(), root, can_use_old, names)?;
                validate_list_def(d.list_def(), root, can_use_old, is_mut, names)?;
            },
            ListDefValue::InnerListDef(d) => {
                if is_mut {
                    Err(format!("{} {} MutList can't have InnerList", names, name))?;
                }
                validate_list_def(d, root, can_use_old, is_mut,&names.append(name))?;
            },
            ListDefValue::Param(_,_) => {},
            //_ => { Err(format!("{} {} can't be defined here", names, name))? }
        }
    }

    Ok(())
}