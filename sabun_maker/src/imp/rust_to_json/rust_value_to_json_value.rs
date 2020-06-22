use crate::imp::rust_to_json::get_param::get_param;
use crate::imp::rust_to_json::list::tmp_list_to_json::rust_list_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::TmpJsonList;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;
use crate::imp::rust_to_json::list::default_to_json::{ inner_def_to_json};
use crate::imp::rust_to_json::list::inner_mut_def_to_json::inner_mut_def_to_json;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::value_type::VarType;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::list_type::ListType;

pub fn rust_value_to_json_value(v : &RustValue, name : &str) -> (String, Value){
    let value = match v {
        RustValue::Param(param, vt) => {
            return (name_with_suffix(name, *vt), get_param(param))
        },
        RustValue::Data(l) => { rust_list_to_json(&TmpJsonList::from_const_data(l), ListType::Data) },
        RustValue::List(l) => { rust_list_to_json(&TmpJsonList::from_const_list(l), ListType::List) },
        RustValue::Mut(l) => { rust_list_to_json(&TmpJsonList::from_mut_list(l), ListType::Mut) },
        RustValue::InnerData(l) => { rust_list_to_json(&TmpJsonList::from_inner_data(l), ListType::InnerData) },
        RustValue::InnerList(l) => { rust_list_to_json(&TmpJsonList::from_inner_list(l), ListType::InnerList) },
        RustValue::InnerMut(l) => {
            match l {
                Some(l) => { rust_list_to_json(&TmpJsonList::from_inner_mut(l), ListType::InnerMut) },
                None => { Value::Array(vec![Value::String("__InnerMutUndefined".to_string())]) },
            }
        },
        RustValue::InnerDataDef(d) =>{ inner_def_to_json(d, ListType::InnderDataDef) },
        RustValue::InnerListDef(d) =>{ inner_def_to_json(d, ListType::InnerListDef) },
        RustValue::InnerMutDef(obj) =>{
            let val = inner_mut_def_to_json(obj);
            if obj.undefinable(){
                return (name_with_suffix(name, VarType::Undefiable), val);
            } else{
                val
            }
        }
    };
    (name.to_string(), value)
}
