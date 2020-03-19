use json5_parser::{JVal, Span};
use crate::imp::json_to_rust::names::Names;
use std::collections::BTreeMap;
use crate::imp::json_to_rust::get_renamed::get_renamed;
use crate::error::Result;

pub fn get_redef(array : &[JVal], span : &Span, names : &Names) -> Result<BTreeMap<String, String>>{
    get_renamed(array, span, names)
}