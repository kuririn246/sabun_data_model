use crate::structs::rust_value::ListType;

pub fn list_type_to_string(l : &ListType, is_violated : bool) -> String{
    let s = match l{
        ListType::Data => "Data",
        ListType::List => "List",
        ListType::Mut => if is_violated{ "__ViolatedList" } else{ "MutList" },
        ListType::InnerData => "InnerData",
        ListType::InnerList => "InnerList",
        ListType::InnerMut => if is_violated{ "__InnerViolatedList" } else{ "InnerMut" },
        ListType::InnderDataDef => "InnerDataDef",
        ListType::InnerListDef => "InnerListDef",
        ListType::InnerMutDef => if is_violated{ "__InnerViolatedListDef" } else{ "InnerMutDef" },
    };
    s.to_string()

}