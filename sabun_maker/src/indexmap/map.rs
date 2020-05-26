use std::collections::HashMap;

pub struct IndexMap<K, V>{
    contents : Vec<V>,
    map : HashMap<K, usize>
}