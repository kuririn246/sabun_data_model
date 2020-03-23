use std::collections::HashMap;
use crate::rust_struct::{RustValue, RustObject, ValueType, Qv, ArrayType};
use crate::error::Result;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use indexmap::IndexMap;

///参照先が存在し、Obsoleteされてないか調べる
fn validate_ref(list_name : &str, list_items : &[RustObject], root_def : &HashMap<String, RustValue>, list_def_ref : &HashMap<String, (Qv<String>, ValueType)>) -> Result<()> {
    for item in list_items{
        let sabun = &item.refs;
        for (name, (qv, vt)) in list_def_ref{

        }
    }
    todo!()
}
