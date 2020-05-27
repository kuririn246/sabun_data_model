use json5_parser::JVal;
use crate::indexmap::IndexMap;
use super::names::Names;
use super::json_name::{json_name, NameType, SystemNames};
use super::json_item_to_rust::json_item_to_rust;
use crate::error::Result;
use super::get_refs::get_refs;
use super::get_renamed::get_renamed;
use super::json_item_to_rust::json_item_to_rust_ref;

use crate::imp::json_to_rust::get_include::get_include;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::imp::json_to_rust::get_old::get_old;


pub fn json_obj_to_rust(v : &IndexMap<String, JVal>, is_ref_obj : bool, names : &Names) -> Result<TmpObj>{
    let mut r  = TmpObj::new();
    for (k,v) in v{
        let name = json_name(k).ok_or_else(|| format!("{} {} is not a valid name {}",v.line_str(), k, names))?;
        match name{
            NameType::Name(name, vt) =>{
                let v = if is_ref_obj {
                    json_item_to_rust_ref(&name, vt,v, names)?
                }
                else{
                    json_item_to_rust(&name, vt,v, names)?
                };

                r.insert_default(name.to_string(), v);
            },
            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            match v{
                                JVal::String(s,span) =>{

                                }
                            }
                            r.id = Some(v.as_str().ok_or_else(|| format!("{} ID must be string : {} {}", v.line_str(), v.slice(), names))?.to_string())
                        } else{
                            Err(format!("{} ID is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Include=>{
                        if r.include.len() == 0{
                            r.include.append(&mut get_include(v, &names.append("Include"))?)
                        } else{
                            Err(format!("{} Include is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Ref =>{
                        if r.refs.len() == 0{
                            match &v {
                                JVal::Map(map, span) =>{
                                    r.refs = get_refs(map, span,names)?;
                                },
                                _ =>{ Err(format!("{} Ref must be an object {}", v.line_str(), names))?;}

                            }
                        } else {
                            Err(format!("{} RefIDs is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Old =>{
                        if r.old.len() == 0{
                            match &v{
                                JVal::Array(a, span) =>{
                                    r.old = get_old(a, span, names)?;
                                },
                                _ =>{ Err(format!("{}  {}", v.line_str(), names))?; }
                            }

                        } else{
                            //そもそも複数回の定義はjsonパーサーによって弾かれるはずだが・・・
                            Err(format!("{} Old is defined multiple times {}", v.line_str(), names))?;
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}