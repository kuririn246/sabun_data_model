use crate::imp::structs::linked_m::LinkedMap;
use crate::imp::structs::rust_list::MutListItem;
use std::marker::PhantomData;

pub struct MutIntf<'a, V>{
    map : &'a LinkedMap<MutListItem>,
    phantom : PhantomData<&'a V>,
}

#[derive(Debug)]
struct BaseItem{
    mem : u64,
}
#[derive(Debug)]
struct WrappedItem<'a>{
    base : &'a BaseItem
}
impl<'a> From<&'a BaseItem> for WrappedItem<'a>{
    fn from(base : &'a BaseItem) -> Self {
        WrappedItem{ base }
    }
}
#[derive(Debug)]
struct List<'a, V : From<&'a BaseItem>>{
    vec : Vec<BaseItem>,
    phantom : PhantomData<&'a V>
}

impl<'a, V : From<&'a BaseItem>> List<'a, V>{
    fn get(&'a self, index : usize) -> V{
        V::from(self.vec.get(index).unwrap())
    }
}

#[test]
fn test_something(){
    let mut list : List<WrappedItem> = List{ vec : vec![], phantom : PhantomData };
    list.vec.push(BaseItem{ mem : 1 });
    let hoge = list.get(0);
    println!("{:?}", hoge);
    let hoge2 = list.get(0);
    println!("{:?} {:?}", hoge2, hoge);
}