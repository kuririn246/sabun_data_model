use std::collections::BTreeMap;
use crate::imp::structs::my_json::Value;


#[derive(Debug)]
pub struct JsonFile{
    pub file_name_without_ext : String,
    pub json : String,
}

#[derive(Debug)]
pub struct JsonDir(pub BTreeMap<String, Value>);

impl JsonDir{
    pub fn to_string(&self) -> String{
        let mut result = String::new();
        let map = &self.0;
        for (name, value) in map{

            result.push_str(&format!("{} : {}\n",name, value.to_string_pretty()));
        }
        return result;
    }
}
