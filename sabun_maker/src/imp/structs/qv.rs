use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Qv<T :Clone>{ Val(T), Undefined, Null }

impl<T : Clone> Qv<T>{
    pub(crate) fn qv_type(&self) -> QvType{
        match self{
            Qv::Val(_) => QvType::Val,
            Qv::Null => QvType::Null,
            Qv::Undefined => QvType::Undefined,
        }
    }

    pub(crate) fn map<U : Clone>(&self, f : impl Fn(&T) -> U) -> Qv<U> {
        match self {
            Qv::Val(v) => Qv::Val(f(v)),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        }
    }

    pub(crate) fn convert<'a, U : From<&'a T> + Clone>(&'a self) -> Qv<U>{
        match self {
            Qv::Val(v) => Qv::Val(U::from(v)) ,
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        }
    }

    pub(crate) fn try_convert<'a, U : TryFrom<&'a T, Error=TError> + Clone, TError>(&'a self) -> Result<Qv<U>, TError>{
        Ok(match self {
            Qv::Val(v) => Qv::Val(U::try_from(v)?),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        })
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