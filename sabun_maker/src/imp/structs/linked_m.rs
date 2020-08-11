use crate::{HashM, HashMt};
use std::ptr::{null_mut};
use std::cell::{ RefCell};

unsafe impl<V> Send for LinkedMap<V> {}
unsafe impl<V> Sync for LinkedMap<V> {}

#[derive(Debug)]
pub struct LinkedMap<V>{
    map : HashM<u64, Box<MutNode<V>>>,
    first : *mut MutNode<V>,
    last : *mut MutNode<V>,
    next_id : u64,
}


#[derive(Debug)]
struct MutNode<V>{
    prev : *mut MutNode<V>,
    next : *mut MutNode<V>,
    item : V,
    id : u64,
}
impl<V> MutNode<V>{
    fn new(item : V, id : u64) -> MutNode<V>{
        MutNode{ item, prev : null_mut(), next : null_mut(), id }
    }
}
fn get_next<V>(this : *mut MutNode<V>) -> *mut MutNode<V>{
    let node = unsafe{ this.as_mut().unwrap() };
    node.next
}
fn set_next<V>(this : *mut MutNode<V>, next : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.next = next;
}
fn get_prev<V>(this : *mut MutNode<V>) -> *mut MutNode<V>{
    let node = unsafe{ this.as_mut().unwrap() };
    node.prev
}
fn set_prev<V>(this : *mut MutNode<V>, prev : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.prev = prev;
}
fn get_id<V>(this : *const MutNode<V>) -> u64{
    let node = unsafe{ this.as_ref().unwrap() };
    node.id
}
fn get_item<'a, V>(this : *const MutNode<V>) -> &'a V{
    let node = unsafe{ this.as_ref().unwrap() };
    &node.item
}
fn get_item_mut<'a, V>(this : *mut MutNode<V>) -> &'a mut V{
    let node = unsafe{ this.as_mut().unwrap() };
    &mut node.item
}

fn ptr_eq<T>(l : *const T, r : *const T) -> bool{ std::ptr::eq(l,r) }

impl<V> LinkedMap<V> {
    pub fn new() -> LinkedMap<V> {
        LinkedMap { map : HashMt::new(), first : null_mut(), last : null_mut(), next_id : 0, }
    }

    pub fn first(&self) -> &V{ get_item(self.first) }
    pub fn first_mut(&mut self) -> &mut V{ get_item_mut(self.first) }
    pub fn first_id(&self) -> u64{ get_id(self.first) }
    pub fn last(&self) -> &V{ get_item(self.last) }
    pub fn last_mut(&mut self) -> &mut V{ get_item_mut(self.last) }
    pub fn last_id(&self) -> u64{ get_id(self.last) }

    fn node_from_id_mut(&mut self, id : u64) -> Option<&mut MutNode<V>>{ self.map.get_mut(&id).map(|b| b.as_mut()) }
    fn node_from_id(&self, id : u64) -> Option<&MutNode<V>>{ self.map.get(&id).map(|b| b.as_ref()) }

    pub fn from_id(&self, id : u64) -> Option<&V>{ self.map.get(&id).map(|b| &b.as_ref().item) }
    pub fn from_id_mut(&mut self, id : u64) -> Option<&mut V>{ self.map.get_mut(&id).map(|b| &mut b.as_mut().item) }

    pub fn contains_key(&self, key : u64) -> bool{ self.map.contains_key(&key) }
    pub fn len(&self) -> usize{ self.map.len() }

    fn pull_last(&mut self) -> *mut MutNode<V>{
        if self.last.is_null(){ return null_mut() }
        let last = unsafe{ self.last.as_mut().unwrap() };
        if last.prev.is_null(){
            self.last = null_mut();
            self.first = null_mut();
            return last;
        } else{
            self.last = last.prev;
            set_next(self.last, null_mut());
            last.prev = null_mut();
            return last;
        }
    }

    fn pull_first(&mut self) -> *mut MutNode<V>{
        if self.first.is_null(){ return null_mut() }
        let first = unsafe{ self.first.as_mut().unwrap() };
        if first.next.is_null(){
            self.first = null_mut();
            self.last = null_mut();
            return first;
        } else{
            self.first = first.next;
            set_prev(self.first, null_mut());
            first.next = null_mut();
            return first;
        }
    }

