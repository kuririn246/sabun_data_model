use serde_json::{Value, Map };
use crate::rust_struct::{RustValue, RustObject, RustArray, ArrayType};
use crate::convert_from_json_error::ConvertFromJsonError;

pub fn json_obj_to_rust(v : &Value) -> Result<RustObject,String> {
    let v = v.as_object().ok_or("v is not an object".to_string())?;
    return Ok(json_obj_to_rust2(v).unwrap());
}

fn json_obj_to_rust2(v : &Map<String, Value>) -> Option<RustObject>{ //}, String>{
    let mut r : RustObject = RustObject::new();
    for (k,v) in v{

        match v{
            Value::Bool(b) =>{
                let (is_nullable,name) = is_nullable(k);
                if is_nullable {
                    r.insert(name, RustValue::NullableBool(Some(*b)));
                } else{
                    r.insert(name, RustValue::Bool(*b));
                }
            },
            Value::Number(num)=>{
                let (is_nullable,name) = is_nullable(k);
                let num = num.as_f64()?;
                if is_nullable {
                    r.insert(name, RustValue::NullableNumber(Some(num)));
                } else{
                    r.insert(name, RustValue::Number(num));
                }
            },
            Value::String(s) =>{
                let (is_nullable,name) = is_nullable(k);
                let s = s.to_string();
                if is_nullable {
                    r.insert(name, RustValue::NullableString(Some(s)));
                } else{
                    r.insert(name, RustValue::String(s));
                }
            },
            Value::Array(a) => {
                let (is_nullable, name) = is_nullable(k);

                if let Some(value) = json_array_to_rust(a, is_nullable) {
                    r.insert(name, value);
                }
            },
            Value::Object(o) =>{
                let (is_nullable, name) = is_nullable(k);
                let obj = json_obj_to_rust2(o)?;

                if is_nullable{
                    r.insert(name, RustValue::NullableObject(Some(obj)));
                } else{
                    r.insert(name, RustValue::Object(obj));
                }
            }
            _ =>{panic!(); },
        }
        //println!("key {} value {}", k, v);
    }
    Some(r)
}



fn is_nullable(s : &str) -> (bool, String){
    if s.ends_with("?"){
        (true, s[0..s.len()-1].to_string())
    } else{
        (false, s.to_string())
    }
}

fn json_array_to_rust(array : &Vec<Value>, is_nullable : bool) -> Option<RustValue>{
    enum GatResult{
        AT(ArrayType),
        List,
        None
    }
    fn get_array_type(a : &Vec<Value>) -> GatResult{
        if let Some(v) = a.get(0){
            if let Some(s) = v.as_str(){
                match s{
                    "Num-Array" =>{ return GatResult::AT(ArrayType::Num); },
                    "Str-Array" =>{ return GatResult::AT(ArrayType::String); },
                    "Num-Array2" =>{ return GatResult::AT(ArrayType::Num2); }
                    "List" => { return GatResult::List; }
                    _=>{ return GatResult::None; }
                }
            }
        }
        GatResult::None
    }

    fn get_array(t : ArrayType, a : &Vec<Value>, is_nullable : bool) -> Option<RustValue>{
        let a = &a[1..];
        match t{
            ArrayType::Num =>{
                let array = get_num_array(a)?;
                if is_nullable{
                    return Some(RustValue::NullableArray(Some(array)));
                } else{
                    return Some(RustValue::Array(array));
                }
            },
            ArrayType::String =>{
                let array = get_str_array(a)?;
                if is_nullable{
                    return Some(RustValue::NullableArray(Some(array)));
                } else{
                    return Some(RustValue::Array(array));
                }
            },
            ArrayType::Num2 =>{
                let array = get_num_array2(a)?;
                if is_nullable{
                    return Some(RustValue::NullableArray(Some(array)));
                } else{
                    return Some(RustValue::Array(array));
                }
            },
        }
    }

    fn get_num_array(a : &[Value]) -> Option<RustArray>{
        let mut vec : Vec<RustValue> = vec![];
        for item in a{
            vec.push(RustValue::Number(item.as_f64()?));
        }
        return Some(RustArray{ vec, array_type : ArrayType::Num });
    }

    fn get_str_array(a : &[Value]) -> Option<RustArray>{
        let mut vec : Vec<RustValue> = vec![];
        for item in a{
            vec.push(RustValue::String(item.as_str()?.to_string()));
        }
        return Some(RustArray{ vec, array_type : ArrayType::String });
    }

    fn get_num_array2(a : &[Value]) -> Option<RustArray>{
        let mut vec : Vec<RustValue> = vec![];
        for item in a{
            match item{
                Value::Array(a) =>{
                    let array = get_num_array(a)?;
                    vec.push(RustValue::Array(array));
                }
                _=>{ return None; }
            }
        }
        return Some(RustArray{ vec, array_type : ArrayType::Num2 })
    }

    match get_array_type(array){
        GatResult::AT(array_type) =>{ return Some(get_array(array_type, array, is_nullable)?); }
        GatResult::None =>{ return None; },
        GatResult::List =>{ return None; },
    }

}