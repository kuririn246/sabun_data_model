use crate::{HashM, HashS};
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::ref_def_obj::RefDefObj;
use std::collections::hash_map::Iter;

#[derive(Debug, PartialEq, Clone)]
pub struct ListDefObj{
    default : Box<ListDefMap>,
    ///RustValueを巨大にしすぎないためにBoxにしてサイズを削る
    refs: Box<RefDefObj>,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashS<String>>,
}


impl ListDefObj{
    pub(crate) fn new(default : HashM<String, (usize, ListDefValue)>, refs : RefDefObj, old : HashS<String>) -> ListDefObj{
        ListDefObj{ default : Box::new(ListDefMap::new(default)), refs : Box::new(refs), old : Box::new(old) }
    }
    pub(crate) fn default(&self) -> &ListDefMap{ self.default.as_ref() }
    pub(crate) fn refs(&self) -> &RefDefObj{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }

    // pub(crate) fn compatible(&self, other : &Self) -> bool{
    //     for (k,v) in self.default(){
    //         match other.default.get(k){
    //             Some(v2) =>{
    //                 if v.compatible(v2) == false{
    //                     return false;
    //                 }
    //             },
    //             None =>{ return false; }
    //         }
    //     }
    //     return self.refs.compatible(other.refs())
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListDefMap{
    map : HashM<String, (usize, ListDefValue)>
}

impl ListDefMap{
    pub(crate) fn new(map : HashM<String, (usize, ListDefValue)>) -> ListDefMap{
        ListDefMap{ map }
    }

    pub(crate) fn get(&self, key : &str) -> Option<&ListDefValue>{ self.map.get(key).map(|(_,v)| v) }
    pub(crate) fn contains_key(&self, key : &str) -> bool{ self.map.contains_key(key) }
    pub(crate) fn iter(&self) -> ListDefMapIter{ ListDefMapIter{ hash_iter : self.map.iter() } }
    pub(crate) fn len(&self) -> usize{ self.map.len() }
}

pub struct ListDefMapIter<'a>{
    hash_iter : Iter<'a, String, (usize, ListDefValue)>,
}

impl<'a> Iterator for ListDefMapIter<'a>{
    type Item = (&'a String, &'a ListDefValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, (_, val))) = self.hash_iter.next(){
            Some((key, val))
        }  else{ None }
    }
}

impl<'a> IntoIterator for &'a ListDefMap{
    type Item = (&'a String, &'a ListDefValue);
    type IntoIter = ListDefMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

