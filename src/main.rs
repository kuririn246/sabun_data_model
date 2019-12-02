use std::iter::repeat;

struct StartFrom{
    index : i32,
    vec : Vec<i32>
}

struct CSabuns{
    vec : Vec<StartFrom>
}

struct Sabuns{
    start_index : i32,
    initial : Vec<i32>,
    a_sabun : Vec<StartFrom>,
    b_sabun : Vec<StartFrom>,
    c_sabun : Vec<CSabuns>,
    last_sabun : String,
}

fn calc_sabuns(sabuns : &mut Sabuns, current : &Vec<i32>){
    let last_sabun = match sabuns.last_sabun.as_str(){
        "i" => 0,
        "a" => 1,
        "b" => 2,
        "c" => 3,
        _ => -1,
    };
}

fn main() {
//    fn last_index(vec : &Vec<StartFrom>) -> i32{
//        let last = vec.last().unwrap();
//        return last.index + last.vec.len();
//    }

    let first = StartFrom{
        index : 0,
        vec : vec![0; 100]
    };

    let mut current = first.vec.clone();

    let mut vec : Vec<StartFrom> = vec![];
    vec.push(first);

    //データが一個ずつふえ、rewrite個のデータがrewriteされる。
    //rewriteする数は一定で、かならず配列の最後の部分だけrewriteする。
    //なので変化しない部分も一つずつ増えていく。
    let increase = 1;
    let rewrite = 10;

    for n in 1..100{
        current.push(n);
        let start = current.len() - rewrite;
        for i in start..current.len(){
            current[i] = n;
        }


    }






}
