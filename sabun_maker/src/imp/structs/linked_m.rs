use crate::{HashM, HashMt};
use std::ptr::{null_mut};
use std::marker::PhantomData;

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
    fn new(id : u64, item : V, dummy : u64) -> MutNode<V>{
        MutNode{ item, prev : null_mut(), next : null_mut(), id }
    }
}
fn get_next<V>(this : *const MutNode<V>) ->*mut MutNode<V>{
    let node = unsafe{ &*this };
    node.next
}
fn set_next<V>(this : *mut MutNode<V>, next : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.next = next;
}
fn get_prev<V>(this : *const MutNode<V>) -> *mut MutNode<V>{
    let node = unsafe{ &*this };
    node.prev
}
fn set_prev<V>(this : *mut MutNode<V>, prev : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.prev = prev;
}
fn get_id<V>(this : *const MutNode<V>) -> u64{
    let node = unsafe{ &*this };
    node.id
}
fn get_item<'a, V>(this : *const MutNode<V>) -> &'a V{
    let node = unsafe{ &*this };
    &node.item
}
fn get_item_mut<'a, V>(this : *mut MutNode<V>) ->&'a mut V{
    let node = unsafe{ &mut *this };
    &mut node.item
}

fn ptr_eq<T>(l : *const T, r : *const T) -> bool{ std::ptr::eq(l,r) }

impl<V> LinkedMap<V> {

    pub fn new() -> LinkedMap<V> {
        LinkedMap { map : HashMt::new(), first : null_mut(), last : null_mut(), next_id : 0, }
    }
    pub fn construct(items : Vec<(u64,V)>, next_id : u64) -> LinkedMap<V>{
        if items.is_empty(){
            return LinkedMap::new();
        }
        let mut map : HashM<u64,Box<MutNode<V>>> = HashMt::with_capacity(items.len());
        let mut iter = items.into_iter();
        let (id,val) = iter.next().unwrap();
        let mut first_node  = Box::new(MutNode::new(id, val, id));
        let first_node_ptr = first_node.as_mut() as *mut _;
        let mut prev_node_ptr = first_node_ptr;
        map.insert(id, first_node);
        for (id,val) in items{
            let mut node = Box::new(MutNode::new( id, val, id));
            set_next(prev_node_ptr, node.as_mut());
            node.as_mut().prev = prev_node_ptr;
            prev_node_ptr = node.as_mut();
            map.insert(id, node);
        }
        LinkedMap{ map, first : first_node_ptr, last : prev_node_ptr, next_id }
    }
    pub fn next_id(&self) -> u64{ self.next_id }

    pub fn first(&self) -> Option<&V> {
        if self.first.is_null() { None } else { Some(get_item(self.first)) }
    }
    pub fn first_mut(&mut self) -> Option<&mut V> {
        if self.first.is_null() { None } else { Some(get_item_mut(self.first)) }
    }
    pub fn first_id(&self) -> Option<u64> {
        if self.first.is_null() { None } else { Some(get_id(self.first)) }
    }
    pub fn last(&self) -> Option<&V> {
        if self.last.is_null() { None } else { Some(get_item(self.last)) }
    }
    pub fn last_mut(&mut self) -> Option<&mut V> {
        if self.last.is_null() { None } else { Some(get_item_mut(self.last)) }
    }
    pub fn last_id(&self) -> Option<u64> {
        if self.last.is_null() { None } else { Some(get_id(self.last)) }
    }

    fn node_from_id_mut(&mut self, id : u64) -> Option<&mut MutNode<V>>{ self.map.get_mut(&id).map(|b| b.as_mut()) }
    fn node_from_id(&self, id : u64) -> Option<&MutNode<V>>{ self.map.get(&id).map(|b| b.as_ref()) }

    pub fn from_id(&self, id : u64) -> Option<&V>{ self.map.get(&id).map(|b| &b.as_ref().item) }
    pub fn from_id_mut(&mut self, id : u64) -> Option<&mut V>{ self.map.get_mut(&id).map(|b| &mut b.as_mut().item) }

    pub fn contains_key(&self, key : u64) -> bool{ self.map.contains_key(&key) }
    pub fn len(&self) -> usize{ self.map.len() }
    pub fn is_empty(&self) -> bool{ self.map.is_empty() }

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
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut _ } else{ return null_mut(); };
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

    pub fn iter(&self) -> LinkedMapIter<V> { LinkedMapIter::new(self, self.first) }
    pub fn iter_from_last(&self) -> LinkedMapIter<V>{ LinkedMapIter::new(self, self.last)}
    pub fn iter_from_id(&self, id : u64) -> Option<LinkedMapIter<V>>{
        self.node_from_id(id).map(|node| LinkedMapIter::new(self, node))
    }
    pub fn iter_mut(&mut self) -> LinkedMapIterMut<V> { LinkedMapIterMut::new(self, self.first) }
    pub fn iter_mut_from_last(&mut self) -> LinkedMapIterMut<V>{ LinkedMapIterMut::new(self, self.last)}
    pub fn iter_mut_from_id(&mut self, id : u64) -> Option<LinkedMapIterMut<V>>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return None; };
        Some(LinkedMapIterMut::new(self, node))
    }

    pub unsafe fn iter_unsafe(&mut self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.first) }
    pub unsafe fn iter_from_last_unsafe(&mut self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.last) }
    pub unsafe fn iter_from_id_unsafe(&mut self, id : u64) -> Option<LinkedMapUnsafeIter<V>>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return None; };
        Some(LinkedMapUnsafeIter::new(self, node))

    }
}


