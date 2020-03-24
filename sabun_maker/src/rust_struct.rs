use std::collections::{BTreeMap};
use indexmap::IndexMap;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone)]
pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

impl ArrayType{
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ArrayType::Num => 0,
            ArrayType::String => 1,
            ArrayType::Num2 => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType{
    Normal,
    Nullable,
    Undefinable,
    UndefNullable,
}

impl ValueType{
    pub fn is_nullable(&self) -> bool{
        match self{
            ValueType::Nullable | ValueType::UndefNullable => true,
            _ => false,
        }
    }

    pub fn is_undefiable(&self) -> bool{
        match self{
            ValueType::Undefinable | ValueType::UndefNullable => true,
            _ => false,
        }
    }

    pub fn to_suffix(&self) -> String{
        let s = match self{
            ValueType::Normal => "",
            ValueType::Nullable => "?",
            ValueType::Undefinable => "!",
            ValueType::UndefNullable => "!?",
        };
        s.to_string()
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            ValueType::Normal => 0,
            ValueType::Nullable => 1,
            ValueType::Undefinable => 2,
            ValueType::UndefNullable => 3,
        }
    }

    pub fn acceptable(&self, t : &QvType) -> bool {
        match self{
            ValueType::Normal => {
                match t {
                    QvType::Val => true,
                    _ => false,
                }
            },
            ValueType::Nullable => {
                match t {
                    QvType::Val | QvType::Null => true,
                    _ => false,
                }
            },
            ValueType::Undefinable => {
                match t {
                    QvType::Val | QvType::Undefined => true,
                    _ => false,
                }
            },
            ValueType::UndefNullable => true,
        }
    }
}


#[derive(Debug)]
pub enum RustValue{
    Bool(Qv<bool>, ValueType),
    Number(Qv<f64>, ValueType),
    String(Qv<String>, ValueType),
    Array(Qv<RustArray>, ArrayType, ValueType),
    ///Listは定義上nullやundefinedにならないので多分後で直す
    List(RustList),
    ///Objectも定義上nullやundefinedにならないはず。多分後で直す。
    Object(RustObject),
}

impl RustValue{
    pub fn value_type(&self) -> ValueType {
        let vt = match self{
            RustValue::Bool(_,vt) => vt,
            RustValue::Number(_, vt) => vt,
            RustValue::String(_, vt) => vt,
            RustValue::Array(_, _at, vt) => vt,
            RustValue::List(_) => &ValueType::Normal,
            RustValue::Object(_) => &ValueType::Normal,
        };
        vt.clone()
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustValue::Bool(_, _) => 0,
            RustValue::Number(_, _) => 1,
            RustValue::String(_, _) => 2,
            RustValue::Array(_, _, _) => 3,
            RustValue::List(_) => 4,
            RustValue::Object(_) => 5,
        }
    }

    pub fn qv_type(&self) -> QvType{
        match self{
            RustValue::Bool(b, _) => qv(b),
            RustValue::Number(n, _) => qv(n),
            RustValue::String(s, _) => qv(s),
            RustValue::Array(a, _, _) => qv(a),
            RustValue::List(_) => QvType::Val,
            RustValue::Object(_) => QvType::Val,
        }
    }
}

fn qv<T>(q : &Qv<T>) -> QvType{
    match q{
        Qv::Val(_) => QvType::Val,
        Qv::Null => QvType::Null,
        Qv::Undefined => QvType::Undefined,
    }
}

#[derive(Debug, Clone)]
pub enum Qv<T>{ Val(T), Undefined, Null }

pub enum QvType{
    Val, Undefined, Null
}

#[derive(Debug)]
pub struct RustArray{
    pub vec : Vec<RustValue>,

}

#[derive(Debug)]
pub struct RustList{
    pub list_type : ListType,
    pub default : RustObject,
    pub list : LinkedHashMap<String, RustObject>,
    pub redef : BTreeMap<String, String>,
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
    ///稼働中のシステムでは、次に割り振るべきIDが入っている。IDを割り振った後インクリメントしていく。
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
            list : LinkedHashMap::new(),
            redef : BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct RustObject{
    //別ファイルにあったことを記録しておくためのもの。どう使うかは後で考える。
    pub include : Vec<String>,
    //listのobjectの場合、defaultはlist側にあるのでここにはない。それ以外は絶対ある
    pub default : Option<IndexMap<String, RustValue>>,
    //デフォルト値から変更されたものを記録。差分変更時に、defaultと同じになったらここから削除するかもしれない？
    pub sabun : IndexMap<String, RustValue>,
    //listの場合idがなければならず、list内で一意である必要もある。
    //listのオブジェクトでない場合はNone
    pub id : Option<String>,
    pub refs: Option<RefMap>,
    pub renamed: BTreeMap<String, String>,
    pub obsolete : bool,
}

pub type RefMap = IndexMap<String, (Qv<String>, ValueType)>;

impl RustObject{
    pub fn new() -> RustObject{
        RustObject{
            include : vec![],
            default : Some(IndexMap::new()), sabun : IndexMap::new(),
            id : None, refs: None,
            renamed: BTreeMap::new(), obsolete : false }
    }

    pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        if self.default.is_none(){
            self.default = Some(IndexMap::new());
        }
        let def = self.default.as_mut().unwrap();
        return def.insert(key, value);
    }
}

pub struct JsonFile{
    pub file_name_without_ext : String,
    pub json : String,
}