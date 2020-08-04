use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use sabun_maker::structs::{RustMemberType, ParamType};
use crate::imp::structs::data_source::DataSource;
use crate::imp::structs::list_source::ListSource;

#[derive(Debug, PartialEq)]
pub enum MemberSource{
    Param(ParamSource),
    Data(DataSource),
    List(ListSource),
}

pub fn to_member_source(mem : &MemberDesc) -> MemberSource{
    match mem.member_type(){
        RustMemberType::Bool =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Bool,
                mem.is_old(),
            ))
        },
        RustMemberType::Int =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Int,
                mem.is_old(),
            ))
        },
        RustMemberType::Float =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Float,
                mem.is_old(),
            ))
        },
        RustMemberType::Str =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::String,
                mem.is_old(),
            ))
        },
        RustMemberType::Data =>{
            MemberSource::Data(DataSource::from(mem))
        },
        RustMemberType::List =>{
            MemberSource::List(ListSource::from(mem))
        },
        _ => unreachable!(),
    }
}