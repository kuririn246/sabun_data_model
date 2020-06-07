use json5_parser::{JVal, Span};
use super::names::Names;
use crate::error::Result;
use super::json_array_to_rust::GatResult;
use crate::structs::value_type::ValueType;
use crate::structs::rust_value::{RustValue, RustParam};
use crate::structs::qv::Qv;

pub(crate) fn array_null_or_undefined(a : &[JVal], gat : GatResult, value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue> {
    if a.len() != 1 {
        Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))?
    }

    let val = match a[0] {
        JVal::Null(_) => {
            match gat {
                GatResult::Num => RustValue::Param(RustParam::Number(Qv::Null), value_type),
                GatResult::Str => RustValue::Param(RustParam::String(Qv::Null), value_type),
                GatResult::Bool => RustValue::Param(RustParam::Bool(Qv::Null), value_type),
                _ => unreachable!(),
            }
        },
        JVal::Undefined(_) =>{
            match gat {
                GatResult::Num => RustValue::Param(RustParam::Number(Qv::Undefined), value_type),
                GatResult::Str => RustValue::Param(RustParam::String(Qv::Undefined), value_type),
                GatResult::Bool => RustValue::Param(RustParam::Bool(Qv::Undefined), value_type),
                _ => unreachable!(),
            }
        }
        _ =>{ Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))? }
    };
    Ok(val)
}