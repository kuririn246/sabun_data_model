use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::source_builder::SourceBuilder;

pub(crate) fn generate_root(mems : &[MemberDesc]) -> String{
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

    sb.to_string()
}