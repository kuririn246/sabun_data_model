use crate::imp::structs::struct_desc::RefItem;
use sabun_maker::intf::ref_desc::{ RefDesc};
use crate::imp::util::to_type_name::to_snake_name;

pub fn create_ref_items(descs : &[RefDesc]) -> Vec<RefItem>{
    let mut r :Vec<RefItem> = Vec::with_capacity(descs.len());
    for d in descs{
        r.push(RefItem{
           is_old : d.is_old(),
            snake_name: to_snake_name(d.name()),
            id : d.name().to_string(),
            var_type : d.var_type().clone(),
        });
    }
    r
}