    fn pull(&mut self, id : u64) -> *mut MutNode<V>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return null_mut(); };
        if ptr_eq(node, self.first){ return self.pull_first(); }
        if ptr_eq(node, self.last){ return self.pull_last(); }
        //firstでもlastでもないということは、中間ということ
        let prev = get_prev(node);
        let next = get_next(node);
        set_next(prev, next);
        set_prev(next, prev);
        set_next(node, null_mut());
        set_prev(node, null_mut());
        return node;
    }

    fn put_first(&mut self, node : *mut MutNode<V>){
        if self.first.is_null(){
            self.first = node;
            self.last = node;
        } else{
            if ptr_eq(self.first, node){ panic!() }

            let next = self.first;
            self.first = node;
            set_next(self.first, next);
            set_prev(next, self.first);
        }
    }

    fn put_last(&mut self, node : *mut MutNode<V>){
        if self.last.is_null(){
            self.first = node;
            self.last = node;
        } else{
            if ptr_eq(self.last, node){ panic!() }

            let prev = self.last;
            self.last = node;
            set_next(prev, self.last);
            set_prev(self.last, prev);
        }
    }

    fn put_next(&mut self, prev : *mut MutNode<V>, node : *mut MutNode<V>){
        if ptr_eq(prev, node){ panic!() }
        if self.last.is_null(){ panic!() }

        if ptr_eq(prev, self.last){
            return self.put_last(node);
        }
        let next = get_next(prev);
        set_next(prev, node);
        set_prev(node, prev);
        set_next(node, next);
        set_prev(next, node);
    }

    fn put_prev(&mut self, next : *mut MutNode<V>, node : *mut MutNode<V>){
        if ptr_eq(next, node){ panic!() }
        if self.first.is_null(){ panic!() }

        if ptr_eq(next, self.first){
            return self.put_first(node);
        }

        let prev = get_prev(next);
        set_next(prev, node);
        set_prev(node, prev);
        set_next(node, next);
        set_prev(next, node);
    }

    pub fn insert(&mut self, val : V) -> u64{
        self.insert_last(val)
    }

    pub fn insert_last(&mut self, val : V) -> u64{
        let mut node = Box::new(MutNode::new(val, self.next_id));
        self.put_last(node.as_mut());
        self.map.insert(self.next_id, node);
        self.next_id += 1;
        return self.next_id - 1;
    }
    pub fn insert_first(&mut self, val : V) -> u64{
        let mut node = Box::new(MutNode::new(val, self.next_id));
        self.put_first(node.as_mut());
        self.map.insert(self.next_id, node);
        self.next_id += 1;
        return self.next_id - 1;
    }

    pub fn remove(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.map.remove(&id);
        return true;
    }
    pub fn remove_first(&mut self) -> bool{
        let node = self.pull_first();
        if node.is_null(){ return false; }
        self.map.remove(&get_id(node));
        return true;
    }

    pub fn remove_last(&mut self) -> bool{
        let node = self.pull_last();
        if node.is_null(){ return false; }
        self.map.remove(&get_id(node));
        return true;
    }

    pub fn to_first(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_first(node);
        return true;
    }

    pub fn to_last(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_last(node);
        return true;
    }

    pub fn to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
        if id == next_items_id{ return false; }
        let next = if let Some(node) = self.map.get_mut(&next_items_id).map(|b| b.as_mut() as *mut MutNode<V>){ node } else{ return false };
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_prev(next, node);
        return true;
    }

    pub fn to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
        if id == prev_items_id{ return false; }
        let prev = if let Some(node) = self.map.get_mut(&prev_items_id).map(|b| b.as_mut() as *mut MutNode<V>){ node } else{ return false };
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_next(prev, node);
        return true;
    }

    pub fn iter(&self) -> LinkedMapIter<V> { LinkedMapIter::new(self) }
    pub fn iter_from_last(&self) -> LinkedMapIter<V>{ LinkedMapIter{ map : self, node : self.last }}
    pub fn iter_from_id(&self, id : u64) -> Option<LinkedMapIter<V>>{
        self.node_from_id(id).map(|node| LinkedMapIter{ map : self, node : node as *const _ as *mut _})
    }
}



