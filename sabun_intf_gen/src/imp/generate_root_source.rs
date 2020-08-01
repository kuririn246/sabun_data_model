use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::root_source::RootSource;
use crate::imp::to_member_source::{to_member_source, MemberSource};
use crate::imp::structs::param_source::ParamSource;

pub fn generate_root_source(mems : &[MemberDesc]) -> RootSource{
    let mut members : Vec<MemberSource> = vec![];
    for mem in mems{
        members.push(to_member_source(mem))
    }
    RootSource::new(members)
}