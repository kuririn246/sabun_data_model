use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

#[derive(Debug)]
pub enum RustValue{
    Bool(bool),
    NullableBool(Option<bool>),
    Number(f64),
    NullableNumber(Option<f64>),
    String(String),
    NullableString(Option<String>),
    Array(RustArray),
    NullableArray(Option<RustArray>),
    List(RustList),
    NullableList(Option<RustList>),
    Object(RustObject),
    NullableObject(Option<RustObject>)
}

#[derive(Debug)]
pub struct RustArray{
    pub vec : Vec<RustValue>,
    pub array_type : ArrayType,
}

#[derive(Debug)]
pub struct RustList{
    pub auto_id : Option<u64>,
}

#[derive(Debug)]
pub struct RustObject{
    pub map : BTreeMap<String, RustValue>,
    pub id : Option<String>,
    pub ref_id : Option<String>,
    pub ref_ids : Option<BTreeMap<String, String>>,
    pub rename : HashMap<String, String>,
}

impl RustObject{
    pub fn new() -> RustObject{
        RustObject{ map : BTreeMap::new(), id : None, ref_id : None, ref_ids : None, rename : HashMap::new() }
    }

    pub fn insert(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        self.map.insert(key, value)
    }
}