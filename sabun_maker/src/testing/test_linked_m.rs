#[cfg(test)]
mod tests {
    use crate::imp::structs::linked_m::LinkedMap;

    fn create_map0() -> LinkedMap<i64>{
        LinkedMap::new()
    }
    fn create_map1() -> LinkedMap<i64>{
        let mut map = LinkedMap::new();
        map.insert(0);
        map
    }
    fn create_map2() -> LinkedMap<i64>{
        let mut map = LinkedMap::new();
        map.insert(0);
        map.insert(1);
        map
    }
    fn create_map3() -> LinkedMap<i64>{
        let mut map = LinkedMap::new();
        map.insert(0);
        map.insert(1);
        map.insert(2);
        map
    }

    #[test]
    fn first() {
        assert_eq!(create_map0().first(), None);
        assert_eq!(create_map1().first(), Some(&0));
        assert_eq!(create_map2().first(), Some(&0));
        assert_eq!(create_map3().first(), Some(&0));
    }
    #[test]
    fn first_mut() {
        assert_eq!(create_map0().first_mut(), None);
        assert_eq!(create_map1().first_mut(), Some(&mut 0));
        assert_eq!(create_map2().first_mut(), Some(&mut 0));
        assert_eq!(create_map3().first_mut(), Some(&mut 0));
    }
    #[test]
    fn first_id() {
        assert_eq!(create_map0().first_id(), None);
        assert_eq!(create_map1().first_id(), Some(0));
        assert_eq!(create_map2().first_id(), Some(0));
        assert_eq!(create_map3().first_id(), Some(0));
    }
    #[test]
    fn last() {
        assert_eq!(create_map0().last(), None);
        assert_eq!(create_map1().last(), Some(&0));
        assert_eq!(create_map2().last(), Some(&1));
        assert_eq!(create_map3().last(), Some(&2));
    }
    #[test]
    fn last_mut() {
        assert_eq!(create_map0().last_mut(), None);
        assert_eq!(create_map1().last_mut(), Some(&mut 0));
        assert_eq!(create_map2().last_mut(), Some(&mut 1));
        assert_eq!(create_map3().last_mut(), Some(&mut 2));
    }
    #[test]
    fn last_id() {
        assert_eq!(create_map0().last_id(), None);
        assert_eq!(create_map1().last_id(), Some(0));
        assert_eq!(create_map2().last_id(), Some(1));
        assert_eq!(create_map3().last_id(), Some(2));
    }
    #[test]
    fn from_id() {
        let map = create_map3();
        assert_eq!(map.from_id(1), Some(&1));
    }
    #[test]
    fn from_id_mut() {
        let mut map = create_map3();
        assert_eq!(map.from_id_mut(1), Some(&mut 1));
    }

    #[test]
    fn insert() {
        let mut map = LinkedMap::new();
        map.insert(1);
        map.insert(2);
        map.insert_first(3);
        map.insert_first(4);
        map.insert_last(5);
        let vec = values(&map);
        assert_eq!(vec, vec![4,3,1,2,5]);
    }

    fn values(map : &LinkedMap<i64>) -> Vec<i64>{
        map.iter().map(|(_k,v)| *v).collect()
    }
    fn keys(map : &LinkedMap<i64>) -> Vec<u64>{
        map.iter().map(|(k,_v)| *k).collect()
    }

    #[test]
    fn remove_first(){
        let mut map = create_map3();
        map.remove_first();
        assert_eq!(keys(&map), vec![1,2]);
        let mut map = create_map2();
        map.remove_first();
        assert_eq!(keys(&map), vec![1]);
        let mut map = create_map1();
        map.remove_first();
        assert_eq!(keys(&map), vec![]);
        let mut map = create_map0();
        map.remove_first();
        assert_eq!(keys(&map), vec![]);
    }

    #[test]
    fn remove_last(){
        let mut map = create_map3();
        map.remove_last();
        assert_eq!(keys(&map), vec![0,1]);
        let mut map = create_map2();
        map.remove_last();
        assert_eq!(keys(&map), vec![0]);
        let mut map = create_map1();
        map.remove_last();
        assert_eq!(keys(&map), vec![]);
        let mut map = create_map0();
        map.remove_last();
        assert_eq!(keys(&map), vec![]);
    }

