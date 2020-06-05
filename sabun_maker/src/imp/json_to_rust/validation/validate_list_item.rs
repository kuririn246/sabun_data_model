use std::collections::{HashMap};
use crate::structs::rust_value::{RustValue};
use crate::structs::root_object::{RootObject, ListDefObj};
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_data::validate_data;
use crate::imp::json_to_rust::validation::validate_list::validate_list;
use crate::imp::json_to_rust::validation::validate_refs::validate_refs;
use crate::structs::ref_value::RefValue;
use crate::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;

pub fn validate_list_item(def : &ListDefObj, sabun_values : &HashMap<String, RustValue>,
                          ref_values : &HashMap<String, RefValue>, root : &RootObject,
                          can_use_old: bool, names : &Names) -> Result<()> {
    validate_refs(def.refs(), ref_values, root, can_use_old, names)?;

    for (name, val) in sabun_values {
        if can_use_old == false && def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }
        let def_value = if let Some(def) = def.default().get(name) { def } else {
            Err(format!("{} there's no default members named {}", names, name))?
        };
        if def_value.acceptable(val) == false {
            Err(format!("{} {} the default value doesn't correspond to the list item's value", names, name))?
        }
        //inner listは中までしっかり調べる必要があるわね
        match def_value {
            RustValue::InnerDataDef(def) => {
                if let RustValue::InnerData(data) = val {
                    validate_data(def, data.list(), root, data.old(), can_use_old, &names.append(name))?
                } else {
                    //correspondしてることは確認済みである
                    unreachable!();
                }
            },
            RustValue::InnerListDef(def) => {
                if let RustValue::InnerList(list) = val {
                    validate_list(def, list.list(), root, can_use_old, &names.append(name))?
                } else { unreachable!(); }
            },
            RustValue::InnerMutDef(def) => {
                let list = if let RustValue::InnerMut(list) = val { list } else { unreachable!() };
                match list {
                    Some(list) => {
                        validate_mut_list(def.list_def(), list.list(), root, can_use_old, &names.append(name))?
                    },
                    None => {
                        if def.undefinable() == false {
                            Err(format!("{} {} can't be undefined", names, name))?
                        }
                    }
                }
            },
            _ =>{}
        }
    }

    Ok(())
}