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
        let name = json_name(k).ok_or(format!("{} {} is not a valid name {}",v.span(), k, names.to_string()))?;
        match name{
            NameType::Normal =>{
                let v = json_item_to_rust(k,v, names)?;
                r.insert_default(k.to_string(), v);
            },

            NameType::Nullable(ref s) =>{
                let v = json_item_to_rust(s,v, names)?;
                r.insert_default(s.to_string(), v);
            }

            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            r.id = Some(v.as_str().ok_or(format!("{} ID must be string : {} {}", v.span(), v.span().slice(), names.to_string()))?.to_string())
                        } else{
                            return Err(format!("ID is defined multiple times {}", names.to_string()));
                        }
                    },
                    SystemNames::Include=>{
                        //TODO: implement "Include"
                    },
                    SystemNames::RefID =>{
                        if r.ref_ids.is_none(){
                            let id = v.as_str().ok_or(format!("RefID must be string : {} {}", v, names.to_string()))?.to_string();
                            let mut map : BTreeMap<String,String> = BTreeMap::new();
                            map.insert("RefID".to_string(), id);
                            r.ref_ids = Some(map);
                        } else {
                            return Err(format!("RefID is defined multiple times {}", names.to_string()));
                        }
                    },
                    SystemNames::RefIDs =>{
                        if r.ref_ids.is_none(){
                            r.ref_ids = Some(get_ref_ids(v, names)?);
                        } else {
                            return Err(format!("RefID is defined multiple times {}", names.to_string()));
                        }
                    },
                    SystemNames::Rename =>{
                        if r.rename.len() == 0{
                            r.rename = get_rename(v, names)?;
                        } else{
                            //そもそも複数回の定義はjsonパーサーによって弾かれるはずだが、いずれjsonパーサーを自作した時にこの処理が必要になるはず。
                            return Err(format!("Rename is defined multiple times {}", names.to_string()));
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}