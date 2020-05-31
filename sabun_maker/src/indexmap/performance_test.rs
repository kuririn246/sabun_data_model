
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::indexmap::str_vec_map::StrVecMap;

    struct BigItem {
        map1: HashMap<String, usize>,
        map2: HashMap<String, usize>,
        map3: HashMap<String, usize>,
    }

    impl BigItem {
        fn new() -> BigItem {
            BigItem{ map1 : HashMap::new(), map2 : HashMap::new(), map3 : HashMap::new() }
        }
    }

    fn perf_test() {}

    fn get_random_name(len : usize){
        let mut s = String::with_capacity(len);
        
    }

    fn init_str_vec() -> StrVecMap<BigItem> {}
}