use std::collections::{HashSet, HashMap};
use crate::imp::structs::ref_value::RefValue;
use std::collections::hash_map::Iter;


#[derive(Debug, PartialEq, Clone)]
pub struct RefDefObj {
    refs: Box<RefDefMap>,
    /// Enum とRefの二通りの定義の仕方があり、Enumの場合は Ref のうち一つだけ値があり、ほかは全部nullにしなきゃいけない。
    /// プログラムからはmatch でアクセス出来る。値があるRefをキャストしてゲットする。
    is_enum : bool,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashSet<String>>,
}

impl RefDefObj{
    pub(crate) fn new(refs : HashMap<String, (usize, RefValue)>, is_enum : bool, old : HashSet<String>) -> RefDefObj{
        RefDefObj{ refs : Box::new(RefDefMap::new(refs)), is_enum, old : Box::new(old) }
    }
    pub(crate) fn refs(&self) -> &RefDefMap{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashSet<String>{ self.old.as_ref() }
    pub(crate) fn is_enum(&self) -> bool{ self.is_enum }

    pub(crate) fn compatible(&self, other : &Self) -> bool{
        for (k,v) in self.refs(){
            match other.refs.get(k){
                Some(v2) =>{
                    if v.compatible(v2) == false{
                        return false;
                    }
                },
                None =>{ return false; }
            }
        }
        return true;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDefMap{
    map : HashMap<String, (usize, RefValue)>
}

impl RefDefMap {
    pub fn new(map: HashMap<String, (usize, RefValue)>) -> RefDefMap {
        RefDefMap { map }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&RefValue> { self.map.get(key).map(|(_, v)| v) }
    pub(crate) fn contains_key(&self, key: &str) -> bool { self.map.contains_key(key) }
    pub(crate) fn iter(&self) -> RefDefMapIter { RefDefMapIter { hash_iter: self.map.iter() } }
    pub(crate) fn len(&self) -> usize { self.map.len() }
}

pub struct RefDefMapIter<'a> {
    hash_iter: Iter<'a, String, (usize, RefValue)>,
}

impl<'a> Iterator for RefDefMapIter<'a> {
    type Item = (&'a String, &'a RefValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, (_, val))) = self.hash_iter.next() {
            Some((key, val))
        } else { None }
    }
}

impl<'a> IntoIterator for &'a RefDefMap {
    type Item = (&'a String, &'a RefValue);
    type IntoIter = RefDefMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}



