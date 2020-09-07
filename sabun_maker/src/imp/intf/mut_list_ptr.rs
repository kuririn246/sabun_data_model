use crate::imp::structs::linked_m::{LinkedMap, LinkedMapUnsafeIter};
use crate::imp::structs::rust_list::MutItem;
use std::marker::PhantomData;
use crate::imp::intf::{MutListItemPtr};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;

///&mut LinkedMapからしか使えない。
/// &LinkedMapをas *const _ as *mut _ でキャストして、書き換えないように&selfのメソッドだけ呼び出す、というようなことは出来ない。
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MutListPtr<V : From<MutListItemPtr>>{
    map : *mut LinkedMap<MutItem>,
    list_def : *const ListDefObj,
    root : *mut RootObject,
    phantom : PhantomData<*const V>,
}

impl<V : From<MutListItemPtr>> MutListPtr<V>{
    pub fn new(map : *mut LinkedMap<MutItem>, list_def : *const ListDefObj, root : *mut RootObject) -> MutListPtr<V>{ MutListPtr{ map, list_def, root, phantom : PhantomData } }

    fn from(&self, item : *mut MutItem) -> V{
        V::from(MutListItemPtr::new(item, self.list_def, self.root))
    }

    pub fn first(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.first_mut().map(|r| self.from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.first_id()
    }
    pub fn last(&mut self) -> Option<V> {
        let map = unsafe{ &mut *self.map };
        map.last_mut().map(|r| self.from(r))
    }
    pub fn last_id(&self) -> Option<u64> {
        let map = unsafe{ &mut *self.map };
        map.last_id()
    }
    pub fn from_id(&mut self, id : u64) -> Option<V>{
        let map = unsafe{ &mut *self.map };
        map.from_id_mut(id).map(|b| self.from(b))
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
        let id = map.insert_last(MutItem::new());
        self.from_id(id).unwrap()
    }
    pub fn insert_first(&mut self) -> V{
        let map = unsafe{ &mut *self.map };
        let id = map.insert_first(MutItem::new());
        self.from_id(id).unwrap()
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
        let map = unsafe { &mut *self.map };
        MutPtrIter::new(unsafe { map.iter_unsafe() }, self.list_def, self.root)
    }

    pub fn iter_from_last(&mut self) -> MutPtrIter<V> {
        let map = unsafe{ &mut *self.map };
        MutPtrIter::new(unsafe{ map.iter_from_last_unsafe() }, self.list_def, self.root)
    }

    pub fn iter_from_id(&mut self, id : u64) -> Option<MutPtrIter<V>> {
        let map = unsafe{ &mut *self.map };
        unsafe { map.iter_from_id_unsafe(id) }.map(|iter| MutPtrIter::new(iter, self.list_def, self.root))
    }
}

pub struct MutPtrIter<V : From<MutListItemPtr>>{
    iter : LinkedMapUnsafeIter<MutItem>,
    list_def : *const ListDefObj,
    root : *mut RootObject,
    phantom : PhantomData<*const V>,
}
impl<V : From<MutListItemPtr>> Iterator for MutPtrIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(MutListItemPtr::new(v, self.list_def, self.root))))
    }
}
impl<V : From<MutListItemPtr>> MutPtrIter<V>{
    pub(crate) fn new(iter : LinkedMapUnsafeIter<MutItem>, list_def : *const ListDefObj, root : *mut RootObject) -> MutPtrIter<V>{
        MutPtrIter{ iter, list_def, root, phantom : PhantomData }
    }

    fn from(&self, item : *mut MutItem) -> V{
        V::from(MutListItemPtr::new(item, self.list_def, self.root))
    }
    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(u64, V)> {
        self.iter.next_mut().map(|(k,v)| (*k, self.from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(u64, V)> {
        self.iter.prev_mut().map(|(k,v)| (*k, self.from(v)))
    }
    
    pub fn current<'a>(&mut self) -> Option<(u64, V)> {
        self.iter.current_mut().map(|(k,v)| (*k,self.from(v)))
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
