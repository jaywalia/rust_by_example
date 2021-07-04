//==================================================
// uiuc cs196 sp20 closures
//  https://www.youtube.com/watch?v=6dRy_K8IW5w&t=156s
use std::fmt;

#[derive(Debug)]
pub struct Shoe {
    size: u32,
    style: String
}

impl fmt::Display for Shoe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} in size {}", &self.style, &self.size)
    }
}

pub fn _shoes_that_fits_me(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        // closure passed to filter  
        .filter(|s| s.size == my_size)
        .collect()
}

pub fn _test_closure() {
    _test_closure_map();
    _test_closure_filter();
    _test_closure_fold();
}

fn _test_closure_map() {
    let v: Vec<i32> = vec![1,2,3];

    let v2: Vec<_> = v.iter().map(|x| x*x).collect();

    println!("{:?}", v);
    println!("{:?}", v2);
}

fn _test_closure_filter() {
    let shoes: Vec<Shoe> = vec![
        Shoe{size: 9, style: String::from("Oxford")},
        Shoe{size: 10, style: String::from("Boots")},
        Shoe{size: 9, style: String::from("Sneakers")}
    ];

    println!("{:#?}",_shoes_that_fits_me(shoes, 9));

}

fn _test_closure_fold() {

    let v: Vec<u64> = vec![1,2,3,4,5,6,7,8,9,10];

    // let adder = |s, x| s+x;
    // let mul = |m:u64, x:&u64| m*x;

    // let s: u64 = v.iter().fold(0, adder);
    // let m: u64 = v.iter().fold(1, mul); //|m,x|m*x

    let s: u64 = v.iter().fold(0, |s,x| s+x);
    let m: u64 = v.iter().fold(1, |m,x| m*x);

    println!("{:?}", v);
    println!("Sum: {}, \nMul: {}", s, m);
}

