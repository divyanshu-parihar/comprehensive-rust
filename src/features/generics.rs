use std::fmt::{Debug, Display};
use std::ops::Add;

pub fn pick<T, V>(num: T, num2: V) -> <T as Add<V>>::Output
where
    T: Add<V>,
{
    return num + num2;
}

pub fn trait_return(num: impl Display) -> impl Debug {
    println!("{num} something that implement Display");
    return 42;
}
