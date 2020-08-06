use crate::{HashM, HashMt};
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

fn ptr_eq<T>(l : *const T, r : *const T) -> bool{ std::ptr::eq(l,r) }

impl MutListHash{
    pub(crate) fn new(capacity : usize) -> MutListHash{
        MutListHash{ map : HashMt::with_capacity(capacity), first : null_mut(), last : null_mut(), next_id : 0, }
    }

    pub fn first(&self) -> &MutListItem{
        let first = unsafe{ self.first.as_ref().unwrap()};
        &first.item
    }
    pub fn first_mut(&mut self) -> &mut MutListItem{
        let first = unsafe{ self.first.as_mut().unwrap()};
        &mut first.item
    }
    pub fn first_id(&self) -> u64{
        let first = unsafe{ self.first.as_ref().unwrap()};
        first.id
    }
    pub fn last(&self) -> &MutListItem{
        let last = unsafe{ self.last.as_ref().unwrap()};
        &last.item
    }
    pub fn last_mut(&mut self) -> &mut MutListItem{
        let last = unsafe{ self.last.as_mut().unwrap()};
        &mut last.item
    }
    pub fn last_id(&self) -> u64{
        let last = unsafe{ self.last.as_ref().unwrap()};
        last.id
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
        if ptr_eq(node,self.first){
            return self.remove_first();
        }
        if ptr_eq(node, self.last) {
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
        if first.next.is_null() == false {
            let next = unsafe { first.next.as_mut().unwrap() };
            next.prev = null_mut();
            self.first = next;
            self.map.remove(&first.id);
        } else{
            self.map.remove(&first.id);
            self.first = null_mut();
            self.last = null_mut();
        }
        return true;
    }

    pub fn remove_last(&mut self) -> bool{
        if self.last.is_null(){ return false; }

        let last = unsafe{ self.last.as_ref().unwrap() };
        if last.prev.is_null() == false {
            let prev = unsafe { last.prev.as_mut().unwrap() };
            prev.next = null_mut();
            self.last = prev;
            self.map.remove(&last.id);
        } else{
            self.map.remove(&last.id);
            self.last = null_mut();
            self.first = null_mut();
        }

        return true;
    }


    pub fn to_first(&mut self, id : u64) -> bool {
        let mut node = if let Some(node) = self.node_from_id_mut(id) { node } else { return false; };
        if ptr_eq(node, self.first) { return true; }
        if ptr_eq(node, self.last) {
            let mut last = unsafe{ self.last.as_mut().unwrap() };
            let mut prev = unsafe{ last.prev.as_mut().unwrap() };
            self.last = prev;
            set_next(self.last, null_mut());

            let mut next = unsafe{self.first.as_mut().unwrap() };
            self.first = last;
            set_prev(self.first, null_mut());
            set_next(self.first, next);
            next.prev = self.first;
            return true;
        }
        let mut node_prev = unsafe{ node.prev.as_mut().unwrap() };
        let mut node_next = unsafe{ node.next.as_mut().unwrap() };
        node_prev.next = node_next;
        node_next.prev = node_prev;

        let mut next = unsafe { self.first.as_mut().unwrap() };
        self.first = node;
        set_prev(self.first, null_mut());
        set_next(self.first, next);
        next.prev = self.first;
        return true;
    }

    pub fn to_last(&mut self, id : u64) -> bool {
        let mut node = if let Some(node) = self.node_from_id_mut(id) { node } else { return false; };
        if ptr_eq(node, self.last) { return true; }
        if ptr_eq(node, self.first) {
            let mut first = unsafe{ self.first.as_mut().unwrap() };
            let mut next = unsafe{ first.next.as_mut().unwrap() };
            self.first = next;
            set_prev(self.first, null_mut());

            let mut prev = unsafe{self.last.as_mut().unwrap() };
            self.last = first;
            set_next(self.last, null_mut());
            set_prev(self.last, prev);
            prev.next = self.last;
            return true;
        }
        let mut node_prev = unsafe{ node.prev.as_mut().unwrap() };
        let mut node_next = unsafe{ node.next.as_mut().unwrap() };
        node_prev.next = node_next;
        node_next.prev = node_prev;

        let mut next = unsafe { self.first.as_mut().unwrap() };
        self.first = node;
        set_prev(self.first, null_mut());
        set_next(self.first, next);
        next.prev = self.first;
        return true;
    }


    ///swapが失敗するとfalse
    pub fn swap(&mut self, id1 : u64, id2 : u64) -> bool{
        if id1 == id2{ return false; }
        let mut l = if let Some(l) =  self.map.get_mut(&id1).map(|i1| i1.as_mut() as *mut MutNode){ l } else{ return false; };
        let mut r = if let Some(r) =  self.map.get_mut(&id2).map(|i2| i2.as_mut() as *mut MutNode){ r } else{ return false; };
        let mut l = unsafe{ l.as_mut().unwrap() };
        let mut r = unsafe{ r.as_mut().unwrap() };

        if self.first_id() == id1{
            self.first = r;
        } else if self.first_id() == id2{
            self.first = l;
        }

        if self.last_id() == id1{
            self.last = r;
        } else if self.last_id() == id2{
            self.last = l;
        }

        if ptr_eq(l.next, r){ return self.swap_adjacent(l, r); }
        if ptr_eq(r.next, l){ return self.swap_adjacent(r, l); }

        let l_next = l.next;
        let l_prev = l.prev;
        let r_next = r.next;
        let r_prev = r.prev;
        r.next = l_next;
        r.prev = l_prev;
        l.next = r_next;
        l.prev = r_prev;
        if l_next.is_null() == false{
            set_prev(l_next, r);
        }
        if l_prev.is_null() == false{
            set_next(l_prev, r);
        }
        if r_next.is_null() == false{
            set_prev(r_next, l);
        }
        if r_prev.is_null() == false{
            set_next(r_prev, l);
        }
        return true;
    }
    fn swap_adjacent(&mut self, l : &mut MutNode, r : &mut MutNode) -> bool{
        let l_prev = l.prev;
        let r_next = r.next;
        r.prev = l_prev;
        l.next = r_next;
        r.next = l;
        l.prev = r;
        return true;
    }
    // pub(crate) fn iter(&self) -> MutListHashIter{ MutListHashIter{ hash_iter : self.map.iter() } }
    // pub(crate) fn into_iter(self) -> IntoIter<u64, Box<MutListItem>>{ self.map.into_iter() }



}



pub struct MutListHashIter<'a>{
    hash_iter : Iter<'a, u64, Box<MutNode>>,
}

impl<'a> Iterator for MutListHashIter<'a>{
    type Item = (&'a u64, &'a MutListItem);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, val)) = self.hash_iter.next(){
            Some((key, &val.as_ref().item))
        }  else{ None }
    }
}
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