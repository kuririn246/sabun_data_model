use sabun_maker::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use sabun_maker::structs::{RustMemberType, ParamType};
use crate::imp::structs::data_source::DataSource;
use crate::imp::structs::list_source::ListSource;
use crate::imp::structs::inner_list_source::InnerListSource;
use crate::imp::structs::inner_mut_source::InnerMutSource;
use crate::imp::structs::mut_source::MutSource;

#[derive(Debug, PartialEq)]
pub enum MemberSource{
    Param(ParamSource),
    Data(DataSource),
    List(ListSource),
    Mut(MutSource),
    InnerList(InnerListSource),
    InnerMut(InnerMutSource),
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
            MemberSource::Data(DataSource::from(mem))
        }
        RustMemberType::Template =>{
            MemberSource::List(ListSource::from(mem))
        }
        RustMemberType::InnerTemp =>{
            MemberSource::InnerList(InnerListSource::from(mem))
        }
        RustMemberType::MutList =>{
            MemberSource::Mut(MutSource::from(mem))
        }
        _ => unreachable!(),
    }
}