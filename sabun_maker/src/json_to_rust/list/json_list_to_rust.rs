use json5_parser::{JVal, Span};
use crate::rust_struct::{ValueType, RustValue, RustList, Qv, RustObject};
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
                        if result.reffered == false{
                            result.reffered = true;
                        } else{
                            Err(format!(r#"{} ["Reffered"] is defined multiple times {}"#, span.line_col_str(), names))?
                        }
                    },
                    ListAttribute::Ref(vec) =>{
                        if result.refs.len() == 0{
                            result.refs = vec;
                        } else{
                            Err(format!(r#"{} "Ref" is defined multiple times {}"#, span.line_col_str(), names))?
                        }
                    },
                    ListAttribute::AutoID =>{
                        if result.auto_id.is_none(){
                            result.auto_id = Some(0);
                        } else{
                            Err(format!(r#"{} "AutoID" is defined multiple times {}"#, span.line_col_str(), names))?
                        }
                    },
                    ListAttribute::Renamed(map) =>{

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






