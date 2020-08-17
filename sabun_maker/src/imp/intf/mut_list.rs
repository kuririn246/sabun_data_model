use crate::imp::structs::linked_m::LinkedMap;
use crate::imp::structs::rust_list::MutListItem;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug)]
pub struct MutCon<'a, V : From<&'a MutListItem>>{
    map : &'a LinkedMap<MutListItem>,
    phantom : PhantomData<*mut V>,
}
impl<'a, V : From<&'a MutListItem>> Deref for MutCon<'a, V>{
    type Target = LinkedMap<MutListItem>;

    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl<'a, V : From<&'a MutListItem>> MutCon<'a, V>{
    pub fn new(map : &'a LinkedMap<MutListItem>) -> MutCon<V>{
        MutCon{ map, phantom : PhantomData }
    }

    pub fn last(&self) -> Option<V>{
        self.map.last().map(|item| V::from(item))
    }
}

#[derive(Debug)]
struct HogeItem<'a>{
    item : &'a MutListItem
}

impl<'a> From<&'a MutListItem> for HogeItem<'a>{
    fn from(item : &'a MutListItem) -> Self {
        HogeItem{ item }
    }
}

