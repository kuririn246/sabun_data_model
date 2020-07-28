use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::source_builder::SourceBuilder;

pub(crate) fn generate_root(mems : &[MemberDesc]) -> String{
    let mut sb = SourceBuilder::new();

    sb.push(0,     "\
use sabun_maker::intf::*;
use sabun_maker::structs::*;

pub struct RootIntf{
    item : Box<RootItem>,
}
pub struct RootItem{
    obj : RootObject
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
        RootIntf{ item : Box::new(RootItem{ obj }) }
    }
".to_string());

    sb.to_string()
}