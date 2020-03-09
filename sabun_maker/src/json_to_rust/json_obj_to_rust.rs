use json5_parser::JVal;
use std::collections::BTreeMap;
use super::names::Names;
use crate::rust_struct::RustObject;
use super::json_name::{json_name, NameType, SystemNames};
use super::json_item_to_rust::json_item_to_rust;
use crate::json_to_rust::get_ref_ids::get_ref_ids;
use crate::json_to_rust::get_rename::get_rename;
use crate::error::Result;


pub fn json_obj_to_rust(v : &BTreeMap<String, JVal>, names : &Names) -> Result<RustObject>{
    let mut r : RustObject = RustObject::new();
    for (k,v) in v{
        let name = json_name(k).ok_or_else(|| format!("{} {} is not a valid name {}",v.line_col(), k, names))?;
        match name{
            NameType::Name(name, vt) =>{
                let v = json_item_to_rust(&name, vt,v, names)?;
                r.insert_default(k.to_string(), v);
            },
            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            r.id = Some(v.as_str().ok_or_else(|| format!("{} ID must be string : {} {}", v.line_col(), v.original(), names))?.to_string())
                        } else{
                            Err(format!("{} ID is defined multiple times {}", v.line_col(), names))?;
                        }
                    },
                    SystemNames::Include=>{
                        //TODO: implement "Include"
                    },
                    SystemNames::RefID =>{
                        if r.ref_ids.is_none(){
                            let id = v.as_str().ok_or_else(|| format!("{} RefID must be string : {} {}", v.line_col(), v.original(), names))?.to_string();
                            let mut map : BTreeMap<String, Option<String>> = BTreeMap::new();
                            map.insert("RefID".to_string(), Some(id));
                            r.ref_ids = Some(map);
                        } else {
                            Err(format!("{} RefID is defined multiple times {}", v.line_col(), names))?;
                        }
                    },
                    SystemNames::RefIDs =>{
                        if r.ref_ids.is_none(){
                            r.ref_ids = Some(get_ref_ids(v, names)?);
                        } else {
                            Err(format!("{} RefIDs is defined multiple times {}", v.line_col(), names))?;
                        }
                    },
                    SystemNames::Rename =>{
                        if r.rename.len() == 0{
                            r.rename = get_rename(v, names)?;
                        } else{
                            //そもそも複数回の定義はjsonパーサーによって弾かれるはずだが・・・
                            Err(format!("{} Rename is defined multiple times {}", v.line_col(), names))?;
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}