use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use sabun_maker::structs::{RustMemberType, ParamType};
use crate::imp::structs::table_source::TableSource;
use crate::imp::structs::clist_source::CListSource;
use crate::imp::structs::cil_source::CilSource;
use crate::imp::structs::mil_source::MilSource;
use crate::imp::structs::mlist_source::MListSource;

#[derive(Debug, PartialEq)]
pub enum MemberSource{
    Param(ParamSource),
    Table(TableSource),
    CList(CListSource),
    MList(MListSource),
    Cil(CilSource),
    Mil(MilSource),
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
        RustMemberType::Table =>{
            MemberSource::Table(TableSource::from(mem))
        }
        RustMemberType::Template =>{
            MemberSource::CList(CListSource::from(mem))
        }
        RustMemberType::InnerTemp =>{
            MemberSource::Cil(CilSource::from(mem))
        }
        RustMemberType::MutList =>{
            MemberSource::MList(MListSource::from(mem))
        }
        _ => unreachable!(),
    }
}