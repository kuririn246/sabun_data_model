use std::collections::{HashMap, BTreeMap};
use crate::rust_struct::{RustValue, RustObject, ValueType, Qv, ArrayType, RustList};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;

///参照先が存在し、Obsoleteされてないか調べる。自分自身がObsoleteである場合、参照先がObsoleteでも良い。
pub fn validate_ref(list_name : &str,
                list_items : &LinkedHashMap<String, RustObject>,
                root_def : &IndexMap<String, RustValue>,
                //list_def_ref : &HashMap<String, (Qv<String>, ValueType)>,
                rename : &BTreeMap<String, String>) -> Result<()> {
    for (id, item) in list_items{
        if let Some(sabun_refs) = &item.refs {
            for (name, (qv, vt)) in sabun_refs {
                if let Some(reference) = get_reference(qv, vt) {
                    let name = rename.get(name).map(|n| n.as_str()).unwrap_or(name);
                    if let Some(l) = get_root_list(name, root_def, rename) {
                        if check_if_list_have_id(l, reference) == false {
                            Err(format!("list {} doesn't have id {}, list {} id {}", name, reference, list_name, id))?
                        }
                    } else {
                        Err(format!("There's no list named {}, list {} id {}", name, list_name, id))?
                    }
                } else{
                    //referenceがない場合チェックすることもない
                }
            }
        } else{
            //refがないのでチェックするようなことはない
        }
    }
    todo!()
}

///nullやundefinedの場合None
fn get_reference<'a>(qv : &'a Qv<String>, vt : &'a ValueType) -> Option<&'a str>{
    match qv{
        Qv::Val(s) => Some(s.as_str()),
        Qv::Null => None,
        Qv::Undefined => None,
    }
}

fn check_if_list_have_id(list : &RustList, id : &str) -> bool{
    let id = list.redef.get(id).map(|n| n.as_str()).unwrap_or(id);
    list.list.contains_key(id)
}

fn get_root_list<'a>(
    name : &'a str,
    root_def : &'a IndexMap<String, RustValue>,
    rename : &'a BTreeMap<String, String>) -> Option<&'a RustList>{

    let name = rename.get(name).map(|n| n.as_str()).unwrap_or(name);
    if let Some(value) = root_def.get(name){
        match value{
            RustValue::List(l) =>{
                return Some(l);
            },
            _ =>{},
        }
    }
    return None;
}

fn get_id(obj : &RustObject) -> String{
    obj.id.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "no id").to_string()
}
