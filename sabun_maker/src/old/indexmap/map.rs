// use crate::HashM};
// use std::hash::{Hash, Hasher};
// use std::fmt::Debug;
//
//
// ///順番を保持するハッシュマップ。Remove(未実装)はO(n)になるが、実データがVecなのでiterationが早い。
// /// KeyとValueのポインタ２つをハッシュテーブルに入れているのでコンパクトだしlookup効率はいいはずでござる。
// /// 通常Removeは最後尾とのswapで実装するが、「Removeすると順番が保持されない」とかいうなんだかよくわからないことになる
// /// それならただのハッシュマップでいいような？　しかし効率が良くてボーナスもあるMapとして人気があるらしい。
// /// 順番保持という目的だとLinkedHashMapの方が基本的に優れてるように思う
// ///
// /// crates.ioにあるIndexMapはcfg設定が特殊でIntelliJでサジェストできないので試しに自作
// #[derive(Debug, PartialEq)]
// pub struct IndexMap<K : Eq + Hash, V>{
//     ///Vecは領域が繰り返し作り直されるので、ポインタを永続させるためにBoxが必要
//     contents : Vec<Box<(K,V)>>,
//     ///Boxの中のポインタをHashMapで保持。
//     map : HashMap<IndexMapKey<K>, *mut V>
// }
//
// #[derive(Debug)]
// struct IndexMapKey<K : Eq + Hash>{
//     key : *const K,
// }
//
// impl<K> PartialEq for IndexMapKey<K> where K : Eq + Hash{
//     fn eq(&self, other : &Self) -> bool{
//         unsafe {
//             K::eq(&*self.key, &*other.key)
//         }
//     }
// }
//
// impl<K : Eq + Hash> Eq for IndexMapKey<K>{}
//
// impl<K: Eq + Hash> Hash for IndexMapKey<K> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         unsafe{ K::hash(&*self.key, state) }
//     }
// }
//
// impl<K : Eq + Hash, V> IndexMap<K,V>{
//     pub fn new() -> IndexMap<K,V>{
//         IndexMap{ contents : Vec::new(), map : HashMap::new() }
//     }
//
//     pub fn with_capacity(capacity : usize) -> IndexMap<K,V>{ IndexMap{ contents : Vec::with_capacity(capacity), map : HashMap::with_capacity(capacity) }}
//
//     pub fn len(&self) -> usize{ self.contents.len() }
//
//     pub fn insert(&mut self, key : K, value : V) -> Option<V>{
//         let temp_key = IndexMapKey{ key : &key };
//         match self.map.get(&temp_key){
//             Some(ptr) => Some(unsafe{ std::mem::replace(&mut **ptr, value) }),
//             None=>{
//                 self.contents.push(Box::new((key,value)));
//                 let (key, value) = self.contents.last_mut().unwrap().as_mut();
//                 self.map.insert(IndexMapKey{ key }, value);
//                 None
//             }
//         }
//     }
//
//     pub fn get(&self, key : &K) -> Option<&V>{
//         match self.map.get(&IndexMapKey{ key }){
//             Some(value) => unsafe{ Some(&**value) },
//             None => None,
//         }
//     }
//
//     pub fn iter(&self) -> IndexMapIter<K, V> { IndexMapIter{ map : &self, counter : 0 } }
//
//     pub fn into_iter(self) -> IndexMapIntoIter<K,V>{ IndexMapIntoIter{ iter : self.contents.into_iter() } }
// }
//
//
// impl<'a, K : Eq + Hash, V> IntoIterator for &'a IndexMap<K, V> {
//     type Item = (&'a K, &'a V);
//     type IntoIter = IndexMapIter<'a, K, V>;
//
//     fn into_iter(self) -> IndexMapIter<'a, K, V> {
//         self.iter()
//     }
// }
//
// pub struct IndexMapIter<'a, K : Eq + Hash, V>{
//     map : &'a IndexMap<K, V>,
//     counter : usize,
// }
//
// impl<'a, K : Eq + Hash,V> Iterator for IndexMapIter<'a, K,V> {
//     type Item = (&'a K, &'a V);
//
//     fn next(&mut self) -> Option<Self::Item>{
//         if self.counter < self.map.len() {
//             let counter = self.counter;
//
//             self.counter += 1;
//             let (k,v) = &self.map.contents[counter].as_ref();
//             return Some((k,v));
//         } else{
//             None
//         }
//     }
// }
//
// pub struct IndexMapIntoIter<K : Eq + Hash, V>{
//     iter : std::vec::IntoIter<Box<(K,V)>>
// }
//
// impl<K : Eq + Hash,V> Iterator for IndexMapIntoIter<K,V> {
//     type Item = (K, V);
//
//     fn next(&mut self) -> Option<Self::Item>{
//         match self.iter.next(){
//             Some(b) =>{ Some(*b) },
//             None => None
//         }
//     }
// }
//
//
//
//
// //
// // #[cfg(test)]
// // mod tests {
// //     use crate::indexmap::map::{IndexMap, IndexMapKey };
// //     use crate::HashM;
// //
// //     #[test]
// //     fn it_works() {
// //         let mut im : IndexMap<String, i32> = IndexMap::new();
// //         im.insert("hoge".to_string(), 10);
// //         im.insert("hoge2".to_string(), 20);
// //
// //         for (k,v) in &im{
// //             println!("{} {}", k, v);
// //         }
// //
// //
// //         println!("{:?}", im.insert("hoge".to_string(), 40));
// //         for (k,v) in &im{
// //             println!("{} {}", k, v);
// //         }
// //
// //     }
// //
// //     // #[test]
// //     // fn it_works() {
// //     //     let mut vec : Vec<(usize,usize)> = vec![(10, 10), (20,60)];
// //     //     let mut map : HashMap<String, *mut usize> = HashMap::new();
// //     //     let (_,hoge) = vec.last_mut().unwrap();
// //     //     map.insert("item".to_string(), hoge);
// //     //     let hoge2 = map.get_mut(&"item".to_string()).unwrap();
// //     //
// //     //     let hoge = unsafe{ std::mem::replace(&mut **hoge2, 40) };
// //     //     println!("{:?} {}", vec[1], hoge);
// //     // }
// //
// //
// //
// // }