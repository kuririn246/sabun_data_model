use crate::imp::structs::linked_m::{LinkedMap, LinkedMapIter, LinkedMapUnsafeIter};
use crate::imp::structs::rust_list::MutListItem;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;
use std::path::Iter;

#[derive(Debug)]
pub struct MutUnsafe<V : From<*mut MutListItem>>{
    map : *mut LinkedMap<MutListItem>,
    phantom : PhantomData<*const V>,
}
impl<V : From<*mut MutListItem>> Deref for MutUnsafe<V>{
    type Target = LinkedMap<MutListItem>;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.map }
    }
}
impl<V : From<*mut MutListItem>> DerefMut for MutUnsafe<V>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ &mut *self.map }
    }
}
impl<V : From<*mut MutListItem>> MutUnsafe<V>{
    pub fn new(map : *mut LinkedMap<MutListItem>) -> MutUnsafe<V>{ MutUnsafe{ map, phantom : PhantomData } }

    pub fn first(&mut self) -> Option<V> {
        self.map.first_mut().map(|r| V::from(r))
    }

    pub fn last(&mut self) -> Option<V> {
        self.map.last_mut().map(|r| V::from(r))
    }
    pub fn from_id(&mut self, id : u64) -> Option<V>{ self.map.from_id_mut(id).map(|b| V::from(b)) }


    pub fn iter(&mut self) -> MutUnsafeIter<V> {
        MutUnsafeIter { iter: unsafe { self.iter_unsafe() } }
    }
    // pub unsafe fn iter_from_last_unsafe(&mut self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.last) }
    // pub unsafe fn iter_from_id_unsafe(&mut self, id : u64) -> Option<LinkedMapUnsafeIter<V>>{
    //     let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return None; };
    //     Some(LinkedMapUnsafeIter::new(self, node))
    // }
}

pub struct MutUnsafeIter<V : From<*mut MutListItem>>{
    iter : LinkedMapUnsafeIter<MutListItem>,
}
impl<V : From<*mut MutListItem>> Iterator for MutUnsafeIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(v)))
    }
}


#[derive(Debug)]
pub struct MutCon<'a, V : From<&'a MutListItem>>{
    map : &'a LinkedMap<MutListItem>,
    phantom : PhantomData<*const V>,
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
struct HogeItem{
    item : *mut MutListItem,
    tekitou : i64,
    inner : *mut LinkedMap<MutListItem>
}


impl From<*mut MutListItem> for HogeItem{
    fn from(item : *mut MutListItem) -> Self {
        HogeItem{ item, tekitou : 0, inner : null_mut() }
    }
}

impl HogeItem{
    pub fn tekitou(&self) -> i64{ self.tekitou }

    pub fn inner_con(&self) -> MutCon<HugaCon>{ MutCon::new(unsafe{ &*self.inner })}
    pub fn inner_mut(&mut self) -> MutMut<HugaMut>{}

}

struct Huga{
    item : *mut MutListItem,
}
impl From<*mut MutListItem> for Huga{
    fn from(item: *mut MutListItem) -> Self {
        Huga{ item }
    }
}
struct HugaMut<'a>{
    xxx : Huga,
    phantom : PhantomData<&'a mut u64>,
}
impl<'a> From<&'a mut MutListItem> for HugaMut<'a>{
    fn from(this: &'a mut MutListItem) -> Self {
        HugaMut{ xxx : Huga::from(this as *mut _), phantom : PhantomData }
    }
}
impl<'a> DerefMut for HugaMut<'a>{


    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.xxx
    }
}

impl<'a> Deref for HugaMut<'a>{
    type Target = Huga;

    fn deref(&self) -> &Self::Target {
        &self.xxx
    }
}

struct HugaCon<'a>{
    xxx : Huga,
    phantom : PhantomData<&'a u64>,
}
impl<'a> From<&'a MutListItem> for HugaCon<'a>{
    fn from(this: &'a MutListItem) -> Self {
        HugaCon{ xxx : Huga::from(this as *const _ as *mut _), phantom : PhantomData }
    }
}

impl<'a> Deref for HugaCon<'a>{
    type Target = Huga;

    fn deref(&self) -> &Self::Target {
        &self.xxx
    }
}