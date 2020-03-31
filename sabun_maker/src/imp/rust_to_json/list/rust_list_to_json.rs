use crate::my_json::Value;
//use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;
use crate::imp::rust_to_json::list::list_type_to_json::list_type_to_json;
use crate::imp::rust_to_json::list::redef_to_json::redef_to_json;
use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::rust_to_json_new_default;
use crate::structs::rust_list::RustList;
use crate::structs::list_type::ListType;

pub fn rust_list_to_json(l : &RustList, name : &str) -> Result<Value>{
   let mut result : Vec<Value> = vec![val_str("List")];

   match l.list_type {
      ListType::Normal =>{},
      _ =>{  result.push(list_type_to_json(&l.list_type, name)?); }
   }
   if l.redef.len() != 0 {
      result.push(redef_to_json(&l.redef));
   }

   result.push(default_to_json(&l.default)?);
   for (_id, obj) in &l.list{
      result.push(rust_to_json_new_default(obj, Some(&l.default.default))?);
   }

   return Ok(Value::Array(result));
}

pub fn val_str(s : &str) -> Value{
   Value::String(s.to_string())
}