use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

#[derive(Debug)]
pub enum RustValue{
    Bool(Qv<bool>),
    NullableBool(Qv<Option<bool>>),
    Number(Qv<f64>),
    NullableNumber(Qv<Option<f64>>),
    String(Qv<String>),
    NullableString(Qv<Option<String>>),
    Array(Qv<RustArray>),
    NullableArray(Qv<Option<RustArray>>),
    List(Qv<RustList>),
    NullableList(Qv<Option<RustList>>),
    Object(Qv<RustObject>),
    NullableObject(Qv<Option<RustObject>>)
}

#[derive(Debug)]
pub enum Qv<T>{ Val(T), Undefined }

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