use crate::imp::structs::root_obj::RootObject;

#[repr(C)]
pub struct CifcSt{
    pub cifc_type : CifcType,
    value : CifcValue,
}

impl CifcSt{
    pub(crate) fn value(&self) -> &CifcValue { &self.value }
}

#[repr(u64)]
///this is stabilized
pub enum CifcType {
    Root = 0,
}


pub enum CifcValue{
    Root(RootIfc),
}

pub struct RootIfc{
    obj : RootObject
}