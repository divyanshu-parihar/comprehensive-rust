fn fibonacci(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    let mut count = 1;
    while count != num {
        let temp = first;
        first = second;
        second = dbg!(temp) + second;
        count += 1;
    }
    return first;
}
fn collatz_seq(mut num: i32) -> i32 {
    let mut count = 1;
    loop {
        match num {
            1 => break,
            _ => {
                if num % 2 != 0 {
                    num = 3 * num + 1;
                    count += 1;
                } else {
                    num = num / 2;
                    count += 1;
                }
            }
        }
    }
    return count;
}

// fn arg_pass(val: i64) -> i64{
//     let result = val + 1;
//     return result;
// }
fn main() {
    println!("Hello, Div!");
    let fib_ans = fibonacci(5);
    println!("{fib_ans}-{}", collatz_seq(11));

    // arrays
}
