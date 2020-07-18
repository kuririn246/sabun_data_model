use crate::imp::structs::sources::StructSource;
use crate::imp::structs::str_and_tab::{str_and_tabs_to_string, StrAndTab};
use crate::imp::structs::struct_desc::{StructDesc, ColType};
use crate::imp::util::to_type_name::to_snake_name;
use crate::imp::col_source::const_data::to_const_data_source;
use crate::imp::col_source::const_list::to_const_list_source;

pub fn to_source_from_col_temp(imp : &StructDesc) -> StructSource {
    match imp.col_type{
        ColType::Data => to_const_data_source(imp),
        ColType::List => to_const_list_source(imp),
        _ =>{ unreachable!() },
    }
}