pub struct LinkedMapUnsafeIter<V>{
    map : *mut LinkedMap<V>,
    node : *mut MutNode<V>,
}
impl<V> LinkedMapUnsafeIter<V>{
    fn new(map : *mut LinkedMap<V>, node : *mut MutNode<V>) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter{ map, node } }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        self.next_mut().map(|(k,v)| (k,&*v))
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        if self.node.is_null() { return None; }
        let current_node = self.node as *mut MutNode<V>;
        let map = unsafe{ self.map.as_ref().unwrap() };
        if ptr_eq(self.node, map.last) {
            self.node = null_mut();
        } else {
            self.node = get_next(self.node);
        }
        let node = unsafe { current_node.as_mut().unwrap() };
        return Some((&node.id, &mut node.item));
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        if self.node.is_null(){ return None; }
        let current_node = self.node as *mut MutNode<V>;
        let map = unsafe{ self.map.as_ref().unwrap() };
        if ptr_eq(self.node, map.first){
            self.node = null_mut();
        } else {
            self.node = get_prev(self.node);
        }
        let node = unsafe{ current_node.as_mut().unwrap() };
        return Some((&node.id, &mut node.item))
    }

    pub fn current_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        if self.node.is_null(){ return None; }
        let node = unsafe{ &mut *self.node };
        return Some((&node.id, &mut node.item))
    }

    pub fn current<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        return self.current_mut().map(|(k,v)| (k,&*v))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        self.prev_mut().map(|(k,v)| (k,&*v))
    }

    ///nextもprevも現在のカーソルにあるアイテムを返す
    ///空であるか、最後/最初まで移動してアイテムが無くなったらfalse
    pub fn is_available(&self) -> bool {
        !self.node.is_null()
    }
}


pub struct LinkedMapIter<'a, V>{
    iter : LinkedMapUnsafeIter<V>,
    phantom : PhantomData<&'a LinkedMap<V>>,
}
impl<'a, V> LinkedMapIter<'a, V>{
    fn new(map : &'a LinkedMap<V>, node : *const MutNode<V>) -> LinkedMapIter<'a, V>{
        //LinkedMapIterが有効な間に書き換えるとアウトだが、&が有効なはずなので大丈夫だろう
        LinkedMapIter{ iter : LinkedMapUnsafeIter{ map : map as *const _ as *mut _, node : node as *mut _ }, phantom : PhantomData::default() }
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.next()
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。Cインターフェースやunsafe iterなら
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.prev()
    }

    pub fn current(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.current()
    }

    ///nextもprevも現在のカーソルにあるアイテムを返す
    ///空であるか、最後/最初まで移動してアイテムが無くなったらfalse
    pub fn is_available(&self) -> bool {
        self.iter.is_available()
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

pub struct LinkedMapIterMut<'a, V>{
    iter : LinkedMapUnsafeIter<V>,
    phantom : PhantomData<&'a LinkedMap<V>>,
}
impl<'a, V> LinkedMapIterMut<'a, V>{
    fn new(map : &'a mut LinkedMap<V>, node : *mut MutNode<V>) -> LinkedMapIterMut<'a, V>{
        LinkedMapIterMut{ iter : LinkedMapUnsafeIter::new(map, node), phantom : PhantomData::default() }
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(&'a u64, &'a mut V)> {
        self.iter.next_mut()
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。Cインターフェースやunsafe iterなら
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(&'a u64, &'a mut V)> {
        self.iter.prev_mut()
    }

    pub fn current(&mut self) -> Option<(&'a u64, &'a mut V)> {
        self.iter.current_mut()
    }

    ///nextもprevも現在のカーソルにあるアイテムを返す
    ///空であるか、最後/最初まで移動してアイテムが無くなったらfalse
    pub fn is_available(&self) -> bool {
        self.iter.is_available()
    }
}
impl<'a,V> Iterator for LinkedMapIterMut<'a, V>{
    type Item = (&'a u64, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a,V> IntoIterator for &'a mut LinkedMap<V>{
    type Item = (&'a u64, &'a mut V);
    type IntoIter = LinkedMapIterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct LinkedMapIntoIter<V>{
    iter : LinkedMapUnsafeIter<V>,
    phantom : PhantomData<&'a LinkedMap<V>>,
}