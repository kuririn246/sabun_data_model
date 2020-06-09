#[repr(u64)]
#[derive(Debug,Clone,Copy)]
pub enum CifcType {
    Root = 0,
    ParamBool = 1,
    ParamNum = 2,
    ParamStr = 3,
    ParamNumArray = 4,
    ParamStrArray = 5,
    ParamNum2Array = 6,
    Data = 7,
    List = 8,
    Mut = 9,
}