use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use sabun_maker::structs::{RustMemberType, ParamType};
use crate::imp::structs::data_source::DataSource;

#[derive(Debug, PartialEq)]
pub enum MemberSource{
    Param(ParamSource),
    Data(DataSource)
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
        }
        _ => unreachable!(),
    }
}