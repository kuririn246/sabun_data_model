use sabun_maker::structs::VarType;

pub fn var_type_name(var : VarType) -> String{
    match var{
        VarType::Nullable => "NullOr".to_string(),
        VarType::Undefiable => "UndefOr".to_string(),
        VarType::UndefNullable => "Qv".to_string(),
        VarType::Normal => "".to_string(),
    }
}