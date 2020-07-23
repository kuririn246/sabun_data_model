//#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::collections::HashMap;
    use sabun_maker::intf::RustStrPtr;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    pub struct Item{
        pub map : HashMap<String, String> ,
    }
    impl Item{
        pub fn new() -> Item{
            let mut map : HashMap<String, String> = HashMap::new();
            map.insert("param1".to_string(), "value_h".to_string());
            Item{ map }
        }
        pub fn get(&self, key : &str) -> Option<&String>{
            self.map.get(key)
        }
    }
    pub struct List{
        pub vec : Vec<Item>
    }
    impl List{
        pub fn new() -> List{
            List{ vec : vec![Item::new()] }
        }
        pub fn get(&self, index : usize) -> &Item{
            self.vec.get(index).unwrap()
        }
    }

    pub struct Root{
        list : List
    }
    impl Root{
        pub fn new() -> Root{
            Root{ list : List::new() }
        }
    }

    pub struct RootNormalIntf{
        pub root : Root
    }
    impl RootNormalIntf{
        pub fn new(root : Root) -> RootNormalIntf{
            RootNormalIntf{ root }
        }
        pub fn list(&self) -> ListNormalIntf{
            ListNormalIntf{ list : &self.root.list}
        }
    }

    pub struct ListNormalIntf<'a>{
        pub list : &'a List
    }
    impl ListNormalIntf<'_>{
        pub fn new(list : &List) -> ListNormalIntf{
            ListNormalIntf{ list }
        }
        pub fn get(&self, index : usize) -> ItemNormalIntf{
            ItemNormalIntf::new(self.list.get(index))
        }
    }
    pub struct ItemNormalIntf<'a>{
        pub item : &'a Item
    }
    impl ItemNormalIntf<'_>{
        pub fn new(item : &Item) -> ItemNormalIntf{
            ItemNormalIntf{ item }
        }
        pub fn param1(&self) -> &String{
            self.item.get("param1").unwrap()
        }
    }

    #[repr(C)]
    pub struct RootMagicIntf{
        pub root : Root
    }
    impl RootMagicIntf{
        pub fn new(root : Root) -> RootMagicIntf{
            RootMagicIntf{ root }
        }
        pub fn list(&self) -> ListMagicIntf{
            ListMagicIntf::new(&self.root.list)
        }
    }

    #[repr(C)] #[derive(Debug, Clone, Copy)]
    pub struct ListMagicIntf{
        list : *const List,
    }
    impl ListMagicIntf{
        pub fn new(list : *const List) -> ListMagicIntf{
            ListMagicIntf{ list }
        }
        pub fn get(&self, index : usize) -> ItemMagicIntf{
            let list = unsafe{ &*self.list };
            ItemMagicIntf::new(list.get(index))
        }
        pub fn len(&self) -> usize{
            let list = unsafe{ &*self.list };
            list.vec.len()
        }
        pub fn iter(&self) -> ListMagicIntfIter{
            ListMagicIntfIter{ intf : self.clone(), counter : 0 }
        }
        pub fn general_iter(&self) -> GeneralIter<ListMagicIntf, ItemMagicIntf>{
            GeneralIter{ counter : 0, len : self.len(), intf : self.clone(), getter : ListMagicIntf::get }
        }
    }
    #[allow(non_snake_case)] #[no_mangle]
    pub extern "C" fn ListMagicIntf_get(list : ListMagicIntf, index : usize) -> ItemMagicIntf{
        list.get(index)
    }
    #[allow(non_snake_case)] #[no_mangle]
    pub extern "C" fn ListMagicIntf_len(list : ListMagicIntf) -> usize{
        list.len()
    }
   #[derive(Debug, Clone, Copy)]
    pub struct ListMagicIntfIter{
        intf : ListMagicIntf,
        counter : usize,
    }
    impl Iterator for ListMagicIntfIter{
        type Item = ItemMagicIntf;

        fn next(&mut self) -> Option<Self::Item> {
            let len = self.intf.len();
            if self.counter < len{
                let counter = self.counter;
                self.counter += 1;
                Some(self.intf.get(counter))
            } else{
                None
            }
        }
    }

    pub struct GeneralIter<T,U>{
        len : usize,
        counter : usize,
        intf : T,
        getter : fn(&T,usize) -> U,
    }

    impl<T,U> Iterator for GeneralIter<T,U>{
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            if self.counter < self.len{
                let counter = self.counter;
                self.counter += 1;
                Some((self.getter)(&self.intf, counter))
            } else{
                None
            }
        }
    }

    #[repr(C)]
    pub struct ItemMagicIntf{
        item : *const Item,
    }
    impl ItemMagicIntf{
        pub fn new(item : *const Item) -> ItemMagicIntf{ ItemMagicIntf{ item }}
        pub fn param1(&self) -> String{
            let item = unsafe{ &*self.item };
            item.get("param1").unwrap().clone()
        }
        pub fn set_param1(&self, s : String){
            let item = unsafe{ &mut *(self.item as *mut Item) };
            item.map.insert("param1".to_string(), s);
        }
    }
    #[allow(non_snake_case)]
    pub extern "C" fn ItemMagicIntf_param1(item : *const ItemMagicIntf) -> RustStrPtr{
        let item = unsafe{ &*(*item).item };
        RustStrPtr::new(item.get("param1").unwrap())
    }
    #[allow(non_snake_case)]
    pub extern "C" fn ItemMagicIntf_set_param1(item : *const ItemMagicIntf, s : *const c_char){
        let item = unsafe{ &mut *(item as *mut ItemMagicIntf) };
        let s = unsafe{ CStr::from_ptr(s) };
        item.set_param1(s.to_str().unwrap().to_string())
    }


    // fn test_lifetime<'a>(n : &Vec<usize>) -> &'a usize{
    //     n.get(0).unwrap()
    // }


    #[test]
    fn it_works_magic() {



        let hoge = {
            let root = Root::new();
            let intf = RootMagicIntf::new(root);
            let list = intf.list();
            let item = list.get(0);
            println!("magic param1 {}", item.param1());
            item.set_param1("set param1".to_string());
            let p1 = ItemMagicIntf_param1(&item);
            p1

        };
        println!("magic param1 {}", hoge.to_string())

    }

    #[test]
    fn it_works() {
        let root = Root::new();
        let intf = RootNormalIntf::new(root);
        let list = intf.list();
        let item = list.get(0);
        println!("param1 {}", item.param1());

    }
}