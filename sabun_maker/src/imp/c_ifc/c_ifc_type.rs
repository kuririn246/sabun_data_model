#[repr(u64)]
#[derive(Debug,Clone,Copy)]
pub enum CifcType {
    Root = 0,
    Param = 1,
    Data = 2,
    List = 3,
    Mut = 4,
}