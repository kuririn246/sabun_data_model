use crate::structs::root_object::{RootObject};
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_data::validate_data;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list::validate_list;
use crate::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;

pub fn validate_root(root : &RootObject) -> Result<()>{

    for (name, val) in root.default(){
        let names = &Names::new(name);
        match val {
            RustValue::Param(_, _) => {
                //rootのparamについてはすること何もないよね・・・？
            },
            RustValue::Data(data) =>{
                validate_data(data.default(), data.list(), data.old(), root, names)?
            },
            RustValue::List(list) =>{
                validate_list(list.default(), list.list(), root, names)?
            },
            RustValue::Mut(m) =>{
                validate_mut_list(m.default(), m.list(), m.compatible(), root, names)?
            },
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