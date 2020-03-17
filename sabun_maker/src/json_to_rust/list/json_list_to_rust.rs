use json5_parser::{JVal, Span};
use crate::rust_struct::{ValueType, RustValue, RustList, Qv, RustObject, ListType};
use crate::json_to_rust::names::Names;
use crate::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::error::Result;
use crate::json_to_rust::list::list_attribute::{ListAttribute, list_attribute};

pub fn json_list_to_rust(array : &[JVal], value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue> {
    let mut result = RustList::new();
    for item in array {
        let val = match item {
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
                    ListAttribute::AutoID =>{
                        match result.list_type{
                            ListType::Normal =>{
                                result.list_type = ListType::AutoID;
                            },
                            _ =>{
                                Err(format!(r#"{} "AutoID" can't coexist with "Reffered" {}"#, span.line_col_str(), names))?
                            }
                        }
                    },
                    _ =>{ todo!() }
                }
            },
            //JVal::Map(_, _) => unreachable!(),
            //JVal::Bool(_, _) => unreachable!(),
            _ =>{ todo!() }
        };
    }
    return Ok(RustValue::List(Qv::Val(result), value_type));
}






