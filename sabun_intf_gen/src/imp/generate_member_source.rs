use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use sabun_maker::structs::RustMemberType;

pub(crate) enum Re{
    Param(ParamSource)
}

pub fn generate_member_source(mem : &MemberDesc) -> Re{
    match mem.member_type(){
        RustMemberType::Bool =>{

        }
    }
}