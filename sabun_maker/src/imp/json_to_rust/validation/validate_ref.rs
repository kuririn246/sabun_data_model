//use std::collections::{BTreeMap};
use crate::error::Result;
use crate::indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;
use crate::structs::root_object::RustObject;
use crate::structs::rust_value::RustValue;
use crate::structs::rust_list::RustList;

///参照先が存在し、Obsoleteされてないか調べる。自分自身がObsoleteである場合、参照先がObsoleteでも良い。
pub fn validate_ref(list_name : &str,
                list_items : &LinkedHashMap<String, RustObject>,
                root_def : &IndexMap<String, RustValue>,
                //list_def_ref : &HashMap<String, (Qv<String>, ValueType)>,
                //rename : &BTreeMap<String, String>
                    ) -> Result<()> {
    for (id, item) in list_items {
        let sabun_refs = &item.refs;
        for (ref_list_name, rv) in sabun_refs {
            if let Some(reference) = rv.get_reference() {
                if let Some(l) = get_root_list(ref_list_name, root_def) {
                    match check_if_list_have_id_and_obsolete(l, reference, item.obsolete) {
                        Cilhiao::NotFound => Err(format!("list {} doesn't have id {}, list {} id {}", ref_list_name, reference, list_name, id))?,
                        Cilhiao::Obsolete => Err(format!("list {}'s {} is obsolete, list {} id {}", ref_list_name, reference, list_name, id))?,
                        Cilhiao::Ok => {},
                    }
                } else {
                    Err(format!("There's no list named {}, list {} id {}", ref_list_name, list_name, id))?
                }
            } else {
                //referenceがない場合チェックすることもない
            }
        }
    }
    return Ok(());
}


enum Cilhiao{
    Ok,
    Obsolete,
    NotFound
}
fn check_if_list_have_id_and_obsolete(list : &RustList, id : &str, obsolete : bool) -> Cilhiao{
    //redefは認めるべき？　よくわからない・・・
    let id = list.redef.get(id).map(|n| n.as_str()).unwrap_or(id);
    if let Some(item) = list.list.get(id){
        if item.obsolete{
            //相手がobsoleteでも、自分もobsoleteなら見逃す
            if obsolete{
                return Cilhiao::Ok;
            } else{
                return Cilhiao::Obsolete;
            }
        } else{
            return Cilhiao::Ok;
        }
    } else{
        return Cilhiao::NotFound;
    }
}

fn get_root_list<'a>(
    name : &'a str,
    root_def : &'a IndexMap<String, RustValue>,
    //rename : &BTreeMap<String, String>
) -> Option<&'a RustList>
{
    //let name = rename.get(name).map(|n| n.as_str()).unwrap_or(name);

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
