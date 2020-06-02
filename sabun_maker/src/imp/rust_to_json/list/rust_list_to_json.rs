use crate::error::Result;
use crate::imp::rust_to_json::list::redef_to_json::redef_to_json;
use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::rust_to_json_new_default;
use crate::structs::my_json::Value;
use crate::imp::json_to_rust::tmp::tmp_list::TmpList;
use crate::structs::rust_value::{RustValue, ListType};

pub fn rust_list_to_json(l : &RustValue, list_type : ListType) -> Result<Value>{
   let mut result : Vec<Value> = vec![val_str("List")];

   // match l.list_type {
   //    ListType::Normal =>{},
   //    _ =>{  result.push(list_type_to_json(&l.list_type, name)?); }
   // }
   //
   // match &l.default {
   //    ListDef::Def(def) =>{
   //       result.push(default_to_json(def, root)?);
   //       for (_id, obj) in &l.list{
   //          result.push(rust_to_json_new_default(obj, Some(&def.default), root)?);
   //       }
   //    },
   //    ListDef::Rent(s) => {
   //       result.push(Value::Array(vec![val_str("Rent"), val_str(s)]));
   //       if let Some(ListDef::Def(def)) = root.get_list(s).map(|list| &list.default){
   //          for (_id, obj) in &l.list{
   //             result.push(rust_to_json_new_default(obj, Some(&def.default), root)?);
   //          }
   //       } else{
   //          //ありえないはず
   //       }
   //    },
   //    ListDef::InnerList =>{
   //       let def = match &l.default {
   //          ListDef::Def(def) => Some(def),
   //          ListDef::Rent(s) => root.get_list(s).and_then(|l| match &l.default{
   //             ListDef::Def(def) => Some(def),
   //             _ => None, //rentの先のlistがrentであることはない・・・よね？
   //          }),
   //          ListDef::InnerList => None,
   //       };
   //       if let Some(def) = def {
   //          for (_id, obj) in &l.list {
   //             result.push(rust_to_json_new_default(obj, Some(&def.default), root)?);
   //          }
   //       }
   //
   //    },
   // }


   return Ok(Value::Array(result));
}

pub fn val_str(s : &str) -> Value{
   Value::String(s.to_string())
}