//
//
//
// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use crate::indexmap::str_vec_map::StrVecMap;
//     use rand::prelude::ThreadRng;
//     use rand::Rng;
//
//     use test::Bencher;
//     use linked_hash_map::LinkedHashMap;
//     use crate::indexmap::IndexMap;
//
//     const NAME_LEN : usize = 5;
//     const NUM_NAMES : usize = 10;
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
//         b.iter(|| iterate_str_vec(&names, &map))
//     }
//
//     #[bench]
//     fn bench_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_hash_map_fast(&names);
//         b.iter(|| iterate_hash_map(&names, &map))
//     }
//
//     #[bench]
//     fn bench_hash_map_with_box(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_hash_map_fast_with_box(&names);
//         b.iter(|| iterate_hash_map_with_box(&names, &map))
//     }
//
//     #[bench]
//     fn bench_index_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_index_map_fast(&names);
//         b.iter(|| iterate_index_map(&names, &map))
//     }
//
//     #[bench]
//     fn bench_linked_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         let map = init_linked_hash_map_fast(&names);
//         b.iter(|| iterate_linked_hash_map(&names, &map))
//     }
//
//
//     fn iterate_str_vec(names : &Vec<String>, map : &StrVecMap<BigItem>) -> usize{
//         let mut ans = 0;
//
//         for (k,v) in map{
//             ans += k.len() + v.map1.len();
//         }
//
//         return ans;
//     }
//
//     fn iterate_hash_map(names : &Vec<String>, map : &HashMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for (k,v) in map{
//             ans += k.len() + v.map1.len();
//         }
//
//         return ans;
//     }
//
//     fn iterate_hash_map_with_box(names : &Vec<String>, map : &HashMap<String, Box<BigItem>>) -> usize{
//         let mut ans = 0;
//
//         for (k,v) in map{
//             ans += k.len() + v.map1.len();
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
//     fn iterate_index_map(names : &Vec<String>, map : &IndexMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for (k,v) in map{
//             ans += k.len() + v.map1.len();
//         }
//
//         return ans;
//     }
//
//
//     fn iterate_linked_hash_map(names : &Vec<String>, map : &LinkedHashMap<String, BigItem>) -> usize{
//         let mut ans = 0;
//
//         for (k,v) in map{
//             ans += k.len() + v.map1.len();
//         }
//
//         return ans;
//     }
//
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