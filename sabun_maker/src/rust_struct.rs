use std::collections::{HashMap};
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

#[derive(Debug, Clone)]
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

impl RustList{
    pub fn is_auto_id(&self) -> bool{
        match self.list_type{
            ListType::AutoID(_) => true,
            _ => false,
        }
    }

    pub fn is_null_auto_id(&self) -> bool{
        match self.list_type{
            ListType::AutoID(val) =>{
                val.is_none()
            },
            _ =>{ false }
        }
    }

    pub fn set_auto_id(&mut self, id : u64) -> Result<(), ()>{
        match self.list_type{
            ListType::AutoID(_) =>{
                self.list_type = ListType::AutoID(Some(id));
                Ok(())
            },
            _=>{ Err(()) }
        }
    }
}

#[derive(Debug)]
pub enum ListType{
    AutoID(Option<u64>),
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
    pub sabun : IndexMap<String, RustValue>,
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
        RustObject{ default : Some(IndexMap::new()), sabun : IndexMap::new(),id : None, refs: None,
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