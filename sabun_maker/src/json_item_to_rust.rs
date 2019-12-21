use crate::rust_struct::RustValue;
use serde_json::Value;

pub fn json_item_to_rust(v : &Value) -> Result<RustValue, String> {
    match v {
        Value::Bool(b) => {
            let (is_nullable, name) = is_nullable(k);
            if is_nullable {
                Ok(RustValue::NullableBool(Some(*b)))
            } else {
                Ok(RustValue::Bool(*b))
            }
        },
        Value::Number(num) => {
            let (is_nullable, name) = is_nullable(k);
            let num = num.as_f64()?;
            if is_nullable {
                Ok(RustValue::NullableNumber(Some(num)))
            } else {
                Ok(RustValue::Number(num))
            }
        },
        Value::String(s) => {
            let (is_nullable, name) = is_nullable(k);
            let s = s.to_string();
            if is_nullable {
                Ok(RustValue::NullableString(Some(s)))
            } else {
                Ok(RustValue::String(s))
            }
        },
        Value::Array(a) => {
            let (is_nullable, name) = is_nullable(k);

            if let Some(value) = json_array_to_rust(a, is_nullable) {
                r.insert(name, value);
            }
        },
        Value::Object(o) => {
            let (is_nullable, name) = is_nullable(k);
            let obj = json_obj_to_rust2(o)?;

            if is_nullable {
                r.insert(name, RustValue::NullableObject(Some(obj)));
            } else {
                r.insert(name, RustValue::Object(obj));
            }
        }
        _ => { panic!(); },
    }
Err("hoge".to_string())
}



fn is_nullable(s : &str) -> (bool, String){
if s.ends_with("?"){
(true, s[0..s.len()-1].to_string())
} else{
(false, s.to_string())
}
}