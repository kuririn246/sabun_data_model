use crate::error::Result;
use crate::structs::my_json::Value;
use crate::structs::rust_value::{ListType};
use crate::imp::rust_to_json::list::tmp_json_list::TmpJsonList;
use crate::imp::rust_to_json::list::list_type_to_string::list_type_to_string;
use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::imp::rust_to_json::old_to_json::{old_to_json_long, string_set_to_json};
use crate::imp::rust_to_json::list::tmp_obj_to_json::tmp_obj_to_json;

pub fn rust_list_to_json(l : &TmpJsonList, list_type : ListType) -> Value{
   let mut result : Vec<Value> = vec![];

   result.push(val_s(list_type_to_string(&list_type, l.vec.len() != 0)));
   if let Some(default) = &l.default{
      result.push(Value::Array(vec![default_to_json(default)]))
   }
   if let Some(compatible) = &l.compatible{
      result.push(string_set_to_json("Compatible", compatible));
   }
   if let Some(old) = &l.old{
      result.push(old_to_json_long(old));
   }
   if let Some(next_id) = l.next_id {
      result.push(Value::Array(vec![
         val_str("NextID"),
         Value::Number(next_id as f64)
      ]));
   }

   for item in &l.vec{
      result.push(tmp_obj_to_json(item))
   }

   return Value::Array(result);
}

pub fn val_str(s : &str) -> Value{
   Value::String(s.to_string())
}
pub fn val_s(s : String) -> Value{
   Value::String(s)
}