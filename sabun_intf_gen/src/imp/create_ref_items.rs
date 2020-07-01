use crate::imp::structs::struct_desc::RefItem;
use sabun_maker::intf::ref_desc::{ RefDesc};

pub fn create_ref_items(descs : &[RefDesc]) -> Vec<RefItem>{
    let mut r :Vec<RefItem> = Vec::with_capacity(descs.len());
    for d in descs{
        r.push(RefItem{
            is_old : d.is_old(),
            id : d.name().to_string(),
            var_type : d.var_type().clone(),
        });
    }
    r
}