use crate::structs::qv::QvType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType{
    Normal,
    Nullable,
    Undefiable,
    UndefNullable,
}

impl ValueType{
    // pub(crate) fn is_nullable(&self) -> bool{
    //     match self{
    //         ValueType::Nullable | ValueType::UndefNullable => true,
    //         _ => false,
    //     }
    // }

    pub(crate) fn undefiable(&self) -> bool{
        match self{
            ValueType::Undefiable | ValueType::UndefNullable => true,
            _ => false,
        }
    }

    pub(crate) fn to_suffix(&self) -> String{
        let s = match self{
            ValueType::Normal => "",
            ValueType::Nullable => "?",
            ValueType::Undefiable => "!",
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

    pub(crate) fn acceptable(&self, t : &QvType) -> bool {
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
            ValueType::Undefiable => {
                match t {
                    QvType::Val | QvType::Undefined => true,
                    _ => false,
                }
            },
            ValueType::UndefNullable => true,
        }
    }

    pub(crate) fn compatible(&self, other : &Self) -> bool{
        match self{
            ValueType::Normal => match other{
                ValueType::Normal => true,
                _ => false,
            },
            ValueType::Nullable => match other{
                ValueType::Normal | ValueType::Nullable => true,
                _ => false,
            }
            ValueType::Undefiable => match other{
                ValueType::Normal | ValueType::Undefiable => true,
                _ => false,
            }
            ValueType::UndefNullable => true,
        }
    }
}
