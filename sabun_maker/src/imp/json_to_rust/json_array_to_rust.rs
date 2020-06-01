//use super::json_list_to_rust::json_list_to_rust;
use crate::error::Result;
use super::names::Names;
use json5_parser::{JVal, Span};
use super::list::json_list_to_rust::json_list_to_rust;
use crate::structs::value_type::ValueType;
use crate::structs::rust_value::{RustValue, RustArray, RustParam};
use crate::structs::qv::Qv;
use crate::structs::array_type::ArrayType;
use crate::imp::json_to_rust::array_null::array_null_or_undefined;

pub fn json_array_to_rust(array : &Vec<JVal>, value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue>{
    use GatResult::*;
    let gat = get_array_type(array);
    return match gat{
        AT(array_type) =>{
            let array = get_array( &array[1..], &array_type, names)?;
            match array{
                Qv::Val(array) =>{
                    Ok(RustValue::Param(RustParam::Array(Qv::Val(array), array_type), value_type))
                },
                Qv::Null => {
                    Ok(RustValue::Param(RustParam::Array(Qv::Null, array_type), value_type))
                },
                Qv::Undefined => {
                    Ok(RustValue::Param(RustParam::Array(Qv::Undefined, array_type), value_type))
                },
            }
        },
        NoTagNum =>{
            let array = get_array(&array, &ArrayType::Num, names)?;
            match array{
                Qv::Val(array) =>{
                    Ok(RustValue::Param(RustParam::Array(Qv::Val(array), ArrayType::Num), value_type))
                },
                Qv::Null =>{ unreachable!() },
                Qv::Undefined => { unreachable!() },
            }
        }
        Num | Str | Bool =>{
            array_null_or_undefined(&array[1..], gat, value_type, span, names)
        },
        InnerMutDef =>{
            if value_type.is_undefable() {
                Ok(RustValue::Param(RustParam::Array(Qv::Undefined, array_type), value_type))
            } else{
                Err(format!(r#"{} Undefiable parameters must have "!" in the end of their name {}"#, span.line_str(), names))?
            }
        },
        None =>{ Err(format!(r#"{} Array must be "...Array", "List", "Data", "MutList", "InnerData", "InnerList", "InnerMut", "Num", "Str" or "Bool" {}"#, span.line_str(), names))? },
        List | Data | MutList | InnerList | InnerData | InnerMut | InnerListDef | InnerDataDef | InnerMutDef |
        ViolatedList | InnerViolatedList | InnerViolatedListDef =>{
            match value_type{
                ValueType::Normal =>{
                    let tmp = json_list_to_rust(&array[1..], span, names)?;
                    match gat{
                        List => Ok(RustValue::List(tmp.to_const_list()?)),
                        Data => Ok(RustValue::Data(tmp.to_const_data()?)),
                        MutList => Ok(RustValue::Mut(tmp.to_mut_list()?)),
                        InnerList => Ok(RustValue::InnerList(tmp.to_inner_list()?)),
                        InnerData => Ok(RustValue::InnerData(tmp.to_inner_data()?)),
                        InnerMut => Ok(RustValue::InnerMut(tmp.to_inner_mut_list()?)),
                        InnerListDef => Ok(RustValue::InnerListDef(tmp.to_inner_def()?)),
                        InnerDataDef => Ok(RustValue::InnerDataDef(tmp.to_inner_def()?)),
                        InnerMutDef => Ok(RustValue::InnerMutDef(tmp.to_inner_mut_def()?)),
                        ViolatedList => Ok(RustValue::Mut(tmp.to_violated_list()?)),
                        InnerViolatedList => Ok(RustValue::InnerMut(tmp.to_inner_violated_list()?)),
                        InnerViolatedListDef => Ok(RustValue::InnerMutDef(tmp.to_inner_mut_def()?)),
                        _ => unreachable!(),
                    }
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
    Data,
    MutList,
    InnerList,
    InnerData,
    InnerMut,
    InnerListDef,
    InnerDataDef,
    InnerMutDef,
    ViolatedList,
    InnerViolatedList,
    InnerViolatedListDef,
    Num,
    NoTagNum,
    Str,
    Bool,
    InnerMutUndefined,
    None
}

fn get_array_type(a : &Vec<JVal>) -> GatResult{
    use GatResult::*;
    if let Some(v) = a.get(0){
        return match v{
            JVal::String(s, _)=>{
                return match s.as_str(){
                    "NumArray" =>{ AT(ArrayType::Num) },
                    "StrArray" =>{ AT(ArrayType::String) },
                    "Num2Array" =>{ AT(ArrayType::Num2) },
                    "Num" =>{ Num },
                    "Str" =>{ Str },
                    "Bool" =>{ Bool },
                    "List" => { GatResult::List },
                    "Data" => { GatResult::Data },
                    "MutList" => { GatResult::MutList },
                    "InnerList" => { GatResult::InnerList },
                    "InnerData" => { GatResult::InnerData },
                    "InnerMut" =>{ GatResult::InnerMut },
                    "InnerListDef" => { GatResult::InnerListDef },
                    "InnerDataDef" => { GatResult::InnerDataDef },
                    "InnerMutDef" =>{ GatResult::InnerMutDef },
                    "__ViolatedList" => { GatResult::ViolatedList },
                    "__InnerViolatedList" => { GatResult::InnerViolatedList },
                    "__InnerViolatedListDef" => { GatResult::InnerViolatedListDef },
                    "__InnerMutUndefined" => { GatResult::InnerMutUndefined },
                    _=>{ GatResult::None },
                }
            },
            JVal::Double(_num, _) => NoTagNum,
            _ => None,
        }
    }
    None
}

pub fn get_array(a : &[JVal], array_type : &ArrayType, names : &Names) -> Result<Qv<RustArray>>{
    let mut vec : Vec<RustParam> = Vec::with_capacity(a.len());
    for item in a{
        let val = match item{
            JVal::Double(f, _) => {
                match array_type {
                    ArrayType::Num => RustParam::Number(Qv::Val(*f)),
                    _ => Err(format!(r#"{} {} num is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::String(s, _) =>{
                match array_type {
                    ArrayType::String => RustParam::String(Qv::Val(s.to_string())),
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
                            RustValue::Param(RustParam::Array(array, at), _vt) =>{
                                match array {
                                    Qv::Val(array) => {
                                        match at {
                                            ArrayType::Num => RustParam::Array(Qv::Val(array), ArrayType::Num),
                                            _ => { Err(format!(r#"{} {} is not a num array {}"#, span.line_str(), span.slice(), names))? }
                                        }
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
    return Ok(Qv::Val(RustArray{ vec : vec }));
}