use json5_parser::JVal;
use super::names::Names;
use super::json_name::{json_name, NameType, SystemNames};
use super::json_item_to_rust::json_item_to_rust;
use crate::error::Result;

use crate::imp::json_to_rust::get_include::get_include;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::imp::json_to_rust::get_old::get_old;
use crate::imp::json_to_rust::get_id::get_id;
use linked_hash_map::LinkedHashMap;
use crate::imp::json_to_rust::get_refs::get_ref;
use json5_parser::Span;

pub fn json_obj_to_rust(v : &LinkedHashMap<String, JVal>, span : &Span, names : &Names) -> Result<TmpObj>{
    let mut r  = TmpObj::new(span.clone());
    for (k,v) in v{
        let name = json_name(k).ok_or_else(|| format!("{} {} is not a valid name {}",v.line_str(), k, names))?;
        match name{
            NameType::Name(name, vt) =>{
                let v = json_item_to_rust(&name, vt,v, names)?;

                r.insert_default(name.to_string(), v);
            },
            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            if let Some(id) = get_id(v){
                                r.id = Some(id);
                            } else {
                                Err(format!("{} ID must be a string or a num : {} {}", v.line_str(), v.slice(), names))?
                            }
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
                    SystemNames::Ref | SystemNames::Enum =>{
                        if r.refs.map.len() == 0{
                            match &v {
                                JVal::Map(map, span) =>{
                                    let mut refs = get_ref(map, span,names)?;
                                    match sn{
                                        SystemNames::Enum =>{ refs.is_enum = true; }
                                        _=>{},
                                    }
                                    r.refs = refs;
                                },
                                _ =>{ Err(format!("{} Ref must be an object {}", v.line_str(), names))?;}
                            }
                        } else {
                            Err(format!("{} (Ref|Enum) is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Old => {
                        match &v {
                            JVal::Array(a, _span) => {
                                r.old = get_old(a, names)?;
                            },
                            _ => { Err(format!("{}  {}", v.line_str(), names))?; }
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}