    #[test]
    fn remove(){
        let mut map = create_map3();
        map.remove(0);
        assert_eq!(keys(&map), vec![1,2]);
        let mut map = create_map3();
        map.remove(1);
        assert_eq!(keys(&map), vec![0,2]);
        let mut map = create_map3();
        map.remove(2);
        assert_eq!(keys(&map), vec![0,1]);
        let mut map = create_map2();
        map.remove(0);
        assert_eq!(keys(&map), vec![1]);
        let mut map = create_map2();
        map.remove(1);
        assert_eq!(keys(&map), vec![0]);
        let mut map = create_map1();
        map.remove(0);
        assert_eq!(keys(&map), vec![]);
        let mut map = create_map0();
        assert_eq!(map.remove(0), false);
    }

    #[test]
    fn to_honyarara(){
        let mut map = create_map0();
        map.to_first(0);
        map.to_last(0);
        map.to_next(0,0);
        map.to_prev(0,0);
        map.to_next(0,1);
        map.to_prev(0,1);
        let mut map = create_map1();
        map.to_first(0);
        map.to_last(0);
        map.to_next(0,0);
        map.to_prev(0,0);
        map.to_next(0,1);
        map.to_prev(0,1);
        let mut map = create_map2();
        map.to_next(1, 0);
        assert_eq!(keys(&map), vec![1,0]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_next(1, 0);
        assert_eq!(keys(&map), vec![1,0]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_prev(1, 0);
        assert_eq!(keys(&map), vec![0,1]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_next(1, 0);
        assert_eq!(keys(&map), vec![1,0]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_first(1);
        map.to_last(0);
        assert_eq!(keys(&map), vec![1,0]);
        map.to_first(0);
        assert_eq!(keys(&map), vec![0,1]);
        map.to_last(0);
        assert_eq!(keys(&map), vec![1,0]);
        map.to_next(1,1);
        map.to_prev(1,1);

        let mut map = create_map3();
        map.to_next(2, 0);
        assert_eq!(keys(&map), vec![1,2,0]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_next(2, 1);
        assert_eq!(keys(&map), vec![2,1,0]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &0);
        map.to_next(2, 0);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);
        map.to_next(2, 2);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);
        map.to_next(1, 1);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);
        map.to_next(0, 0);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);
        map.to_next(2, 0);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);
        map.to_next(0, 1);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);

        map.to_prev(2, 1);
        assert_eq!(keys(&map), vec![1,2,0]);
        assert!(map.first().is_some());
        assert!(map.last().is_some());
        map.to_prev(2, 0);
        assert_eq!(keys(&map), vec![1,0,2]);
        assert_eq!(map.first().unwrap(), &1);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(2, 1);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(2, 2);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(1, 1);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(0, 0);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(1, 0);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_prev(2, 1);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);

        map.to_last(0);
        assert_eq!(keys(&map), vec![1,2,0]);
        assert_eq!(map.first().unwrap(), &1);
        assert_eq!(map.last().unwrap(), &0);
        map.to_last(2);
        assert_eq!(keys(&map), vec![1,0,2]);
        assert_eq!(map.first().unwrap(), &1);
        assert_eq!(map.last().unwrap(), &2);
        map.to_last(2);
        assert_eq!(keys(&map), vec![1,0,2]);
        assert_eq!(map.first().unwrap(), &1);
        assert_eq!(map.last().unwrap(), &2);

        map.to_first(1);
        assert_eq!(keys(&map), vec![1,0,2]);
        assert_eq!(map.first().unwrap(), &1);
        assert_eq!(map.last().unwrap(), &2);
        map.to_first(0);
        assert_eq!(keys(&map), vec![0,1,2]);
        assert_eq!(map.first().unwrap(), &0);
        assert_eq!(map.last().unwrap(), &2);
        map.to_first(2);
        assert_eq!(keys(&map), vec![2,0,1]);
        assert_eq!(map.first().unwrap(), &2);
        assert_eq!(map.last().unwrap(), &1);

    }

    #[test]
    fn unsafe_iter(){
        let mut map = create_map3();
        let mut iter = unsafe{ map.iter_unsafe() };
        assert_eq!(iter.next(), Some((&0,&0)));
        assert_eq!(iter.current(), Some((&1,&1)));
        assert_eq!(iter.prev(), Some((&1,&1)));
        map.remove(1);
        assert_eq!(iter.next(), Some((&0,&0)));
        assert_eq!(iter.current(), Some((&2,&2)));

        let mut map = create_map3();
        let mut iter = unsafe{ map.iter_unsafe() };
        assert_eq!(iter.current(), Some((&0,&0)));
        map.to_last(0);
        assert_eq!(iter.prev(), Some((&0,&0)));
        assert_eq!(iter.prev(), Some((&2,&2)));
        assert_eq!(iter.prev(), Some((&1,&1)));
        assert_eq!(iter.prev(), None);
        assert_eq!(iter.prev(), None);


    }
}