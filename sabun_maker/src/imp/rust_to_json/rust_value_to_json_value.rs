use crate::imp::rust_to_json::rust_array_to_json::rust_array_to_json;
use crate::error::Result;
use crate::imp::rust_to_json::list::rust_list_to_json::rust_list_to_json;
use crate::structs::rust_value::RustValue;
use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;
use crate::structs::my_json::Value;
use crate::imp::rust_to_json::get_param::get_param;

pub fn rust_value_to_json_value(v : &RustValue) -> Result<Value>{
    match v{
        RustValue::Param(param, vt) => get_param(param),
        RustValue::List(l) =>
    }

}
