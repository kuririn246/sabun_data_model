use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::qv::Qv;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RefValue{
    value_type : ValueType,
    value : Qv<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RefSabValue{
    value : Qv<String>,
}

impl RefValue{
    pub(crate) fn new(value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ value, value_type }
    }
    pub(crate) fn value_type(&self) -> ValueType{ self.value_type }
    pub(crate) fn value(&self) -> &Qv<String>{ &self.value }
    pub(crate) fn into_sab_value(self) ->RefSabValue{ RefSabValue{ value : self.value } }

    ///右の値を自身に代入できるか
    pub(crate) fn acceptable(&self, other : &RefSabValue) -> bool{
        self.value_type.acceptable(&other.value.qv_type())
    }

    ///右が取りうる値すべてが左に代入できるか
    pub(crate) fn compatible(&self, other : &Self) -> bool{
        self.value_type.compatible(&other.value_type)
    }


}
impl RefSabValue{
    pub(crate) fn new(value : Qv<String>) -> RefSabValue{ RefSabValue{ value } }
    pub(crate) fn value(&self) -> &Qv<String>{ &self.value }
    pub(crate) fn into_ref_value_for_json(self) -> RefValue{
        //sabun側は?とか!とかなくていいのでNormalでよい
        RefValue{ value : self.value, value_type : ValueType::Normal }
    }
}