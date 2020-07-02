// #[cfg(test)]
// mod tests {
//     struct Hoge{
//         pub root : Root,
//         pub ptr : RootPtr,
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
//             let hoge = Hoge { root, ptr };
//             hoge
//         }
//         pub fn root(&self) -> &Root{ &self.root }
//         pub fn ptr(&mut self) -> &mut RootPtr{ &mut self.ptr }
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