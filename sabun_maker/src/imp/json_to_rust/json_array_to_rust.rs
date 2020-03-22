use crate::rust_struct::{RustValue, ArrayType, RustArray, Qv, ValueType };
//use super::json_list_to_rust::json_list_to_rust;
use crate::error::Result;
use super::names::Names;
use json5_parser::{JVal, Span};
use super::array_null::array_null;
use super::list::json_list_to_rust::json_list_to_rust;

pub fn json_array_to_rust(array : &Vec<JVal>, value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue>{
    use GatResult::*;
    let gat = get_array_type(array);
    return match gat{
        AT(array_type) =>{
            let array = get_array( &array[1..], &array_type, names)?;
            match array{
                Qv::Val(array) =>{
                    Ok(RustValue::Array(Qv::Val(array), array_type, value_type))
                },
                Qv::Null => {
                    if value_type.is_nullable() {
                        Ok(RustValue::Array(Qv::Null, array_type, value_type))
                    } else{
                        Err(format!(r#"{} Nullable parameters must have "?" in the end of their name {}"#, span.line_str(), names))?
                    }
                },
                Qv::Undefined => {
                    if value_type.is_nullable() {
                        Ok(RustValue::Array(Qv::Null, array_type, value_type))
                    } else{
                        Err(format!(r#"{} Nullable parameters must have "?" in the end of their name {}"#, span.line_str(), names))?
                    }
                },
            }
        },
        NoTagNum =>{
            let array = get_array(&array, &ArrayType::Num, names)?;
            match array{
                Qv::Val(array) =>{
                    Ok(RustValue::Array(Qv::Val(array), ArrayType::Num, value_type))
                },
                Qv::Null =>{ unreachable!() },
                Qv::Undefined => { unreachable!() },
            }
        }
        Num | Str | Bool =>{
            //いまのところ["Num", null] のような形での、nullのセットしか認めていない。Arrayを使った記法では、null以外はセットできない。
            array_null(&array[1..], gat, value_type, span, names)
        },
        None =>{ Err(format!(r#"{} Array must be "...-Array", "List", "Num", "Str" or "Bool" {}"#, span.line_str(), names))? },
        List =>{
            match value_type{
                ValueType::Normal =>{
                    json_list_to_rust(&array[1..], value_type, span, names)
                },
                _ =>{
                    Err(format!(r#"{} Lists can't be undefined or null {}"#, span.line_str(), names))?
                }
            }

        },
    }
}

pub enum GatResult{
    AT(ArrayType),
    List,
    Num,
    NoTagNum,
    Str,
    Bool,
    None
}

fn get_array_type(a : &Vec<JVal>) -> GatResult{
    use GatResult::*;
    if let Some(v) = a.get(0){
        return match v{
            JVal::String(s, _)=>{
                return match s.as_str(){
                    "Num-Array" =>{ AT(ArrayType::Num) },
                    "Str-Array" =>{ AT(ArrayType::String) },
                    "Num2-Array" =>{ AT(ArrayType::Num2) }
                    "Num" =>{ Num },
                    "Str" =>{ Str },
                    "Bool" =>{ Bool },
                    "List" => { GatResult::List }
                    _=>{ GatResult::None }
                }
            },
            JVal::Double(_num, _) => NoTagNum,
            _ => None,
        }
    }
    None
}

pub fn get_array(a : &[JVal], array_type : &ArrayType, names : &Names) -> Result<Qv<RustArray>>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        let val = match item{
            JVal::Double(f, _) => {
                match array_type {
                    ArrayType::Num => RustValue::Number(Qv::Val(*f), ValueType::Normal),
                    _ => Err(format!(r#"{} {} num is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::String(s, _) =>{
                match array_type {
                    ArrayType::String => RustValue::String(Qv::Val(s.to_string()), ValueType::Normal),
                    _ => Err(format!(r#"{} {} string is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::Null(_) =>{
                if vec.len() == 0 && a.len() == 1{
                    return Ok(Qv::Null);
                } else{
                    Err(format!(r#"{} null must be ["type", null] {}"#, item.line_str(), names))?
                }
            },
            JVal::Undefined(_) =>{
                if vec.len() == 0 && a.len() == 1{
                    return Ok(Qv::Undefined);
                } else{
                    Err(format!(r#"{} undefined must be ["type", undefined] {}"#, item.line_str(), names))?
                }
            },
            JVal::Array(a2, span) =>{
                match array_type{
                    ArrayType::Num2 => {
                        let rv = json_array_to_rust(a2, ValueType::Normal, span, names)?;
                        match rv{
                            RustValue::Array(array, _at, _vt) =>{
                                match array {
                                    Qv::Val(array) => {
                                        RustValue::Array(Qv::Val(array), ArrayType::Num, ValueType::Normal)
                                    },
                                    Qv::Null => {
                                        Err(format!(r#"{} null is not a num array {}"#, item.line_str(), names))?
                                    },
                                    Qv::Undefined => {
                                        Err(format!(r#"{} undefined is not a num array {}"#, item.line_str(), names))?
                                    },
                                }
                            },
                            _ =>{ Err(format!(r#"{} {} is not a num array {}"#, span.line_str(), span.slice(), names))? }
                        }
                    },
                    _ => Err(format!(r#"{} two-dimensional array must be "Num-Array2" {}"#, item.line_str(), names))?,
                }
            },
            JVal::Map(_,_) => Err(format!(r#"{} object is not valid for an array item {}"#, item.line_str(), names))?,
            JVal::Bool(_, _) => Err(format!(r#"{} bool is not valid for an array item {}"#, item.line_str(), names))?,
        };
        vec.push(val);
    }
    return Ok(Qv::Val(RustArray{ vec }));
}