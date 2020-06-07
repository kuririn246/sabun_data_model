
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ArrayType{
    Num,
    String,
    Num2, //two dimensional num array
}

impl ArrayType{
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ArrayType::Num => 0,
            ArrayType::String => 1,
            ArrayType::Num2 => 2,
        }
    }
}
