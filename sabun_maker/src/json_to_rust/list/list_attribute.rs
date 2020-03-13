use crate::rust_struct::{RustObject, ValueType, RefName};
use std::collections::BTreeMap;

pub enum ListAttribute{
    Default(RustObject),
    AutoID,
    Ref(Vec<RefName>),
    Renamed(BTreeMap<String, String>),
    Reffered,
}

