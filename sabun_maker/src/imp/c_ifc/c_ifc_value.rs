use crate::imp::structs::root_obj::RootObject;

pub enum CifcValue{
    Root(RootObject),
    Data(ConstData)
}

pub struct RefIfc{
    parent : Box<Option<RefIfc>>,
    name : String,
}

impl RefIfc{
    pub fn new(parent : Option<RefIfc>, name : String) -> Box<RefIfc>{
        
    }
}

