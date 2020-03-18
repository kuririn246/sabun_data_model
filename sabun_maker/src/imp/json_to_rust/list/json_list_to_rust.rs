use json5_parser::{JVal, Span};
use crate::rust_struct::{ValueType, RustValue, RustList, Qv,  ListType};
use super::super::names::Names;
use crate::error::Result;
use super::list_attribute::{ListAttribute, list_attribute};
use super::get_list_items::get_list_items;

pub fn json_list_to_rust(array : &[JVal], value_type : ValueType, _span : &Span, names : &Names) -> Result<RustValue> {
    let mut result = RustList::new();
    for ind in 0..array.len() {
        let item = &array[ind];
        match item {
            JVal::Array(a2, span) => {
                match list_attribute(a2, span, names)?{
                    ListAttribute::Reffered =>{
                        match result.list_type{
                            ListType::Normal =>{
                                result.list_type = ListType::Reffered;
                            },
                            _ =>{
                                Err(format!(r#"{} "Reffered" can't coexist with "AutoID" {}"#, span.line_str(), names))?
                            }
                        }
                    },
                    ListAttribute::AutoID(id) =>{
                        match result.list_type{
                            ListType::Normal =>{
                                result.list_type = ListType::AutoID(id);
                            },
                            _ =>{
                                Err(format!(r#"{} "AutoID" can't coexist with "Reffered" {}"#, span.line_str(), names))?
                            }
                        }
                    },
                    ListAttribute::Default(obj) =>{
                        result.default = obj;
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
    return Ok(RustValue::List(Qv::Val(result), value_type));
}






