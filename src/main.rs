
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

fn transpose(arr: &mut [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    for i in 0..3 {
        for j in 0..3 {
            // swap elements a[i][j] a[j][i]
            if j <= i {
                continue;
            }
            let temp = arr[i][j];

            arr[i][j] = arr[j][i];
            arr[j][i] = temp;
        }
    }

    return *arr;
}

fn takes_element(elems: (i32, String, &str)) {
    // println!("{} , {}, {}",elems.0,elems.1, elems.2)
    let (.., second, third) = elems;
    println!("{second}-{third}")
}
fn main() {
    println!("Hello, Div!");
    let fib_ans = fibonacci(5);
    println!("{fib_ans}-{}", collatz_seq(11));

    // let mut arr = [0,1,2,3,4,5,6];
    // arr[0] = 10;
    // const FIRST: i32 = i32::MAX;
    // println!("{}",FIRST + 1); // GIVES OVERFLOW ERROR
    // println!("{arr:?}")

    // 2d arrays
    let mut twoarr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("{:?}", transpose(&mut twoarr));

    // take element compound elements;
    let ster = "div";
    takes_element((32, String::from("Divyanshu"), &ster));

    let input = 'a';
    match input {
        'q' => println!("quitting"),
        key if key.is_uppercase() => println!("Uppercase key"),
        _ => println!("Default"),
    }
}
