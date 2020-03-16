use std::collections::{BTreeMap, HashMap, BTreeSet};
use std::hash::{Hash, Hasher};
use indexmap::IndexMap;

#[derive(Debug)]
pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType{
    Normal,
    Nullable,
    Incompatible,
    IncompatNullable,
}

impl ValueType{
    pub fn is_nullable(&self) -> bool{
        match self{
            ValueType::Nullable | ValueType::IncompatNullable => true,
            _ => false,
        }
    }
}


#[derive(Debug)]
pub enum RustValue{
    Bool(Qv<bool>, ValueType),
    Number(Qv<f64>, ValueType),
    String(Qv<String>, ValueType),
    Array(Qv<RustArray>, ValueType),
    List(Qv<RustList>, ValueType),
    Object(Qv<RustObject>, ValueType),
}

#[derive(Debug)]
pub enum Qv<T>{ Val(T), Incompatible, Null }

#[derive(Debug)]
pub struct RustArray{
    pub vec : Vec<RustValue>,
    pub array_type : ArrayType,
}



#[derive(Debug)]
pub struct RustList{
    pub list_type : ListType,
    pub default : RustObject,
    pub list : Vec<RustObject>,
}

#[derive(Debug)]
pub enum ListType{
    AutoID,
    Reffered,
    Normal
}

#[derive(Debug)]
pub struct RefName{
    pub value_type : ValueType,
    pub name : String,
}

impl RustList{
    pub fn new() -> RustList{
        RustList{
            list_type : ListType::Normal,
            default : RustObject::new(),
            list : vec![],
        }
    }
}

#[derive(Debug)]
pub struct RustObject{
    //listのobjectの場合、defaultはlist側にあるのでここにはない。
    pub default : Option<IndexMap<String, RustValue>>,
    //デフォルト値から変更されたものを記録。差分変更時に、defaultと同じになったらここから削除するかもしれない？
    pub sabun : HashMap<String, RustValue>,
    //listの場合idがなければならず、list内で一意である必要もある。
    //listのオブジェクトでない場合はNone
    pub id : Option<String>,
    pub refs: Option<RefMap>,
    pub renamed: HashMap<String, String>,
    pub obsolete : bool,
}

pub type RefMap = IndexMap<String, (Qv<String>, ValueType)>;

impl RustObject{
    pub fn new() -> RustObject{
        RustObject{ default : None, sabun : IndexMap::new(),id : None, refs: None,
            renamed: HashMap::new(), obsolete : false }
    }

    pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        if self.default.is_none(){
            self.default = Some(IndexMap::new());
        }
        let def = self.default.as_mut().unwrap();
        return def.insert(key, value);
    }
}