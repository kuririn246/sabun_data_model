
#[derive(Debug, PartialEq)]
pub enum Qv<T>{ Val(T), Undefined, Null }

impl<T> Qv<T>{
    pub(crate) fn qv_type(&self) -> QvType{
        match self{
            Qv::Val(_) => QvType::Val,
            Qv::Null => QvType::Null,
            Qv::Undefined => QvType::Undefined,
        }
    }

    pub fn map<U>(&self, f : impl Fn(&T) -> U) -> Qv<U> {
        match self {
            Qv::Val(v) => Qv::Val(f(v)),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        }
    }

    pub fn opt_map<U>(&self, f : impl Fn(&T) -> Option<U>) -> Option<Qv<U>>{
        match self {
            Qv::Val(v) => f(v).map(|r| Qv::Val(r)),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    }

    pub fn into_value(self) -> Option<T>{
        match self{
            Qv::Val(v) => Some(v),
            _ => None,
        }
    }
}

impl<T : Clone> Clone for Qv<T>{
    fn clone(&self) -> Self {
        match self{
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined,
            Qv::Val(v) => Qv::Val(v.clone())
        }
    }
}



pub enum QvType{
    Val, Undefined, Null
}

impl Qv<String>{
    ///null undefined "value" のどれか
    pub(crate) fn js_string(&self) -> String{
        match self{
            Qv::Val(s) => format!(r#""{}""#,s.to_string()),
            Qv::Null => "null".to_string(),
            Qv::Undefined => "undefined".to_string(),
        }
    }
}