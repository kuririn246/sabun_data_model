use json5_parser::{JVal, Span};
use crate::rust_struct::{ValueType, RustValue, RustList, Qv,  ListType};
use crate::json_to_rust::names::Names;
use crate::error::Result;
use crate::json_to_rust::list::list_attribute::{ListAttribute, list_attribute};
use crate::json_to_rust::list::get_list_items::get_list_items;

pub fn json_list_to_rust(array : &[JVal], value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue> {
    let mut result = RustList::new();
    let mut auto_id_null = false;
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
                                Err(format!(r#"{} "Reffered" can't coexist with "AutoID" {}"#, span.line_col_str(), names))?
                            }
                        }
                    },
                    ListAttribute::AutoID(id) =>{
                        match result.list_type{
                            ListType::Normal =>{
                                result.list_type = ListType::AutoID(id.unwrap_or(0));
                                if id.is_none(){
                                    auto_id_null = true;
                                }
                            },
                            _ =>{
                                Err(format!(r#"{} "AutoID" can't coexist with "Reffered" {}"#, span.line_col_str(), names))?
                            }
                        }
                    },
                    ListAttribute::Default(obj) =>{
                        result.default = obj;
                    }
                }
            },
            JVal::Map(_, span) =>{
                result.list = get_list_items(&array[ind..], result.list_type.is_auto_id(), span, names)?;
                if auto_id_null{
                    result.list_type = ListType::AutoID(result.list.len() as u64);
                }
                break;
            },
        _ =>{ Err(format!(r#"{} {} List must consist of objects and arrays {}"#, item.span().line_col_str(), item.span().slice(), names))? }
        };
    }
    return Ok(RustValue::List(Qv::Val(result), value_type));
}






