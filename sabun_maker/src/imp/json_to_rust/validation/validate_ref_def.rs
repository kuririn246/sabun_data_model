use crate::structs::root_object::RefDefObj;
use crate::imp::json_to_rust::names::Names;
use crate::error::Result;
use crate::structs::qv::{Qv};

///Enumを調べてるだけやな・・・
pub fn validate_ref_def(def : &RefDefObj, names : &Names) -> Result<()> {
    if def.is_enum() {
        for (_, v) in def.refs() {
            match v.value() {
                Qv::Null => {},
                _ => Err(format!("{} all default members of Enum must be null", names))?,
            }
        }
    }
    Ok(())
}