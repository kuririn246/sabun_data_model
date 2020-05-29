use json5_parser::{JVal};
use super::super::names::Names;
use crate::error::Result;
use super::list_attribute::{ListAttribute, list_attribute};
use super::get_list_items::get_list_items;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::tmp::tmp_list::TmpList;

pub fn json_list_to_rust(array : &[JVal],  names : &Names) -> Result<TmpList> {
    let mut result = TmpList::new();
    for ind in 0..array.len() {
        let item = &array[ind];
        match item {
            JVal::Array(a2, span) => {
                match list_attribute(a2, span, names)?{
                    ListAttribute::Default(obj) =>{
                        if result.default.is_none() {
                            result.default = Some(obj);
                        } else{
                            Err(format!(r#"{} {} Default is defined multiple times {}"#, span().line_str(), span().slice(), names))?
                        }
                    },
                    ListAttribute::Old(old) =>{
                        result.old = old;
                    }
                    ListAttribute::Compatible(compat) =>{

                    }
                }
            },
            JVal::Map(_, span) =>{
                let null_id = result.is_null_auto_id();
                //auto_idがnullの場合、idはいらない
                result.list = get_list_items(&array[ind..], !null_id, span, names)?;
                if null_id{
                    result.set_auto_id(result.list.len() as u64).ok();
                }
                break;
            },
        _ =>{ Err(format!(r#"{} {} List must consist of objects and arrays {}"#, item.span().line_str(), item.span().slice(), names))? }
        };
    }
    return Ok(RustValue::List(result));
}






