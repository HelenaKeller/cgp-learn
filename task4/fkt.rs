use std::ops::{Add, Mul};

fn function<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {

    (x + y, x * y)

}

fn main() {

    println!("3 und 4: {:?}", function(3, 4));

}