pub struct LinkedMapIter<'a, V>{
    map : &'a LinkedMap<V>,
    node : *mut MutNode<V>
}
impl<'a, V> LinkedMapIter<'a, V>{
    pub fn new(map : &'a LinkedMap<V>) -> LinkedMapIter<'a, V>{ LinkedMapIter{ map, node : map.first } }

    pub fn next(&mut self) -> Option<(&'a u64, &'a V)> {
        if self.node.is_null() { return None; }
        let current_node = self.node;
        if ptr_eq(self.node, self.map.last) {
            self.node = null_mut();
        } else {
            self.node = get_next(self.node);
        }
        let node = unsafe { current_node.as_mut().unwrap() };
        return Some((&node.id, get_item(current_node)));
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、再びnextでとっていくことも出来る。今ある場所をremoveしたらポインタが不正に
    pub fn prev(&mut self) -> Option<(&'a u64, &'a V)> {
        if self.node.is_null(){ return None; }
        let current_node = self.node;
        if ptr_eq(self.node, self.map.first){
            self.node = null_mut();
        } else {
            self.node = get_prev(self.node);
        }
        let node = unsafe{ current_node.as_mut().unwrap() };
        return Some((&node.id, get_item(current_node)))
    }
}

impl<'a,V> Iterator for LinkedMapIter<'a, V>{
    type Item = (&'a u64, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a,V> IntoIterator for &'a LinkedMap<V>{
    type Item = (&'a u64, &'a V);
    type IntoIter = LinkedMapIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// 基本的にはLinkedHashMapであるが、indexを用いて getしたときにノードがキャッシュされる。
/// そのため C言語の for文で i++ i--のように順繰りにアクセスした場合、キャッシュしたノードの次、前、という形で探せるので
/// O(N)でなくO(1)に出来る。
/// つまりCからアクセスするときの利便性と引き換えに、ちょっとしたオーバーヘッドが発生し、Syncを失っている。
/// 俺はこのライブラリは実質Cライブラリだと思ってるのでこれで良いだろう。
#[derive(Debug)]
pub struct ListMap<V>{
    map : LinkedMap<V>,
    cache : RefCell<Option<Cache<V>>>
}
#[derive(Debug, Clone)]
struct Cache<V>{
    index : usize,
    ptr : *mut MutNode<V>
}
impl<V> ListMap<V> {
    pub fn new() -> ListMap<V> {
        ListMap { map: LinkedMap::new(), cache : RefCell::new(None) }
    }

    fn delete_cache(&self){
        self.cache.replace(None);
    }

    pub fn first(&self) -> &V { self.map.first() }
    pub fn first_mut(&mut self) -> &mut V { self.map.first_mut() }
    pub fn first_id(&self) -> u64 { self.map.first_id() }
    pub fn last(&self) -> &V { self.map.last() }
    pub fn last_mut(&mut self) -> &mut V { self.map.last_mut() }
    pub fn last_id(&self) -> u64 { self.map.last_id() }

    pub fn from_id(&self, id: u64) -> Option<&V> { self.map.from_id(id) }
    pub fn from_id_mut(&mut self, id: u64) -> Option<&mut V> { self.map.from_id_mut(id) }

    pub fn contains_key(&self, key: u64) -> bool { self.map.contains_key(key) }
    pub fn len(&self) -> usize { self.map.len() }

    pub fn insert(&mut self, val: V) -> u64 {
        self.delete_cache();
        self.map.insert(val)
    }

    pub fn insert_last(&mut self, val: V) -> u64 {
        self.delete_cache();
        self.map.insert_last(val)
    }
    pub fn insert_first(&mut self, val: V) -> u64 {
        self.delete_cache();
        self.map.insert_first(val)
    }

    pub fn remove(&mut self, id: u64) -> bool {
        self.delete_cache();
        self.map.remove(id)
    }
    pub fn remove_first(&mut self) -> bool {
        self.delete_cache();
        self.map.remove_first()
    }

    pub fn remove_last(&mut self) -> bool {
        self.delete_cache();
        self.map.remove_last()
    }

    pub fn to_first(&mut self, id: u64) -> bool {
        self.delete_cache();
        self.map.to_first(id)
    }

    pub fn to_last(&mut self, id: u64) -> bool {
        self.delete_cache();
        self.map.to_last(id)
    }

    pub fn to_prev(&mut self, next_items_id: u64, id: u64) -> bool {
        self.delete_cache();
        self.map.to_prev(next_items_id, id)
    }

    pub fn to_next(&mut self, prev_items_id: u64, id: u64) -> bool {
        self.delete_cache();
        self.map.to_next(prev_items_id, id)
    }

    pub fn iter(&self) -> LinkedMapIter<V> { self.map.iter() }

    pub fn index_mut(&mut self, index : usize) -> &mut V{
        let node = self.index_impl(index);
        self.cache.replace(Some(Cache{ index, ptr : node }));
        unsafe{ &mut node.as_mut().unwrap().item }
    }

    pub fn index(&self, index : usize) -> &V{
        let node = self.index_impl(index);
        self.cache.replace(Some(Cache{ index, ptr : node }));
        unsafe{ &node.as_ref().unwrap().item }
    }

    fn index_impl(&self, index : usize) -> *mut MutNode<V>{
        if self.len() <= index{ panic!("index out of bound") }
        let index = index as i64;
        let from_first = index as i64;
        let from_last = self.len() as i64 - 1 - index;
        let cache = self.cache.borrow();
        if cache.is_none() {
            if from_first < from_last {
                self.from_first(index as usize)
            } else{
                self.from_last(index as usize)
            }
        } else{
            let cache = cache.as_ref().unwrap();
            let diff = (index - cache.index as i64).abs();
            if diff < from_first && diff < from_last{
                self.get_node(index as usize, cache.ptr, cache.index)
            } else{
                if from_first < from_last {
                    self.from_first(index as usize)
                } else{
                    self.from_last(index as usize)
                }
            }
        }
    }

    fn from_first(&self, index : usize) -> *mut MutNode<V>{
        return self.get_node(index, self.map.first, 0);
    }

    fn from_last(&self, index : usize) -> *mut MutNode<V>{
        return self.get_node(index, self.map.last, self.len() - 1);
    }

    fn get_node(&self, index : usize, node : *mut MutNode<V>, node_index : usize) -> *mut MutNode<V>{
        let mut current = node_index;
        let mut node = node;
        if node_index <= index{
            loop{
                if index == current {
                    return node;
                }
                node = get_next(node);
                current += 1;
            }
        } else{
            loop{
                if index == current {
                    return node;
                }
                node = get_prev(node);
                current -= 1;
            }
        }

    }
}