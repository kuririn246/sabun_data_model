//#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    pub struct Item{
        pub map : HashMap<String, String>,
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

    #[repr(C)]
    pub struct ListMagicIntf{
        list : *const List,
    }
    impl ListMagicIntf{
        pub fn new(list : *const List) -> ListMagicIntf{
            ListMagicIntf{ list }
        }
        pub fn get(&self, index : usize) -> ItemMagicIntf{
            let list = unsafe{ self.list.as_ref().unwrap() };
            ItemMagicIntf::new(list.get(index))
        }
    }
    #[allow(non_snake_case)]
    pub extern "C" fn ListMagicIntf_get(list : *const ListMagicIntf, index : usize) -> ItemMagicIntf{
        let list = unsafe{ list.as_ref().unwrap() };
        list.get(index)
    }
    #[repr(C)]
    pub struct ItemMagicIntf{
        item : *const Item,
    }
    impl ItemMagicIntf{
        pub fn new(item : *const Item) -> ItemMagicIntf{ ItemMagicIntf{ item }}
        pub fn param1(&self) -> &String{
            let item = unsafe{ &*self.item };
            item.get("param1").unwrap()
        }
        pub fn set_param1(&self, s : String){
            let item = unsafe{ &mut *(self.item as *mut Item) };
            item.map.insert("param1".to_string(), s);
        }
    }
    #[allow(non_snake_case)]
    pub extern "C" fn ItemMagicIntf_param1<'a>(item : *const ItemMagicIntf) -> *const String{
        let item = unsafe{ &*item };
        item.param1()
    }
    #[allow(non_snake_case)]
    pub extern "C" fn ItemMagicIntf_set_param1(item : *const ItemMagicIntf, s : String){
        let item = unsafe{ &mut *(item as *mut ItemMagicIntf) };
        item.set_param1(s)
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
        println!("magic param1 {}", unsafe{ &*hoge })

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