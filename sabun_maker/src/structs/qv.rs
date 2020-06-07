

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Qv<T>{ Val(T), Undefined, Null }

impl<T> Qv<T>{
    pub(crate) fn qv_type(&self) -> QvType{
        match self{
            Qv::Val(_) => QvType::Val,
            Qv::Null => QvType::Null,
            Qv::Undefined => QvType::Undefined,
        }
    }

    pub(crate) fn map<U>(&self, f : impl Fn(&T) -> U) -> Qv<U> {
        match self {
            Qv::Val(v) => Qv::Val(f(v)),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        }
    }
}

pub(crate) enum QvType{
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