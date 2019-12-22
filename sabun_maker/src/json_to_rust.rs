use serde_json::{Value, Map };
use crate::rust_struct::{RustValue, RustObject, RustArray, ArrayType};
use crate::convert_from_json_error::ConvertFromJsonError;
use crate::json_name::{json_name, NameType, SystemNames};
use crate::json_item_to_rust::json_item_to_rust;

pub fn json_obj_to_rust(v : &Value) -> Result<RustObject,String> {
    let v = v.as_object().ok_or("v is not an object".to_string())?;
    return Ok(json_obj_to_rust2(v, &Names::new("")).unwrap());
}

pub struct Names<'a>{
    pub name : &'a str,
    pub next : Option<&'a Names<'a>>,
}

impl Names{
    pub fn to_string(&self, name : &str) -> String{
        let mut vec : Vec<String> = vec![name.to_string()];
        let mut cur = self;
        loop{
            vec.push(cur.name.to_string());
            if cur.next.is_none(){
                break;
            }
            cur = cur.next.unwrap();
        }
        vec.reverse();
        vec.join(".")
    }

    pub fn append(&self, name : &str) -> Self{
        Names{ name, next : Some(self)}
    }

    pub fn new(name : &str) -> Self{
        Names{ name, next : None }
    }
}

fn json_obj_to_rust2(v : &Map<String, Value>, names : &Names) -> Result<RustObject, String>{
    let mut r : RustObject = RustObject::new();
    for (k,v) in v{
        let name = json_name(k)?;
        match name{
            NameType::Normal =>{
                let v = json_item_to_rust(v)?;
                r.insert(k.to_string(), v);
            }

            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            r.id = Some(v.as_str().ok_or(format!("ID must be string : {}\n{}", v, names.to_string(k)))?.to_string())
                        } else{
                            return Err(format!("ID is defined multiple times {}", names.to_string(k)));
                        }
                    },
                    SystemNames::Include=>{
                        //TODO: implement "Include"
                    },
                    SystemNames::RefID =>{
                        if r.ref_id.is_none(){
                            r.ref_id = Some(v.as_str().ok_or(format!("RefID must be string : {}\n{}", v, names.to_string(k)))?.to_string());
                        } else {
                            return Err(format!("RefID is defined multiple times {}", names.to_string(k)));
                        }
                    },
                    SystemNames::RefIDs =>{
                        if r.ref_ids.is_none(){
                            r.ref_ids = Some(v.as_str().ok_or(format!("RefID must be string : {}\n{}", v, names.to_string(k)))?.to_string());
                        } else {
                            return Err(format!("RefID is defined multiple times {}", names.to_string(k)));
                        }
                    }
                }
            }
        }

    }
    Ok(r)
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