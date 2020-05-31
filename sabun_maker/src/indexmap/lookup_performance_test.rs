//
//
//
// #[cfg(test)]
// mod tests {
//     use std::collections::{HashMap, HashSet};
//     use crate::indexmap::str_vec_map::StrVecMap;
//     use rand::prelude::ThreadRng;
//     use rand::Rng;
//
//     use test::Bencher;
//     use linked_hash_map::LinkedHashMap;
//     use crate::indexmap::IndexMap;
//     use crate::structs::rust_value::RustValue;
//     use crate::structs::root_object::ListDefObj;
//     use crate::structs::rust_list::MutList;
//
//     const NAME_LEN : usize = 5;
//     const NUM_NAMES : usize = 100_0000;
//     const FOLD : usize = 1;
//     const REPEAT : usize = 1000;
//
//     struct BigItem {
//         map1: HashMap<String, usize>,
//         map2: HashMap<String, usize>,
//         map3: HashMap<String, usize>,
//     }
//
//     impl BigItem {
//         fn new() -> BigItem {
//             BigItem{ map1 : HashMap::new(), map2 : HashMap::new(), map3 : HashMap::new() }
//         }
//     }
//
//     // #[test]
//     // fn hoge(){
//     //     println!("{}", core::mem::size_of::<RustValue>());
//     //     println!("{}", core::mem::size_of::<ListDefObj>());
//     //     println!("{}", core::mem::size_of::<MutList>());
//     //     println!("{}", core::mem::size_of::<Box<usize>>());
//     //     println!("{}", core::mem::size_of::<LinkedHashMap<String, usize>>());
//     //     println!("{}", core::mem::size_of::<StrVecMap<usize>>());
//     //     println!("{}", core::mem::size_of::<HashMap<String, usize>>());
//     //     println!("{}", core::mem::size_of::<HashSet<usize>>());
//     // }
//
//     fn init1() -> (ThreadRng, Vec<String>) {
//         let mut rng = rand::thread_rng();
//
//         let mut names : Vec<String> = vec![];
//
//         for _ in 0..NUM_NAMES{
//             names.push(get_random_name(NAME_LEN, rng));
//         }
//
//         (rng, names)
//     }
//
//     fn get_random_name(len : usize, mut rng : ThreadRng) -> String{
//         let mut s = String::with_capacity(len);
//         for i in 0..len {
//             s.push(rng.gen_range('a' as u8, 'z' as u8) as char );
//         }
//         return s;
//     }
//
//
//     #[bench]
//     fn bench_str_vec(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_str_vec_fast(&names);
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_str_vec(&names, &map)))
//     }
//
//     #[bench]
//     fn bench_str_vec_with_box(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_str_vec_fast_with_box(&names);
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_str_vec_with_box(&names, &map)))
//     }
//
//     #[bench]
//     fn bench_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_hash_map_fast(&names);
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_hash_map(&names, &map)))
//     }
//
//     #[bench]
//     fn bench_hash_map_with_box(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_hash_map_fast_with_box(&names);
//
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_hash_map_with_box(&names, &map)))
//     }
//
//     #[bench]
//     fn bench_index_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_index_map_fast(&names);
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_index_map(&names, &map)))
//     }
//
//     #[bench]
//     fn bench_linked_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_linked_hash_map_fast(&names);
//         b.iter(|| (0..FOLD).fold(0,|a,_| a + lookup_linked_hash_map(&names, &map)))
//     }
//
//
//     fn lookup_str_vec(names : &Vec<String>, map : &StrVecMap<BigItem>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn lookup_str_vec_with_box(names : &Vec<String>, map : &StrVecMap<Box<BigItem>>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn lookup_hash_map(names : &Vec<String>, map : &HashMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn lookup_hash_map_with_box(names : &Vec<String>, map : &HashMap<String, Box<BigItem>>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn lookup_linked_hash_map(names : &Vec<String>, map : &LinkedHashMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn lookup_index_map(names : &Vec<String>, map : &IndexMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for i in 0..REPEAT{
//             let name = &names[i];
//             match map.get(name){
//                 Some(b ) => ans += b.map1.len(),
//                 _=>{}
//             }
//         }
//
//         return ans;
//     }
//
//     fn init_str_vec_fast(names : &Vec<String>) -> StrVecMap<BigItem> {
//         let mut map : StrVecMap<BigItem> = StrVecMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem::new());
//         }
//
//         return map;
//     }
//
//     fn init_str_vec_fast_with_box(names : &Vec<String>) -> StrVecMap<Box<BigItem>> {
//         let mut map : StrVecMap<Box<BigItem>> = StrVecMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), Box::new(BigItem::new()));
//         }
//
//         return map;
//     }
//
//     fn init_index_map_fast(names : &Vec<String>) -> IndexMap<String, BigItem> {
//         let mut map : IndexMap<String,BigItem> = IndexMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem::new());
//         }
//
//         return map;
//     }
//
//     fn init_linked_hash_map_fast(names : &Vec<String>) -> LinkedHashMap<String, BigItem>{
//         let mut map : LinkedHashMap<String, BigItem> = LinkedHashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem::new());
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_fast(names : &Vec<String>) -> HashMap<String, BigItem>{
//         let mut map : HashMap<String, BigItem> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem::new());
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_fast_with_box(names : &Vec<String>) -> HashMap<String, Box<BigItem>>{
//         let mut map : HashMap<String, Box<BigItem>> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), Box::new(BigItem::new()));
//         }
//
//         return map;
//     }
// }