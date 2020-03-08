use std::collections::{BTreeMap, HashMap, BTreeSet};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

#[derive(Debug)]
pub enum ValueType{
    Normal,
    Nullable,
    Incompatible,
    IncompatNullable,
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
pub struct Decim{
    val : i128,
    comma : u64,
}

impl Decim{
    pub fn new(val : i128, comma : u64) -> Decim{
        Decim{ val, comma }
    }

    pub fn val(&self) -> i128{ self.val }
    pub fn comma(&self) -> u64{ self.comma }
    pub fn to_f64(&self) -> f64{
        let val = self.val as f64;
        let comma = self.comma as f64;
        return val / 10f64.powf(comma);
    }
}

#[derive(Debug)]
pub enum Qv<T>{ Val(T), Incompatible, Null }

#[derive(Debug)]
pub struct RustArray{
    pub vec : Vec<RustArrayItem>,
    pub array_type : ArrayType,
}

#[derive(Debug)]
pub enum RustArrayItem{
    Num(f64),
    Str(String),
    NumArray(Vec<f64>),
}

#[derive(Debug)]
pub struct RefListID{
    pub id : String,
    pub is_nullable : bool,
}

impl Eq for RefListID{}

impl PartialEq for RefListID{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Hash for RefListID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
pub struct RustList{
    pub auto_id : Option<u64>,
    pub ref_list_ids : Option<BTreeSet<RefListID>>,
    pub list_id : Option<String>,
    pub default : RustObject,
    pub list : Vec<RustObject>,
}

#[derive(Debug)]
pub struct RustObject{
    //listのobjectの場合、defaultはlist側にあるのでここにはない。
    pub default : Option<BTreeMap<String, RustValue>>,
    //デフォルト値から変更されたものを記録。差分変更時に、defaultと同じになったらここから削除するかもしれない？
    pub sabun : BTreeMap<String, RustValue>,
    //listの場合idがなければならず、list内で一意である必要もある。
    //listのオブジェクトでない場合はNone
    pub id : Option<String>,
    //単一のref_idしかない時は、キーはRefIDである。
    //RefIDもRefIDsもない場合は空
    pub ref_ids : Option<BTreeMap<String, Option<String>>>,
    pub rename : HashMap<String, String>,
}

impl RustObject{
    pub fn new() -> RustObject{
        RustObject{ default : None, sabun : BTreeMap::new(),id : None, ref_ids : None,
            rename : HashMap::new() }
    }

    pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        if self.default.is_none(){
            self.default = Some(BTreeMap::new());
        }
        let def = self.default.as_mut().unwrap();
        return def.insert(key, value);
    }
}