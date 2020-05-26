use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub struct IndexMap<K : Eq + Hash, V : Debug>{
    contents : Vec<(K,V)>,
    map : HashMap<IndexMapKey<K>, usize>
}

#[derive(Debug)]
struct IndexMapKey<K : Eq + Hash>{
    key : *const K,
}

impl<K> PartialEq for IndexMapKey<K> where K : Eq + Hash{
    fn eq(&self, other : &Self) -> bool{
        unsafe {
            K::eq(&*self.key, &*other.key)
        }
    }
}

impl<K : Eq + Hash> Eq for IndexMapKey<K>{}

impl<K: Eq + Hash> Hash for IndexMapKey<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe{ K::hash(&*self.key, state) }
    }
}

impl<K : Eq + Hash, V : Debug> IndexMap<K,V>{
    pub fn new() -> IndexMap<K,V>{
        IndexMap{ contents : vec![], map : HashMap::new() }
    }

    pub fn len(&self) -> usize{ self.contents.len() }

    pub fn insert(&mut self, key : K, value : V) -> Option<V>{
        let temp_key = IndexMapKey{ key : &key };
        match self.map.get(&temp_key){
            Some(ind) => std::mem::replace(&mut self.contents[usize], value),
            None=>{

                self.contents.push((key, value));

            }
        }
        if self.map.contains_key(&temp_key){
            let val = self.map.get_mut(&temp_key).unwrap();
            println!("val {:?}", value);
            unsafe{ println!("moto {:?}", **val); }
            Some(unsafe{ std::mem::replace(&mut **val, value) })
        } else{

            let (key, value) = self.contents.last_mut().unwrap();
            self.map.insert(IndexMapKey { key }, value);
            return None;
        }
    }

    pub fn get(&self, key : &K) -> Option<&V>{
        match self.map.get(&IndexMapKey{ key }){
            Some(value) => unsafe{ Some(&**value) },
            None => None,
        }
    }
    pub fn iter(&self) -> IndexMapIter<K, V> { IndexMapIter{ map : &self, counter : 0 } }
}


impl<'a, K : Eq + Hash, V : Debug> IntoIterator for &'a IndexMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = IndexMapIter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> IndexMapIter<'a, K, V> {
        self.iter()
    }
}

pub struct IndexMapIter<'a, K : Eq + Hash, V : Debug>{
    map : &'a IndexMap<K, V>,
    counter : usize,
}

impl<'a, K : Eq + Hash,V : Debug> Iterator for IndexMapIter<'a, K,V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item>{
        if self.counter < self.map.len() {
            let counter = self.counter;

            self.counter += 1;
            let (k,v) = &self.map.contents[counter];
            return Some((k,v));
        } else{
            None
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::indexmap::map::{IndexMap, IndexMapKey };
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut im : IndexMap<String, i32> = IndexMap::new();
        im.insert("hoge".to_string(), 10);
        // im.insert("hoge2".to_string(), 20);
        //
        // for (k,v) in &im{
        //     println!("{} {}", k, v);
        // }


        println!("{:?}", im.insert("hoge".to_string(), 40));
        for (k,v) in &im{
            println!("{} {}", k, v);
        }

        for (key,val) in &im.contents{
            println!("{} {}", key, val);
        }
    }

    // #[test]
    // fn it_works() {
    //     let mut vec : Vec<(usize,usize)> = vec![(10, 10), (20,60)];
    //     let mut map : HashMap<String, *mut usize> = HashMap::new();
    //     let (_,hoge) = vec.last_mut().unwrap();
    //     map.insert("item".to_string(), hoge);
    //     let hoge2 = map.get_mut(&"item".to_string()).unwrap();
    //
    //     let hoge = unsafe{ std::mem::replace(&mut **hoge2, 40) };
    //     println!("{:?} {}", vec[1], hoge);
    // }



}