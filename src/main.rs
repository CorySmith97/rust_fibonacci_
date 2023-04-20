
fn main() {
    let mut x = 1;
    let mut y = 1;
    let mut fib: i32 = 1;

    for _i in 1..15 {
        println!("{}",fib);
        fib = y + x;
        x = y;
        y = fib;
    }
}

