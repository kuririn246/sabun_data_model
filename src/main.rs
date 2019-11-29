fn main() {

    let moto = 10.0;
    let shin = 5.0;
    let b_sabun = 50.0;
    for n in 1..100{
        let mut turn = n as f64;
        let current = (b_sabun * 2.0 + shin * turn) / (turn + 2.0);
        let next = ((b_sabun * 2.0 + shin * (turn + 1.0)) + moto) / (turn + 4.0);
        let big = if current < next{ "big" } else { "small" };
        println!("{} {} {} {}", big, turn, current, next);
    }





}
