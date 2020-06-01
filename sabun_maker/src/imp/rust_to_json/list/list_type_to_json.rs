// use crate::error::Result;
// use crate::structs::my_json::Value;
//
// pub fn list_type_to_json(l : &ListType, _name : &str) -> Result<Value>{
//     let mut result : Vec<Value> = vec![];
//
//     match l{
//         ListType::Normal =>{},
//         ListType::AutoID(val) =>{
//             result.push(val_str("AutoID"));
//             match val{
//                 Some(i) => result.push(Value::Number(*i as f64)),
//                 None =>{
//                     //常識的に考えるとエラーだが文法上はなくてもいい。次のIDが既に使われていた場合、その都度新しく有効なIDを探せば良い
//                 }
//             };
//         },
//         ListType::Reffered =>{
//             result.push(val_str("Reffered"));
//         }
//     }
//
//     return Ok(Value::Array(result));
// }
//
// pub fn val_str(s : &str) -> Value{
//     Value::String(s.to_string())
// }