use crate::imp::structs::value_type::VarType;

pub fn name_with_suffix(name : &str, vt : VarType) -> String{
    format!("{}{}", name, vt.to_suffix())
}