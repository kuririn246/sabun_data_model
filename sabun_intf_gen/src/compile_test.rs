//
// use sabun_maker::intf::*;
// use sabun_maker::structs::*;
//
// pub struct RootIntf{
//     obj : Box<RootObject>,
//     intf : Box<RootItem>,
// }
// impl RootIntf{
//     pub fn new(obj : RootObject) -> RootIntf{
//         let mut b = Box::new(obj);
//         let intf = RootItem::new(RootObjectPtr::new(b.as_mut()));
//         RootIntf{ obj : b, intf : Box::new(intf) }
//     }
//     pub fn intf(&mut self) -> &mut RootItem{ &mut self.intf }
//     pub fn deconstruct(self) -> (Box<RootObject>, Box<RootItem>){ (self.obj, self.intf) }
// }
//
// #[derive(Debug, PartialEq)]
// pub struct RootItem {
//     pub ptr : RootObjectPtr,
//     pub root : *const RootItem,
//     p_bu : Option<bool>,
//     p_refed_data : Option<RefedData>,
//     p_col_data : Option<ColData>,
// }
// impl RootItem {
//     pub fn new(ptr : RootObjectPtr) -> RootItem{
//         RootItem{ ptr, p_bu : None, p_refed_data : None, p_col_data : None, }
//     }
//     pub fn bu(&mut self) -> bool{
//         if let Some(v) = &self.p_bu{
//             return v.clone();
//         }
//         let qv = root::get_bool(self.ptr, "bu").unwrap();
//         let ans = qv.into_value().unwrap();
//         self.p_bu = Some(ans);
//         return self.p_bu.as_ref().unwrap();
//     }
//     pub fn set_bu(&mut self, bu : bool){
//         self.p_bu = Some(bu);
//         root::set_bool(self.ptr, "bu", Qv::Val(bu));
//     }
//     pub fn refed_data(&mut self) -> &RefedData{
//         if let Some(v) = &self.p_refed_data{
//             return v;
//         }
//         let ans = root::get_data(self.ptr, "refed").unwrap();
//         self.p_refed_data = Some(RefedData::new(ans, self.root));
//         return self.p_refed_data.as_ref().unwrap();
//     }
//     pub fn col_data(&mut self) -> &ColData{
//         if let Some(v) = &self.p_col_data{
//             return v;
//         }
//         let ans = root::get_data(self.ptr, "col").unwrap();
//         self.p_col_data = Some(ColData::new(ans, self.root));
//         return self.p_col_data.as_ref().unwrap();
//     }
// }
//
// #[derive(Debug, PartialEq)]
// pub struct RefedData {
//     pub ptr : ConstDataPtr,
//     root : *const RootItem,
//     p_first : *const FirstItem,
//     p_second : *const SecondItem,
// }
// impl RefedData {
//     pub fn new(ptr : ConstDataPtr, root : *const RootItem) -> RefedData{ RefedData{ ptr, root, p_first : None, p_second : None, } }
//     pub fn first(&mut self) -> &RefedItem {
//         if let Some(item) = &self.p_first {
//             return item;
//         }
//         let ptr = data::get_value(self.ptr, "first").unwrap();
//         let item = RefedItem::new(ptr, root);
//         self.p_RefedItem = Some(item);
//         self.p_RefedItem.as_ref().unwrap()
//     }
//     pub fn second(&mut self) -> &RefedItem {
//         if let Some(item) = &self.p_second {
//             return item;
//         }
//         let ptr = data::get_value(self.ptr, "second").unwrap();
//         let item = RefedItem::new(ptr, root);
//         self.p_RefedItem = Some(item);
//         self.p_RefedItem.as_ref().unwrap()
//     }
//     pub fn from_id(&mut self, id : &str) -> &RefedItem {
//         match id{
//             "first" => self.first(),
//             "second" => self.second(),
//         }
//     }
//
//     #[derive(Debug, PartialEq)]
//     pub struct RefedItem {
//         pub ptr : ListItemPtr,
//         pub root : *const RootItem,
//         p_mem : Option<f64>,
//     }
//     impl RefedItem {
//         pub fn new(ptr : ListItemPtr, root : *const RootItem) -> RefedItem{
//             RefedItem{ ptr, root, p_mem : None, }
//         }
//         pub fn mem(&mut self) -> f64{
//             if let Some(v) = &self.p_mem{
//                 return v.clone();
//             }
//             let qv = list_item::get_num(self.ptr, "mem").unwrap();
//             let ans = qv.into_value().unwrap();
//             self.p_mem = Some(ans);
//             return self.p_mem.as_ref().unwrap();
//         }
//     }
//
//     #[derive(Debug, PartialEq)]
//     pub struct ColData {
//         pub ptr : ConstDataPtr,
//         root : *const RootItem,
//         p_huga : *const HugaItem,
//     }
//     impl ColData {
//         pub fn new(ptr : ConstDataPtr, root : *const RootItem) -> ColData{ ColData{ ptr, root, p_huga : None, } }
//         pub fn huga(&mut self) -> &ColItem {
//             if let Some(item) = &self.p_huga {
//                 return item;
//             }
//             let ptr = data::get_value(self.ptr, "huga").unwrap();
//             let item = ColItem::new(ptr, root);
//             self.p_ColItem = Some(item);
//             self.p_ColItem.as_ref().unwrap()
//         }
//         pub fn from_id(&mut self, id : &str) -> &ColItem {
//             match id{
//                 "huga" => self.huga(),
//             }
//         }
//
//         #[derive(Debug, PartialEq)]
//         pub struct ColItem {
//             pub ptr : ListItemPtr,
//             pub root : *const RootItem,
//             p_nakabu : Option<bool>,
//             ref_refed : Option<RefedItem>,
//         }
//         impl ColItem {
//             pub fn new(ptr : ListItemPtr, root : *const RootItem) -> ColItem{
//                 ColItem{ ptr, root, p_nakabu : None, ref_refed : None, }
//             }
//             pub fn nakabu(&mut self) -> bool{
//                 if let Some(v) = &self.p_nakabu{
//                     return v.clone();
//                 }
//                 let qv = list_item::get_bool(self.ptr, "nakabu").unwrap();
//                 let ans = qv.into_value().unwrap();
//                 self.p_nakabu = Some(ans);
//                 return self.p_nakabu.as_ref().unwrap();
//             }
//             pub fn ref_refed(&mut self) -> &RefedItem{
//                 if let Some(v) = &self.ref_refed{
//                     return unsafe{ v.as_ref().unwrap() };
//                 }
//                 let qv = list_item::get_ref(self.ptr, "refed").unwrap();
//                 let ref_id = match qv{
//                     Qv::None =>{ return ::Null; },		Qv::Undefined =>{ return ::Undefined; },		Qv::Val(id) =>id,};	let root = unsafe{ self.root.as_ref().unwrap() };	let ref_ptr : *const refed = root.RefedItem().from_id(ref_id);	self.ref_refed = Some(ref_ptr);
//                 let pp = self.ref_refed.as_ref().unwrap();
//                 unsafe{ (*pp).as_ref().unwrap() }
//             }
//         }
//
//
