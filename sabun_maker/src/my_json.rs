use indexmap::IndexMap;

pub enum Value{
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Map(IndexMap<String, Value>),
    Null,
    Undefined,
}

impl Value{
    pub fn to_string_pretty(&self) -> String{
        let mut s = String::new();
        write(self, &mut s, 0);
        return s;
    }
}

fn write(value : &Value, s : &mut String, indent_level : usize){
    match value{
        Value::Bool(b) => s.push_str(&b.to_string()),
        Value::String(val) => s.push_str(&format!(r#""{}""#, val)),
        Value::Null => s.push_str("null"),
        Value::Undefined => s.push_str("undefined"),
        Value::Number(f) => s.push_str(&f.to_string()),
        Value::Array(v) => write_array(v, s, indent_level + 1),
        Value::Map(obj) =>{
            s.push_str("{\n");
            for (k,v) in obj{
                s.push_str(&indent_str(indent_level + 1));
                s.push_str(&format!("\"{}\" : ", k));
                write(v, s, indent_level + 1);
                s.push_str(",\n");
            }
            s.push_str(&indent_str(indent_level));
            s.push_str("}");
        }
    }
}

fn write_array(array : &Vec<Value>, s : &mut String, indent_level : usize){
    if array.len() == 0{
        s.push_str("[]");
    } else if array.len() == 1 {
        s.push_str("[ ");
        write(&array[0], s, 0);
        s.push_str(" ]");
    } else {
        s.push_str("[\n");
        s.push_str(&indent_str(indent_level));
        write(&array[0], s, indent_level);
        s.push_str(",\n");
        for item in &array[1..array.len()-1] {
            s.push_str(&indent_str(indent_level));
            write(item, s, indent_level);
            s.push_str(",\n");
        }
        s.push_str(&indent_str(indent_level));
        write(&array[array.len()-1], s, indent_level);
        s.push('\n');
        if indent_level != 0 {
            s.push_str(&indent_str(indent_level - 1));
        }
        s.push(']');
    }
}

fn indent_str(indent_level : usize) -> String{
    "  ".repeat(indent_level)
}