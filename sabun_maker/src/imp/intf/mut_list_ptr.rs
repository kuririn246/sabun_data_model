use crate::imp::structs::linked_m::{LinkedMap, LinkedMapIter, LinkedMapUnsafeIter};
use crate::imp::structs::rust_list::MutListItem;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;
use std::path::Iter;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MutListPtr<V : From<*mut MutListItem>>{
    map : *mut LinkedMap<MutListItem>,
    phantom : PhantomData<*const V>,
}
impl<V : From<*mut MutListItem>> MutListPtr<V>{
    pub fn new(map : *mut LinkedMap<MutListItem>) -> MutListPtr<V>{ MutListPtr{ map, phantom : PhantomData } }

    pub fn first(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.first_mut().map(|r| V::from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.first_id()
    }
    pub fn last(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.last_mut().map(|r| V::from(r))
    }
    pub fn last_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.last_id()
    }
    pub fn from_id(&mut self, id : u64) -> Option<V>{
        let map = unsafe{ &mut *self.map };
        map.from_id_mut(id).map(|b| V::from(b))
    }

    pub fn next_id(&self) -> u64{
        let map = unsafe{ &mut *self.map };
        map.next_id()
    }

    pub fn contains_key(&self, key : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.contains_key(key)
    }
    pub fn len(&self) -> usize{
        let map = unsafe{ &mut *self.map };
        map.len()
    }
    pub fn is_empty(&self) -> bool {
        let map = unsafe { &mut *self.map };
        map.is_empty()
    }

    pub fn insert(&mut self) -> V{
        self.insert_last()
    }

    pub fn insert_last(&mut self) -> V{
        let map = unsafe{ &mut *self.map };
        let id = map.insert_last(MutListItem::new());
        self.from_id(id)
    }
    pub fn insert_first(&mut self) -> V{
        let map = unsafe{ &mut *self.map };
        let id = map.insert_first(MutListItem::new());
        self.from_id(id)
    }

    pub fn remove(&mut self, id : u64) -> bool {
        let map = unsafe{ &mut *self.map };
        map.remove(id)
    }
    pub fn remove_first(&mut self) -> bool{
        let map = unsafe{ &mut *self.map };
        map.remove_first()
    }

    pub fn remove_last(&mut self) -> bool{
        let map = unsafe{ &mut *self.map };
        map.remove_last()
    }

    pub fn to_first(&mut self, id : u64) -> bool {
        let map = unsafe{ &mut *self.map };
        map.to_first(id)
    }

    pub fn to_last(&mut self, id : u64) -> bool {
        let map = unsafe{ &mut *self.map };
        map.to_last(id)
    }

    pub fn to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.to_prev(next_items_id, id)
    }

    pub fn to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
        let map = unsafe{ &mut *self.map };
        map.to_next(prev_items_id, id)
    }

    pub fn iter(&mut self) -> MutPtrIter<V> {
        let map = unsafe{ &mut *self.map };
        MutPtrIter { iter: unsafe{ map.iter_unsafe() } }
    }

    pub fn iter_from_last(&mut self) -> MutPtrIter<V> {
        let map = unsafe{ &mut *self.map };
        MutPtrIter { iter: unsafe { map.iter_from_last_unsafe() } }
    }

    pub fn iter_from_id(&mut self, id : u64) -> Option<MutPtrIter<V>> {
        let map = unsafe{ &mut *self.map };
        unsafe { map.iter_from_id_unsafe(id) }.map(|iter| MutPtrIter { iter })
    }
}

pub struct MutPtrIter<V : From<*mut MutListItem>>{
    iter : LinkedMapUnsafeIter<MutListItem>,
}
impl<V : From<*mut MutListItem>> Iterator for MutPtrIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(v)))
    }
}
impl<V : From<*mut MutListItem>> MutPtrIter<V>{
    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(u64, V)> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(u64, V)> {
        self.iter.prev_mut().map(|(k,v)| (*k, V::from(v)))
    }
    
    pub fn current<'a>(&mut self) -> Option<(u64, V)> {
        self.iter.current_mut().map(|(k,v)| (*k,V::from(v))
    }


    pub fn is_available(&self) -> bool {
        self.iter.is_available()
    }

    pub fn is_first(&self) -> bool {
        self.iter.is_first()
    }

    pub fn is_last(&self) -> bool {
        self.iter.is_last()
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