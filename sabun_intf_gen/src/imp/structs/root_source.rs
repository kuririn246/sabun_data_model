use crate::imp::structs::param_source::ParamSource;
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::to_member_source::MemberSource;

pub struct RootSource{
    members : Vec<MemberSource>,
}
impl RootSource{
    pub fn new(members : Vec<MemberSource>) -> RootSource{
        RootSource{ members }
    }
    pub fn members(&self) -> &[MemberSource]{
        &self.members
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
        let item = Box::new(RootItem{ ptr : RootObjectPtr::new(root.as_ref()) });
        RootIntf{ root, item }
    }
    pub fn ptr(&self) -> RootObjectPtr{ self.item.as_ref().ptr }
");
        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    sb.push(1, &param.get("root", "self.ptr()"));
                    sb.push(1, &param.set("root", "self.ptr()"));
                }
            }
        }
        sb.push(0, "}");

        sb.to_string()
    }

    pub fn to_test_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push(0, &format!("#[cfg(test)] pub mod test{{"));
        sb.push(1, &self.to_string());
        sb.push(0, "}");
        sb.to_string()
    }
}