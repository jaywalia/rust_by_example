
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number{ value: item }
    }
}

pub fn _test_from_into(){
    let i: i32 = 88;
    let n: Number = Number::from(i);
    println!("{:?}",n);

    let n2: Number = i.into();
    println!("{:?}", n2);
}

//------------------------------------

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v%2 == 0 {
            Ok(EvenNumber(v))
        } else {
            Err(()) // <<<<< achtung! syntax
        }
    }
}

pub fn _test_try_from_into() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // try into
    let r : Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(r, Ok(EvenNumber(8)));

    let r: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(r, Err(()));
}


//------------------------------------

#[derive(Debug)]
struct Circle {
    r : u32
}

use std::fmt;

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius : {}", self.r)
    }
}


use std::str::FromStr;
use std::num::ParseIntError;

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                        .split(':').collect();
        let sr = parts[1].parse::<u32>()?;

        Ok(Circle{r:sr})
    }
}

pub fn _test_circle(){
    let c: Circle = Circle::from_str("(r:32)").unwrap();
    println!("{}", c);
}