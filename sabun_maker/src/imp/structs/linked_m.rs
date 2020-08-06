use crate::HashM;
use crate::imp::structs::rust_list::MutListItem;
use std::collections::hash_map::{Iter, IntoIter};
use std::ptr::{null_mut, null};

unsafe impl Send for MutListHash{}
unsafe impl Sync for MutListHash{}

#[derive(Debug, PartialEq, Clone)]
pub struct MutListHash{
    map : HashM<u64, Box<MutNode>>,
    first : *mut MutNode,
    last : *mut MutNode,
    next_id : u64,
}


#[derive(Debug, PartialEq, Clone)]
struct MutNode{
    prev : *mut MutNode,
    next : *mut MutNode,
    item : MutListItem,
    id : u64,
}
impl MutNode{
    fn new(item : MutListItem, id : u64) -> MutNode{
        MutNode{ item, prev : null_mut(), next : null_mut(), id }
    }
}
fn set_next(this : *mut MutNode, next : *mut MutNode){
    let node = unsafe{ this.as_mut().unwrap() };
    node.next = next;
}
fn set_prev(this : *mut MutNode, prev : *mut MutNode){
    let node = unsafe{ this.as_mut().unwrap() };
    node.prev = prev;
}

impl MutListHash{
    pub(crate) fn new() -> MutListHash{
        MutListHash{ map : HashM::default(), first : null_mut(), last : null_mut(), next_id : 0, }
    }

    pub fn insert(&mut self, val : MutListItem) -> u64{
        self.insert_last(val)
    }

    pub fn insert_last(&mut self, val : MutListItem) -> u64{
        let mut node = Box::new(MutNode::new(val, self.next_id));
        if self.last.is_null(){
            let ptr = node.as_mut() as *mut MutNode;
            self.last = ptr;
            self.first = ptr;
            self.map.insert(self.next_id, node);
            self.next_id += 1;
            return self.next_id - 1;
        } else{
            node.as_mut().prev = self.last;
            set_next(self.last, node.as_mut());
            self.last = node.as_mut();
            self.map.insert(self.next_id, node);
            self.next_id += 1;
            return self.next_id - 1;
        }
    }
    pub fn insert_first(&mut self, val : MutListItem) -> u64{
        let mut node = Box::new(MutNode::new(val, self.next_id));
        if self.first.is_null(){
            let ptr = node.as_mut() as *mut MutNode;
            self.last = ptr;
            self.first = ptr;
            self.map.insert(self.next_id, node);
            self.next_id += 1;
            return self.next_id - 1;
        } else{
            node.as_mut().next = self.first;
            set_prev(self.first, node.as_mut());
            self.first = node.as_mut();
            self.map.insert(self.next_id, node);
            self.next_id += 1;
            return self.next_id - 1;
        }
    }

    fn node_from_id_mut(&mut self, id : u64) -> Option<&mut MutNode>{ self.map.get_mut(&id).map(|b| b.as_mut()) }

    pub fn from_id(&self, id : u64) -> Option<&MutListItem>{ self.map.get(&id).map(|b| &b.as_ref().item) }
    pub fn from_id_mut(&mut self, id : u64) -> Option<&mut MutListItem>{ self.map.get_mut(&id).map(|b| &mut b.as_mut().item) }

    pub fn contains_key(&self, key : u64) -> bool{ self.map.contains_key(&key) }
    pub fn len(&self) -> usize{ self.map.len() }

    pub fn remove(&mut self, id : u64) -> bool {
        let node = if let Some(node) = self.node_from_id_mut(id) { node } else { return false; };
        if node == self.first {
            return self.remove_first();
        }
        if node == self.last {
            return self.remove_last();
        }
        //lastかfirstでないなら前と後ろがあるはずである
        let (next, prev) = unsafe { (node.next.as_mut().unwrap(), node.prev.as_mut().unwrap()) };
        next.prev = prev;
        prev.next = next;
        self.map.remove(&id);
        return true;
    }
    pub fn remove_first(&mut self) -> bool{
        if self.first.is_null(){ return false; }

        let first = unsafe{ self.first.as_ref().unwrap() };
        let next = unsafe{ first.next.as_mut().unwrap() };
        next.prev = null_mut();
        self.first = next;
        self.map.remove(&first.id);
        return true;
    }

    pub fn remove_last(&mut self) -> bool{
        if self.last.is_null(){ return false; }

        let last = unsafe{ self.last.as_ref().unwrap() };
        let prev = unsafe{ last.prev.as_mut().unwrap() };
        prev.next = null_mut();
        self.last = prev;
        self.map.remove(&last.id);
        return true;
    }

    pub fn to_first(&mut self, id : u64) -> bool{
        let node = if let Some(node) = self.node_from_id_mut(id){ node } else{ return false; }
        if node == self.first { return true; }
        if node == self.last{ }
    }

    pub fn swap_first_for_last(&mut self) -> bool{
        if self.first.is_null(){ return false; }
    }
    // pub(crate) fn iter(&self) -> MutListHashIter{ MutListHashIter{ hash_iter : self.map.iter() } }
    // pub(crate) fn into_iter(self) -> IntoIter<u64, Box<MutListItem>>{ self.map.into_iter() }



}

// pub struct MutListHashIter<'a>{
//     hash_iter : Iter<'a, u64, Box<MutListItem>>,
// }
//
// impl<'a> Iterator for MutListHashIter<'a>{
//     type Item = (&'a u64, &'a MutListItem);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some((key, val)) = self.hash_iter.next(){
//             Some((key, val.as_ref()))
//         }  else{ None }
//     }
// }
//
// impl<'a> IntoIterator for &'a MutListHash{
//     type Item = (&'a u64, &'a MutListItem);
//     type IntoIter = MutListHashIter<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
// impl IntoIterator for MutListHash{
//     type Item = (u64, Box<MutListItem>);
//     type IntoIter = IntoIter<u64, Box<MutListItem>>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.into_iter()
//     }
// }