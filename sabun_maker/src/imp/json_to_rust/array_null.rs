use json5_parser::{JVal, Span};
use super::names::Names;
use crate::rust_struct::{RustValue, Qv, ValueType};
use crate::error::Result;
use super::json_array_to_rust::GatResult;

pub fn array_null(a : &[JVal], gat : GatResult, value_type : ValueType, span : &Span, names : &Names) -> Result<RustValue> {
    if a.len() != 1 {
        Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))?
    }

    let val = match a[0] {
        JVal::Null(_) => {
            if value_type.is_nullable() == false{
                Err(format!(r#"{} {} the parameter is not nullable {}"#, span.line_str(), span.slice(), names))?
            }
            match gat {
                GatResult::Num => RustValue::Number(Qv::Null, value_type),
                GatResult::Str => RustValue::String(Qv::Null, value_type),
                GatResult::Bool => RustValue::Bool(Qv::Null, value_type),
                _ => unreachable!(),
            }
        },
        _ =>{ Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))? }
    };
    Ok(val)
}