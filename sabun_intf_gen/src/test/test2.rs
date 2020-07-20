#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    struct Item{
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
    struct List{
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

    struct Root{
        list : List
    }
    impl Root{
        pub fn new() -> Root{
            Root{ list : List::new() }
        }
    }

    struct RootNormalIntf{
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

    struct ListNormalIntf<'a>{
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
    struct ItemNormalIntf<'a>{
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

    struct ListMagicIntf{
        list : *mut List,
    }
    impl ListMagicIntf{
        pub fn new(list : &mut List) -> ListMagicIntf{
            ListMagicIntf{ list }
        }
        pub fn get(&self, index : usize) -> ItemMagicIntf{

        }
    }
    struct ItemMagicIntf{
        item : *mut Item,
    }
    impl ItemMagicIntf{
        
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