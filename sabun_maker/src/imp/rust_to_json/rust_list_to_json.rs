use crate::rust_struct::{Qv, RustValue, ArrayType, ValueType, RustList};
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;

pub fn rust_list_to_json(l : &RustList) -> Result<Value>{
    let mut result : Vec<Value> = vec![];

    l.list_type


}