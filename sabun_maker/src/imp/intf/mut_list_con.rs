use crate::imp::structs::linked_m::{LinkedMap, LinkedMapIter};
use crate::imp::structs::rust_list::MutListItem;
use std::marker::PhantomData;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MutListCon<'a, V : From<&'a MutListItem>>{
    map : &'a LinkedMap<MutListItem>,
    phantom : PhantomData<*const V>,
}
impl<'a, V : From<&'a MutListItem>> MutListCon<'a, V>{
    pub fn new(map : &'a LinkedMap<MutListItem>) -> MutListCon<V>{ MutListCon{ map, phantom : PhantomData } }

    pub fn first(&self) -> Option<V> {
        self.map.first().map(|v| V::from(v))
    }
    pub fn first_id(&self) -> Option<u64> {
        self.map.first_id()
    }
    pub fn last(&self) -> Option<V> {
        self.map.last().map(|v| V::from(v))
    }
    pub fn last_id(&self) -> Option<u64> {
        self.map.last_id()
    }
    pub fn from_id(&self, id : u64) -> Option<V>{
        self.map.from_id(id).map(|v| V::from(v))
    }
    pub fn next_id(&self) -> u64{
        self.map.next_id()
    }
    pub fn contains_key(&self, key : u64) -> bool{
        self.map.contains_key(key)
    }
    pub fn len(&self) -> usize{
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn iter(&self) -> MutConIter<'a, V> {
        MutConIter::new(self.map.iter())
    }
    pub fn iter_from_last(&self) -> MutConIter<'a, V> {
        MutConIter::new(self.map.iter_from_last())
    }
    pub fn iter_from_id(&self, id : u64) -> Option<MutConIter<'a, V>> {
        self.map.iter_from_id(id).map(|iter| MutConIter::new(iter))
    }
}

pub struct MutConIter<'a, V : From<&'a MutListItem>>{
    iter : LinkedMapIter<'a, MutListItem>,
    phantom : PhantomData<&'a V>,
}
impl<'a, V : From<&'a MutListItem>> Iterator for MutConIter<'a, V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k,v)| (*k, V::from(v)))
    }
}
impl<'a, V : From<&'a MutListItem>> MutConIter<'a, V>{
    pub(crate) fn new(iter : LinkedMapIter<'a, MutListItem>) -> MutConIter<'a, V>{
        MutConIter{ iter, phantom : PhantomData }
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(u64, V)> {
        self.iter.next().map(|(k,v)| (*k, V::from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(u64, V)> {
        self.iter.prev().map(|(k,v)| (*k, V::from(v)))
    }
    
    pub fn current(&mut self) -> Option<(u64, V)> {
        self.iter.current().map(|(k,v)| (*k,V::from(v)))
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

//
// struct Hoge{
//     ptr : *mut MutListItem,
// }
// impl Hoge{
//     pub fn get_a(&self) -> bool{
//         true
//         //super::mut_list_item::get_bool(self.ptr, "a").unwrap().into_value().unwrap()
//     }
//
//     pub fn set_a(&mut self, b : bool){
//         //super::mut_list_item::set_bool(self.ptr, "a", Qv::Val(b));
//     }
// }
//
// struct HogeCon<'a>{
//     hoge : Hoge,
//     phantom : PhantomData<&'a u64>,
// }
// impl<'a> From<&'a MutListItem> for HogeCon<'a>{
//     fn from(item : &'a MutListItem) -> Self {
//         HogeCon{ hoge : Hoge{ ptr : item as *const _ as *mut _ }, phantom : PhantomData }
//     }
// }
// impl<'a> Deref for HogeCon<'a>{
//     type Target = Hoge;
//
//     fn deref(&self) -> &Self::Target {
//         &self.hoge
//     }
// }
//
// #[test]
// fn test(){
//
//
//     let mut map : LinkedMap<MutListItem> = LinkedMap::new();
//     map.insert(MutListItem::new());
//     let con : MutListCon<HogeCon> = MutListCon::new(&map);
//     let item = con.first().unwrap();
//     println!("poyppoyo {}", item.get_a());
// }