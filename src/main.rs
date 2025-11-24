// use std::isie;

// fn fibonacci(num: isize) -> i64 {
//     let mut first :i32 = 0;
//     let mut second :i32 = 1;

//     let count = 1;
//     while count != num {
//         let temp :i64 = second.into() + first;
//         first = second;
//         second = temp;
//     }
//     return first.into();
// }


fn arg_pass(val: i64) -> i64{
    let result = val + 1;
    return result;
}
fn main() {
    println!("Hello, Div!");
    // let fib_ans = fibonacci(5);
    // println!("{}", fib_ans)

    let value =  10;
    let result = arg_pass(value);
    println!("{result} - {value}");
    let s1 = String::from("hello");
takes_ownership(s1); // s1's ownership is moved into the function.
println!("{s1}")
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // The function now owns 'some_string'
} // 'some_string' goes out of scope and is dropped/freed here.

