use crate::imp::structs::list_type::ListType;

pub fn list_type_to_string(l : &ListType, has_item : bool) -> String{
    let s = match l{
        ListType::Data => "Data",
        ListType::List => "List",
        ListType::Mut => if has_item{ "__ViolatedList" } else{ "MutList" },
        //ListType::InnerData => "InnerData",
        ListType::InnerList => "InnerList",
        ListType::InnerMut => if has_item{ "__InnerViolatedList" } else{ "InnerMut" },
        ListType::InnderDataDef => "InnerDataDef",
        ListType::InnerListDef => "InnerListDef",
        //ListType::InnerMutDef => if has_item{ "__InnerViolatedListDef" } else{ "InnerMutDef" },
    };
    s.to_string()

}