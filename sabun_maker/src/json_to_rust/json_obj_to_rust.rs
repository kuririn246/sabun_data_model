use json5_parser::JVal;
use indexmap::IndexMap;
use super::names::Names;
use crate::rust_struct::RustObject;
use super::json_name::{json_name, NameType, SystemNames};
use super::json_item_to_rust::json_item_to_rust;
use crate::error::Result;
use crate::json_to_rust::get_refs::get_refs;
use crate::json_to_rust::get_renamed::get_renamed;


pub fn json_obj_to_rust(v : &IndexMap<String, JVal>, names : &Names) -> Result<RustObject>{
    let mut r : RustObject = RustObject::new();
    for (k,v) in v{
        let k : &String = k;
        let v : &JVal = v;
        let name = json_name(k).ok_or_else(|| format!("{} {} is not a valid name {}",v.line_str(), k, names))?;
        match name{
            NameType::Name(name, vt) =>{
                let v = json_item_to_rust(&name, vt,v, names)?;
                r.insert_default(k.to_string(), v);
            },
            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            r.id = Some(v.as_str().ok_or_else(|| format!("{} ID must be string : {} {}", v.line_str(), v.slice(), names))?.to_string())
                        } else{
                            Err(format!("{} ID is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Include=>{
                        //TODO: implement "Include"
                    },
                    SystemNames::Ref =>{
                        if r.refs.is_none(){
                            match &v {
                                JVal::Map(map, span) =>{
                                    r.refs = Some(get_refs(map, span,names)?);
                                },
                                _ =>{ Err(format!("{} Ref must be an object {}", v.line_str(), names))?;}

                            }
                        } else {
                            Err(format!("{} RefIDs is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Renamed =>{
                        if r.renamed.len() == 0{
                            match &v{
                                JVal::Array(a, span) =>{
                                    r.renamed = get_renamed(a, span, names)?;
                                },
                                _ =>{ Err(format!("{}  {}", v.line_str(), names))?; }
                            }

                        } else{
                            //そもそも複数回の定義はjsonパーサーによって弾かれるはずだが・・・
                            Err(format!("{} Rename is defined multiple times {}", v.line_str(), names))?;
                        }
                    }
                    SystemNames::Obsolete =>{
                        if let Some(b) = v.as_bool(){
                            if b == false{
                                Err(format!("{} Obsolete must be \"true\" {}", v.line_str(), names))?;
                            }
                            if r.obsolete == true {
                                Err(format!("{} Obsolete is defined multiple times {}", v.line_str(), names))?;
                            }
                            r.obsolete = true;
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}