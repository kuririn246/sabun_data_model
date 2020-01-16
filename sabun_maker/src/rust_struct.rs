use std::collections::{BTreeMap, HashMap, BTreeSet};
use std::hash::{Hash, Hasher};

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
    //ref_list_idsにより、どのメンバはどのrefを参照するべきか、あるいは曖昧なので参照できないかが決定できる。
    //参照できない場合None,参照できる場合参照先のRefListIDが入る。
    pub ref_names : HashMap<String, Option<String>>,
}

#[derive(Debug)]
pub struct RustObject{
    //listの場合、defaultはlist側にあるのでここにはない。
    pub default : Option<BTreeMap<String, RustValue>>,
    //デフォルト値からの変更を記録。差分変更時に、defaultと同じになったらここから削除する
    pub sabun : BTreeMap<String, RustValue>,
    //listの場合idがなければならず、list内で一意である必要もある。
    //listのオブジェクトでない場合はNone
    pub id : Option<String>,
    //単一のref_idしかない時は、キーはRefIDである。
    //RefIDもRefIDsもない場合は空
    pub ref_ids : BTreeMap<String, String>,
    pub rename : HashMap<String, String>,
    //継承したメンバをoverrideした場合、ここにいれていく
    pub ref_override : BTreeMap<String, BTreeMap<String, RustValue>>,
}

impl RustObject{
    pub fn new() -> RustObject{
        RustObject{ default : None, sabun : BTreeMap::new(),id : None, ref_ids : None,
            rename : HashMap::new(), ref_override : BTreeMap::new(), }
    }

    pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        if self.default.is_none(){
            self.default = Some(BTreeMap::new());
        }
        self.default.unwrap().insert(key, value)
    }
}