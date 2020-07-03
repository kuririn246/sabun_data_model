// #[cfg(test)]
// mod tests {
//
//     struct Item{
//         pub x : usize,
//     }
//
//     struct Hoge{
//         pub root : Root,
//         pub ptr : RootPtr,
//         pub item : Option<Item>,
//     }
//
//     struct Root{
//         pub x : usize,
//     }
//
//     struct RootPtr{
//         pub ptr : *const Root,
//     }
//     impl RootPtr{
//         pub fn inner_x(&self) -> usize{
//             let p = unsafe{ self.ptr.as_ref().unwrap() };
//             p.x
//         }
//     }
//
//     impl Hoge{
//
//         pub fn new() -> Hoge {
//             let root = Root { x: 2 };
//             let ptr = RootPtr { ptr: &root };
//             let item = Item{ x : 4 };
//             let hoge = Hoge { root, ptr, item : Some(item) };
//             hoge
//         }
//         pub fn root(&self) -> &Root{ &self.root }
//         pub fn ptr(&mut self) -> &mut RootPtr{ &mut self.ptr }
//         pub fn item(&mut self) -> &mut Item {
//             if self.item.is_some(){
//                 return self.item.as_mut().unwrap()
//             } else{
//                 return anoth2(self)
//             }
//
//         }
//
//
//     }
//
//     fn anoth2(hoge : &mut Hoge) -> &mut Item{
//         hoge.item = Some(Item{ x : 10 });
//         hoge.item.as_mut().unwrap()
//     }
//
//     #[test]
//     fn it_works() {
//         let mut hoge = Hoge::new();
//         let ptr = hoge.ptr();
//         println!("inner_x {}", ptr.inner_x());
//         println!("x {}", hoge.root.x);
//
//     }
// }