use crate::structs::qv::QvType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType{
    Normal,
    Nullable,
    Undefinable,
    UndefNullable,
}

impl ValueType{
    pub fn is_nullable(&self) -> bool{
        match self{
            ValueType::Nullable | ValueType::UndefNullable => true,
            _ => false,
        }
    }

    pub fn is_undefable(&self) -> bool{
        match self{
            ValueType::Undefinable | ValueType::UndefNullable => true,
            _ => false,
        }
    }

    pub fn to_suffix(&self) -> String{
        let s = match self{
            ValueType::Normal => "",
            ValueType::Nullable => "?",
            ValueType::Undefinable => "!",
            ValueType::UndefNullable => "!?",
        };
        s.to_string()
    }


    // pub(crate) fn _type_num(&self) -> usize{
    //     match self{
    //         ValueType::Normal => 0,
    //         ValueType::Nullable => 1,
    //         ValueType::Undefinable => 2,
    //         ValueType::UndefNullable => 3,
    //     }
    // }

    pub fn acceptable(&self, t : &QvType) -> bool {
        match self{
            ValueType::Normal => {
                match t {
                    QvType::Val => true,
                    _ => false,
                }
            },
            ValueType::Nullable => {
                match t {
                    QvType::Val | QvType::Null => true,
                    _ => false,
                }
            },
            ValueType::Undefinable => {
                match t {
                    QvType::Val | QvType::Undefined => true,
                    _ => false,
                }
            },
            ValueType::UndefNullable => true,
        }
    }
}
