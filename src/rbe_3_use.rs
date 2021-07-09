#![allow(dead_code)]

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier
}

pub fn _test_enum_use() {

    use Status::{Rich,Poor};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("So much!!!!"),
        Poor => println!("So little!"),
    }

    match work {
        Civilian => println!("Party time!"),
        Soldier => println!("Party time too!"),
    }
}

// C like enums

enum Number {
    Zero, 
    One,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

pub fn _test_c_like_enums() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}