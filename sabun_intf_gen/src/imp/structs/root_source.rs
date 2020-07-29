use crate::imp::structs::param_source::ParamSource;
use crate::imp::structs::col_source::ColSource;
use crate::imp::structs::source_builder::SourceBuilder;

pub struct RootSource{
    params : Vec<ParamSource>,
    cols : Vec<ColSource>
}
impl RootSource{
    pub fn new(params : Vec<ParamSource>, cols : Vec<ColSource>) -> RootSource{
        RootSource{ params, cols }
    }
    pub fn params(&self) -> &[ParamSource]{
        &self.params
    }
    pub fn cols(&self) -> &[ColSource]{
        &self.cols
    }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        sb.push(0,     "\
use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    root : Box<RootObject>,
    item : Box<RootItem>,
}
pub struct RootItem{
    ptr : RootObjectPtr
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
        let root = Box::new(obj);
        let item = Box::new(RootObjectPtr::new(root.as_ref()));
        RootIntf{ root, item }
    }
");
        for param in self.params() {
            sb.push(1, param.get().unwrap());
            sb.push(1, param.set().unwrap());
        }
        sb.push(0, "}");

        sb.to_string()
    }
}