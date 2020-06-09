use crate::imp::structs::qv::Qv;

#[derive(Debug)]
#[repr(u64)]
pub enum QVal{
    Normal = 0,
    Null = 1,
    Undefined = 2,
}

pub struct ParamInf<T>{
    pub qval : QVal,
    pub val : T
}

impl<T> ParamInf<T>{
    pub fn new(qv : Qv<T>, def : T) -> Dc<T>{
        match qv{
            Qv::Val(t) => Dc{ qval : QVal::Normal, val : t },
            Qv::Null => Dc{ qval : QVal::Null, val : def },
            Qv::Undefined => Dc{ qval : QVal::Undefined, val : def }
        }
    }
}
}