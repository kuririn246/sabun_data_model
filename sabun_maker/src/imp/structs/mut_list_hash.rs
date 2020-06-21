use crate::HashM;
use crate::imp::structs::rust_list::MutListItem;
use std::collections::hash_map::{Iter, IntoIter};

#[derive(Debug, PartialEq, Clone)]
pub struct MutListHash{
    map : HashM<u64, Box<MutListItem>>
}

impl MutListHash{
    pub(crate) fn new(map : HashM<u64, Box<MutListItem>>) -> MutListHash{
        MutListHash{ map }
    }

    pub(crate) fn get(&self, key : &u64) -> Option<&MutListItem>{ self.map.get(key).map(|b| b.as_ref()) }
    //pub(crate) fn contains_key(&self, key : u64) -> bool{ self.map.contains_key(&key) }
    pub(crate) fn iter(&self) -> MutListHashIter{ MutListHashIter{ hash_iter : self.map.iter() } }
    pub(crate) fn into_iter(self) -> IntoIter<u64, Box<MutListItem>>{ self.map.into_iter() }
    pub(crate) fn len(&self) -> usize{ self.map.len() }

    pub(crate) fn insert(&mut self, key : u64, val : Box<MutListItem>) -> Option<Box<MutListItem>>{ self.map.insert(key, val) }
}

pub struct MutListHashIter<'a>{
    hash_iter : Iter<'a, u64, Box<MutListItem>>,
}

impl<'a> Iterator for MutListHashIter<'a>{
    type Item = (&'a u64, &'a MutListItem);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, val)) = self.hash_iter.next(){
            Some((key, val.as_ref()))
        }  else{ None }
    }
}

impl<'a> IntoIterator for &'a MutListHash{
    type Item = (&'a u64, &'a MutListItem);
    type IntoIter = MutListHashIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for MutListHash{
    type Item = (u64, Box<MutListItem>);
    type IntoIter = IntoIter<u64, Box<MutListItem>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}