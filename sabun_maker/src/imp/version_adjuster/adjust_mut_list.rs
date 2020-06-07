use linked_hash_map::LinkedHashMap;
use crate::error::Result;
use crate::imp::version_adjuster::adjust_mut_list_item_sabun::adjust_mut_list_item_sabun;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_mut_list_item_ref::adjust_mut_list_item_ref;
use crate::imp::structs::def_obj::ListDefObj;
use crate::imp::structs::rust_list::{MutListItem, MutList, InnerMutList};


pub fn adjust_mut(def : &ListDefObj, old_list : LinkedHashMap<u64, MutListItem>, names : &Names) -> Result<LinkedHashMap<u64, MutListItem>>{
    let mut counter : u64 = 0;
    let mut result = LinkedHashMap::with_capacity(old_list.len());
    for (_, value) in old_list{
        let (sabun, refs) = value.deconstruct();
        let new_sabun = adjust_mut_list_item_sabun(def, sabun, names)?;
        let new_refs = adjust_mut_list_item_ref(def.refs(), refs, names)?;
        result.insert(counter, MutListItem::new(counter, new_sabun, new_refs));
        counter += 1;
    }
    return Ok(result);
}

pub fn adjust_mut_list(new : MutList, old : MutList, names : &Names) -> Result<MutList>{
    let (_,old_list,_,_) = old.deconstruct();

    let new_list = adjust_mut(new.default(), old_list, names)?;
    let next_id = new_list.len() as u64;
    let(default,_,_, compatible) = new.deconstruct();
    Ok(MutList::new(default, new_list, next_id, compatible))
}

pub fn adjust_inner_mut_list(def : &ListDefObj, old : InnerMutList, names : &Names) -> Result<InnerMutList>{
    let (old_list,_) = old.deconstruct();

    let new_list = adjust_mut(def, old_list, names)?;
    let next_id = new_list.len() as u64;
    Ok(InnerMutList::new(new_list, next_id))
}