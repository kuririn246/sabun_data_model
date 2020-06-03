use crate::structs::root_object::{RootObject};
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::structs::rust_value::RustValue;

pub fn validate_root(root : &RootObject) -> Result<()>{

    for (name, val) in root.default(){
        match val {
            RustValue::Param(_, _) => {},
            RustValue::Data(data) =>{},
            RustValue::List(list) =>{},
            RustValue::Mut(m) =>{},
            RustValue::InnerData(_) => { Err(format!("{} : InnerData must not be defined in the root object", name))? },
            RustValue::InnerList(_) => { Err(format!("{} : InnerList must not be defined in the root object", name))? },
            RustValue::InnerMut(_) => { Err(format!("{} : InnerMut must not be defined in the root object", name))? },
            RustValue::InnerDataDef(_) => { Err(format!("{} : InnerDataDef must not be defined in the root object", name))? },
            RustValue::InnerListDef(_) => { Err(format!("{} : InnerListDef must not be defined in the root object", name))? },
            RustValue::InnerMutDef(_) => { Err(format!("{} : InnerMutDef must not be defined in the root object", name))? },
        }
    }

    return Ok(());
}