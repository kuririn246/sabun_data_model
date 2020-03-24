

#[derive(Debug, Clone)]
pub enum Qv<T>{ Val(T), Undefined, Null }

impl<T> Qv<T>{
    pub fn qv_type(&self) -> QvType{
        match self{
            Qv::Val(_) => QvType::Val,
            Qv::Null => QvType::Null,
            Qv::Undefined => QvType::Undefined,
        }
    }
}

pub enum QvType{
    Val, Undefined, Null
}
