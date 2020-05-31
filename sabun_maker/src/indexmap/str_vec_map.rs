use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter, Error};
use std::slice::from_raw_parts;


/// StringをKeyとし、入れた順番を保持する、追加は出来るが削除はできないハッシュマップ
/// Removeを実装しないことで、Vecとindexでのアイテムの指定を可能にし、データを連続させ可能な限り間接ポインタアクセス、キャッシュミスを減らそうと試みている
/// ついでにstringのcapacityも削っている。
/// 本当にやる気なら、理論的にはRawVecを使うことでVec.lenも削れるが・・・8バイトは大きいか・・・？
///
/// 性能的なことを言うと、大体の数字でいえば、iterationはHashMapより50%速い
/// lookupはHashMapより5%遅い
/// constructはHashMapより早いが、HashMap(Box(value))よりなぜか遅い
/// IndexMapやlinkedHashMapと比べると、with_capacityがない場合のconstructが遅い
/// これら2つはlookup性能がHashMapより10%低い。
/// LinkedHashMapはiterationがこれの5倍ぐらい遅く、with_capacityがなくてもそんなにconstruct性能が落ちない
/// indexmapはiteration性能はこれと同じで、with_capacityがなくてもそんなに落ちないのも同じ
///
/// with_capacityがあれば、順番保持できるHashMapの中で、lookup,construct,iteration全てにおいて最強というのが今の所の評価
#[derive(Debug, PartialEq)]
pub struct StrVecMap<V>{
    contents : Vec<(StrSlice,V)>,
    ///Vecは再構成されるのでポインタとしては使えないが、キーの書き換えも削除もないのでインデックスで参照出来る
    map : HashMap<MapKey, usize>
}

///len==capacityのStringから作られ、バッファの所有権を持つ
struct StrSlice{
    buf : *mut u8,
    len : usize,
}

impl StrSlice{
    fn new(s : String) -> StrSlice{
        let mut s = s;
        s.shrink_to_fit();
        let ptr = s.as_mut_ptr();
        let len = s.len();
        std::mem::forget(s);

        StrSlice { buf: ptr, len }
    }

    fn to_string(self) -> String{
        unsafe{
            let buf = self.buf;
            let len = self.len;
            std::mem::forget(self);
            String::from_raw_parts(buf, len, len)
        }
    }

    fn to_slice(&self) -> &str{
        unsafe{
            let s = from_raw_parts(self.buf, self.len);
            std::str::from_utf8_unchecked(s)
        }
    }
}

impl Drop for StrSlice{
    ///これでええんとちゃいますかね・・・？
    fn drop(&mut self) {
        unsafe{
            String::from_raw_parts(self.buf, self.len, self.len);
        }
    }
}

impl Debug for StrSlice{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error>{
        self.to_slice().fmt(f)
    }
}

impl PartialEq for StrSlice{
    fn eq(&self, other: &Self) -> bool {
        self.to_slice() == other.to_slice()
    }
}

///基本的にunsafe。Stringの寿命よりこれが長生きした場合ぶっ壊れる
#[repr(C)]
struct MapKey{
    buf : *const u8,
    len : usize,
}

impl MapKey{
    fn to_slice(&self) -> &str{
        unsafe{
            let s = from_raw_parts(self.buf, self.len);
            std::str::from_utf8_unchecked(s)
        }
    }
}

impl PartialEq for MapKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_slice() == other.to_slice()
    }
}

impl Eq for MapKey{}

impl Hash for MapKey {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.to_slice().hash(state)
    }
}

impl Debug for MapKey{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error>{
        self.to_slice().fmt(f)
    }
}

impl<V> StrVecMap<V> {
    pub fn new() -> StrVecMap<V>{
        StrVecMap { contents: Vec::new(), map: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> StrVecMap<V> {
        StrVecMap { contents: Vec::with_capacity(capacity), map: HashMap::with_capacity(capacity) }
    }

    pub fn len(&self) -> usize { self.contents.len() }

    pub fn insert(&mut self, key: String, value: V) -> Option<V> {
        let tmp_key = MapKey { buf: key.as_ptr(), len: key.len() };

        match self.map.get(&tmp_key) {
            Some(idx) => {
                let (_key,val) = unsafe { self.contents.get_unchecked_mut(*idx) };
                Some(std::mem::replace(val, value))
            }
            None => {
                let slice = StrSlice::new(key);
                let mapkey = MapKey { buf: slice.buf, len : slice.len };
                self.map.insert(mapkey, self.contents.len());
                self.contents.push((slice, value));
                None
            }
        }
    }

    pub fn get(&self, key : &str) -> Option<&V>{
         match self.map.get(&MapKey{ buf : key.as_ptr(), len : key.len() }){
             Some(idx) => unsafe{
                 Some(&self.contents.get_unchecked(*idx).1)
             },
             None => None,
         }
     }

     pub fn iter(&self) -> StrVecMapIter<V> { StrVecMapIter{ vec : &self.contents, counter : 0 } }

     pub fn into_iter(self) -> StrVecMapIntoIter<V>{ StrVecMapIntoIter{ iter : self.contents.into_iter() } }
}




pub struct StrVecMapIter<'a, V>{
    vec : &'a Vec<(StrSlice,V)>,
    counter : usize,
}

impl<'a, V> Iterator for StrVecMapIter<'a, V> {
    type Item = (&'a str, &'a V);

    fn next(&mut self) -> Option<Self::Item>{
        if self.counter < self.vec.len() {
            let counter = self.counter;

            self.counter += 1;
            unsafe{
                let (s, v)  = self.vec.get_unchecked(counter);
                Some((s.to_slice(), v))
            }
        } else{
            None
        }
    }
}

impl<'a, V> IntoIterator for &'a StrVecMap<V> {
     type Item = (&'a str, &'a V);
     type IntoIter = StrVecMapIter<'a, V>;

     fn into_iter(self) -> Self::IntoIter {
         self.iter()
     }
}


pub struct StrVecMapIntoIter<V>{
    iter : std::vec::IntoIter<(StrSlice,V)>
}

impl<V> Iterator for StrVecMapIntoIter<V> {
    type Item = (String, V);

    fn next(&mut self) -> Option<Self::Item>{
        match self.iter.next(){
            Some((slice, value)) =>{
                Some((slice.to_string(), value))
            },
            None => None
        }
    }
}

impl<V> IntoIterator for StrVecMap<V> {
    type Item = (String, V);
    type IntoIter = StrVecMapIntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use crate::indexmap::str_vec_map::StrVecMap;
//
//     #[test]
//     fn it_works() {
//         let mut im : StrVecMap<i32> = StrVecMap::new();
//         im.insert("hoge".to_string(), 10);
//         im.insert("hoge2".to_string(), 20);
//
//         for (k,v) in &im{
//             println!("{} {}", k, v);
//         }
//
//
//         println!("{:?}", im.insert("hoge".to_string(), 40));
//         for (k,v) in im{
//             println!("{} {}", k, v);
//         }
//     }
// }