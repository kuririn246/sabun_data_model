// use crate::imp::structs::linked_m::{LinkedMap, LinkedMapIterMut};
// use crate::imp::structs::rust_list::MutListItem;
// use std::marker::PhantomData;
// use std::ops::{Deref, DerefMut};
//
// #[repr(C)]
// #[derive(Debug)]
// pub struct MutListMut<'a>{
//     map : &'a mut LinkedMap<MutListItem>,
// }
//
// struct Item<'a>{
//
//     x : &'a mut MutListItem,
// }
// impl<'a> MutListMut<'a>{
//     pub fn new(map : &'a mut LinkedMap<MutListItem>)
//     -> MutListMut<'a>{ MutListMut{ map } }
//
//     // fn first(&mut self, fun : impl Fn(&mut V) -> ()){
//     //
//     //     let map = unsafe{ &mut *self.map };
//     //     map.first_mut().map(|v| fun(&mut V::from(v)));
//     // }
//
//     fn huga(&'a mut self) -> Item<'a>{ Item{ x : self.map.first_mut().unwrap() }}
//     // pub fn first_id(&self) -> Option<u64> {
//     //     self.map.first_id()
//     // }
//     // pub fn last(&'a mut self) -> Option<V> {
//     //     let getter = self.getter;
//     //     self.map.last_mut().map(|v| getter(v))
//     // }
//     // pub fn last_id(&self) -> Option<u64> {
//     //     self.map.last_id()
//     // }
//     // pub fn from_id(&'a mut self, id : u64) -> Option<V>{
//     //     let getter = self.getter;
//     //     self.map.from_id_mut(id).map(|v| getter(v))
//     // }
//     //
//     // pub fn next_id(&self) -> u64{
//     //     self.map.next_id()
//     // }
//     //
//     // pub fn contains_key(&self, key : u64) -> bool{
//     //     self.map.contains_key(key)
//     // }
//     // pub fn len(&self) -> usize{
//     //     self.map.len()
//     // }
//     // pub fn is_empty(&self) -> bool {
//     //     self.map.is_empty()
//     // }
//     //
//     // pub fn insert(&'a mut self) -> V{
//     //     self.insert_last()
//     // }
//     //
//     // pub fn insert_last(&'a mut self) -> V{
//     //     let id = self.map.insert_last(MutListItem::new());
//     //     (self.getter)(self.map.from_id_mut(id).unwrap())
//     // }
//     // pub fn insert_first(&'a mut self) -> V{
//     //     let id = self.map.insert_first(MutListItem::new());
//     //     (self.getter)(self.map.from_id_mut(id).unwrap())
//     // }
//     //
//     // pub fn remove(&mut self, id : u64) -> bool {
//     //     self.map.remove(id)
//     // }
//     // pub fn remove_first(&mut self) -> bool{
//     //     self.map.remove_first()
//     // }
//     //
//     // pub fn remove_last(&mut self) -> bool{
//     //     self.map.remove_last()
//     // }
//     //
//     // pub fn to_first(&mut self, id : u64) -> bool {
//     //     self.map.to_first(id)
//     // }
//     //
//     // pub fn to_last(&mut self, id : u64) -> bool {
//     //     self.map.to_last(id)
//     // }
//     //
//     // pub fn to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
//     //     self.map.to_prev(next_items_id, id)
//     // }
//     //
//     // pub fn to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
//     //     self.map.to_next(prev_items_id, id)
//     // }
//
//     // pub fn iter(&'a mut self) -> MutMutIter<'a, V> {
//     //     MutMutIter::new(self.map.iter_mut())
//     // }
//     //
//     // pub fn iter_from_last(&'a mut self) -> MutMutIter<'a, V> {
//     //     MutMutIter::new(self.map.iter_mut_from_last())
//     // }
//     //
//     // pub fn iter_from_id(&'a mut self, id : u64) -> Option<MutMutIter<'a, V>> {
//     //     self.map.iter_mut_from_id(id).map(|iter| MutMutIter::new(iter))
//     // }
// }
//
// pub struct MutMutIter<'a, V : From<&'a mut MutListItem>>{
//     iter : LinkedMapIterMut<'a, MutListItem>,
//     phantom : PhantomData<*const V>,
// }
// impl<'a, V : From<&'a mut MutListItem>> Iterator for MutMutIter<'a, V>{
//     type Item = (u64, V);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(|(k,v)| (*k, V::from(v)))
//     }
// }
// impl<'a, V : From<&'a mut MutListItem>> MutMutIter<'a, V>{
//     pub(crate) fn new(iter : LinkedMapIterMut<'a, MutListItem>) -> MutMutIter<'a, V>{
//         MutMutIter{ iter, phantom : PhantomData }
//     }
//
//     ///現在のカーソルにあるアイテムを返し、カーソルを進める
//     pub fn next(&mut self) -> Option<(u64, V)> {
//         self.iter.next().map(|(k,v)| (*k, V::from(v)))
//     }
//
//     ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
//     /// next2回でそれをとることも出来る。
//     ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
//     pub fn prev(&mut self) -> Option<(u64, V)> {
//         self.iter.prev().map(|(k,v)| (*k, V::from(v)))
//     }
//
//     pub fn current(&mut self) -> Option<(u64, V)> {
//         self.iter.current().map(|(k,v)| (*k,V::from(v)))
//     }
//
//     pub fn is_available(&self) -> bool {
//         self.iter.is_available()
//     }
//
//     pub fn is_first(&self) -> bool {
//         self.iter.is_first()
//     }
//
//     pub fn is_last(&self) -> bool {
//         self.iter.is_last()
//     }
// }
//
// struct Hoge{
//     ptr : *mut MutListItem,
//     a : bool,
// }
// impl Hoge{
//     pub fn get_a(&self) -> bool{
//         self.a
//     }
//
//     pub fn set_a(&mut self, b : bool){
//         self.a = b
//     }
//
//     pub fn new(ptr : *mut MutListItem) -> Hoge{ Hoge{ ptr, a : false }}
// }
// impl From<*mut MutListItem> for Hoge{
//     fn from(ptr : *mut MutListItem) -> Self {
//         Hoge{ ptr, a : false }
//     }
// }
//
//
//
// #[test]
// fn test() {
//     let mut map: LinkedMap<MutListItem> = LinkedMap::new();
//     map.insert(MutListItem::new());
//     let mut mm : MutListMut = MutListMut::new(&mut map);
//     // let b = false;
//     //
//     // mm.first(|hoge| hoge.set_a(b));
//     // mm.first(|hoge| println!("{} payopayo", hoge.get_a()));
//     //
//     // let mut item = map.first_mut().unwrap();
//     // println!("poyppoyo {:?}", item.refs());
//     {
//         let hoge = mm.huga();
//     }
//     {
//         let hoge = mm.huga();
//     }
// }