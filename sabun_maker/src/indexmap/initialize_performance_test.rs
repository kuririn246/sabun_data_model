


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::indexmap::str_vec_map::StrVecMap;
    use rand::prelude::ThreadRng;
    use rand::Rng;

    use test::Bencher;
    use linked_hash_map::LinkedHashMap;
    use crate::indexmap::IndexMap;

    const NAME_LEN : usize = 5;
    const NUM_NAMES : usize = 10;

    ///56バイトだとBoxに入れる価値はない
    /// 168バイトだとBoxに入れる価値大いにある
    struct BigItem {
        map1: HashMap<String, usize>,
        map2: HashMap<String, usize>,
        map3: HashMap<String, usize>,
    }

    impl BigItem {
        fn new() -> BigItem {
            BigItem{
                map1 : HashMap::new(),
                map2 : HashMap::new(),
                map3 : HashMap::new()
            }
        }
    }

    fn init1() -> (ThreadRng, Vec<String>) {
        let mut rng = rand::thread_rng();

        let mut names : Vec<String> = vec![];

        for _ in 0..NUM_NAMES{
            names.push(get_random_name(NAME_LEN, rng));
        }

        (rng, names)
    }

    fn get_random_name(len : usize, mut rng : ThreadRng) -> String{
        let mut s = String::with_capacity(len);
        for i in 0..len {
            s.push(rng.gen_range('a' as u8, 'z' as u8) as char );
        }
        return s;
    }

    #[bench]
    fn bench_init_str_vec_slow(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_str_vec_slow(&names));
    }

    #[bench]
    fn bench_init_str_vec_slow_with_box(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_str_vec_slow_with_box(&names));
    }

    #[bench]
    fn bench_init_str_vec_fast(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_str_vec_fast(&names));
    }

    #[bench]
    fn bench_init_str_vec_fast_with_box(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_str_vec_fast_with_box(&names));
    }

    #[bench]
    fn bench_init_index_map_slow(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_index_map_slow(&names));
    }

    #[bench]
    fn bench_init_index_map_fast(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_index_map_fast(&names));
    }


    #[bench]
    fn bench_init_linked_hash_map_slow(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_linked_hash_map_slow(&names));
    }

    #[bench]
    fn bench_init_linked_hash_map_fast(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_linked_hash_map_fast(&names));
    }

    #[bench]
    fn bench_init_hash_map_slow(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_hash_map_slow(&names));
    }

    #[bench]
    fn bench_init_hash_map_fast(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_hash_map_fast(&names));
    }

    #[bench]
    fn bench_init_hash_map_fast_with_box(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_hash_map_fast_with_box(&names));
    }

    #[bench]
    fn bench_init_hash_map_fast_with_2boxes(b: &mut Bencher) {
        let (rng, names) = init1();

        b.iter(|| init_hash_map_fast_with_2boxes(&names));
    }


    fn init_str_vec_slow(names : &Vec<String>) -> StrVecMap<BigItem> {
        let mut map : StrVecMap<BigItem> = StrVecMap::new();

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_str_vec_slow_with_box(names : &Vec<String>) -> StrVecMap<Box<BigItem>> {
        let mut map : StrVecMap<Box<BigItem>> = StrVecMap::new();

        for name in names{
            map.insert(name.to_string(), Box::new(BigItem::new()));
        }

        return map;
    }

    fn init_str_vec_fast(names : &Vec<String>) -> StrVecMap<BigItem> {
        let mut map : StrVecMap<BigItem> = StrVecMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_str_vec_fast_with_box(names : &Vec<String>) -> StrVecMap<Box<BigItem>> {
        let mut map : StrVecMap<Box<BigItem>> = StrVecMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), Box::new(BigItem::new()));
        }

        return map;
    }

    fn init_index_map_slow(names : &Vec<String>) -> IndexMap<String, BigItem> {
        let mut map : IndexMap<String,BigItem> = IndexMap::new();

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_index_map_fast(names : &Vec<String>) -> IndexMap<String, BigItem> {
        let mut map : IndexMap<String,BigItem> = IndexMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }


    fn init_linked_hash_map_slow(names : &Vec<String>) -> LinkedHashMap<String, BigItem>{
        let mut map : LinkedHashMap<String, BigItem> = LinkedHashMap::new();

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_linked_hash_map_fast(names : &Vec<String>) -> LinkedHashMap<String, BigItem>{
        let mut map : LinkedHashMap<String, BigItem> = LinkedHashMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }


    fn init_hash_map_slow(names : &Vec<String>) -> HashMap<String, BigItem> {
        let mut map: HashMap<String, BigItem> = HashMap::new();

        for name in names {
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_hash_map_fast(names : &Vec<String>) -> HashMap<String, BigItem>{
        let mut map : HashMap<String, BigItem> = HashMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), BigItem::new());
        }

        return map;
    }

    fn init_hash_map_fast_with_box(names : &Vec<String>) -> HashMap<String, Box<BigItem>>{
        let mut map : HashMap<String, Box<BigItem>> = HashMap::with_capacity(names.len());

        for name in names{
            map.insert(name.to_string(), Box::new(BigItem::new()));
        }

        return map;
    }

    fn init_hash_map_fast_with_2boxes(names : &Vec<String>) -> HashMap<Box<String>, Box<BigItem>>{
        let mut map : HashMap<Box<String>, Box<BigItem>> = HashMap::with_capacity(names.len());

        for name in names{
            map.insert(Box::new(name.to_string()), Box::new(BigItem::new()));
        }

        return map;
    }
}