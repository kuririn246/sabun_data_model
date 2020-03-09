use crate::rust_struct::{RustValue, ArrayType, RustArray, Qv, ValueType };
//use super::json_list_to_rust::json_list_to_rust;
use crate::error::Result;
use super::names::Names;
use json5_parser::{JVal, Span};

pub fn json_array_to_rust(array : &Vec<JVal>, value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue>{
    use GatResult::*;
    return match get_array_type(array){
        AT(array_type) =>{
            let array = get_array( array,array_type, names)?;
            if let Some(array) = array {
                Ok(RustValue::Array(Qv::Val(array), value_type))
            } else{
                if value_type.is_nullable() {
                    Ok(RustValue::Array(Qv::Null, value_type))
                } else{
                    Err(format!(r#"{} Nullable parameters must have "?" in the end of their name {}"#, span.line_col_str(), names))?
                }
            }
        },
        Num | Str | Bool =>{
            todo!()
        },
        None =>{ Err(format!(r#"{} Array must be "...-Array", "List", "Num", "Str" or "Bool" {}"#, span.line_col_str(), names))? },
        List =>{
            todo!()
            //return Ok(json_list_to_rust(array, is_nullable, names)?);
        },
    }
}

enum GatResult{
    AT(ArrayType),
    List,
    Num,
    Str,
    Bool,
    None
}

fn get_array_type(a : &Vec<JVal>) -> GatResult{
    use GatResult::*;
    if let Some(v) = a.get(0){
        if let Some(s) = v.as_str(){
            return match s{
                "Num-Array" =>{ AT(ArrayType::Num) },
                "Str-Array" =>{ AT(ArrayType::String) },
                "Num-Array2" =>{ AT(ArrayType::Num2) }
                "Num" =>{ Num },
                "Str" =>{ Str },
                "Bool" =>{ Bool },
                "List" => { GatResult::List }
                _=>{ return GatResult::None; }
            }
        }
    }
    None
}

fn get_array(a : &[JVal], array_type : ArrayType, names : &Names) -> Result<Option<RustArray>>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        let val = match item{
            JVal::Double(f, _) => {
                match array_type {
                    ArrayType::Num => RustValue::Number(Qv::Val(*f), ValueType::Normal),
                    _ => Err(format!(r#"{} {} num is not valid in this array {}"#, item.line_col(), item.original(), names))?,
                }
            },
            JVal::String(s, _) =>{
                match array_type {
                    ArrayType::String => RustValue::String(Qv::Val(s.to_string()), ValueType::Normal),
                    _ => Err(format!(r#"{} {} string is not valid in this array {}"#, item.line_col(), item.original(), names))?,
                }
            },
            JVal::Null(_) =>{
                if vec.len() == 0 && a.len() == 1{
                    return Ok(None);
                } else{
                    Err(format!(r#"{} null must be ["type", null] {}"#, item.line_col(), names))?
                }
            },
            JVal::Array(a2, _) =>{
                match array_type{
                    ArrayType::Num2 => {
                        let array = get_array(a2, ArrayType::Num, names)?;
                        if let Some(array) = array {
                            RustValue::Array(Qv::Val(array), ValueType::Normal)
                        } else{
                            if vec.len() == 0 && a.len() == 1{
                                return Ok(None);
                            } else{
                                Err(format!(r#"{} null must be ["type", null] {}"#, item.line_col(), names))?
                            }
                        }
                    },
                    _ => Err(format!(r#"{} two-dimensional array must be "Num-Array2" {}"#, item.line_col(), names))?,
                }
            },
            JVal::Map(_,_) => unreachable!(),
            JVal::Bool(_, _) => unreachable!(),
        };
        vec.push(val);
    }
    return Ok(Some(RustArray{ vec, array_type }));
}

// fn get_str_array(a : &[Value], names : &Names) -> Result<RustArray, String>{
//     let mut vec : Vec<RustValue> = vec![];
//     for item in a{
//         let s = item.as_str().ok_or(format!("{} is not string {}", item, names.to_string()))?;
//         vec.push(RustValue::String(Qv::Val(s.to_string())));
//     }
//     return Ok(RustArray{ vec, array_type : ArrayType::String });
// }
//
// fn get_num_array2(a : &[Value], names : &Names) -> Result<RustArray, String>{
//     let mut vec : Vec<RustValue> = vec![];
//     for item in a{
//         match item{
//             Value::Array(a) =>{
//                 let array = get_num_array(a, names)?;
//                 vec.push(RustValue::Array(Qv::Val(array)));
//             }
//             _=>{ return Err(format!("{} is not an array {}", item, names.to_string())); }
//         }
//     }
//     return Ok(RustArray{ vec, array_type : ArrayType::Num2 })
// }