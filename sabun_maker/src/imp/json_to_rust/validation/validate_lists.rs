use crate::rust_struct::{RustObject, RustValue};
use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_list_sabuns::validate_list_sabuns;
use crate::imp::json_to_rust::validation::validate_ref_names::validate_ref_names;

pub fn validate_lists(root : &RustObject) -> Result<()>{
    if root.default.is_none(){ return Ok(()); }

    let root_def = root.default.as_ref().unwrap();
    for (name, value) in root_def{
        let name : &str = name;
        let value : &RustValue = value;

        if let RustValue::List(l) = value {
            let list_def = &l.default;
            //unwrapは絶対に成功するはずだが、データ型はそう言ってないのでデータ型に従ってコーディングする。
            validate_list_sabuns(name, list_def.default.as_ref().ok_or_else(|| format!("list {} doesn't have default obj", name))?, &l.list, &l.default.renamed);
            if let Some(refs) = &list_def.refs {
                validate_ref_names(name, &l.list, refs, &root.renamed);
                //validate_refs()
            } else{
                if let Some(id) = check_if_items_have_ref(&l.list){
                    Err(format!("{}'s {} has Ref", name, id))?
                }
            }
        }

    }
    return Ok(());
}

fn check_if_items_have_ref(list_items : &[RustObject]) -> Option<&str>{
    for item in list_items{
        if item.refs.is_some(){
            return Some(item.id.as_ref().unwrap());
        }
    }
    return None;
}