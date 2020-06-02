use crate::structs::value_type::ValueType;

pub fn name_with_suffix(name : &str, vt : ValueType) -> String{
    format!("{}{}", name, vt.to_suffix())
}