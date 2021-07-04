/// print examples
/// 

pub fn _print_days_left_in_year(){
    println!("{}", 10);

    // positional args
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named args
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    // formatting
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // align
    println!("{number:>width$}", number=1, width=6);
    
    // pad 0s
    println!("{number:>0width$}", number=1, width=6);

    // missing args
    //println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
}

#[derive(Debug)]
pub struct Person<'a>{
    name: &'a str,
    age:u8
}

use std::fmt;

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

pub fn _print_test_person() {
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // simple print : doesn't work
    // now works with fmt::Display implemented
    println!("{}", peter);

    // debug print
    println!("{:?}", peter);

    // pretty print
    println!("{:#?}", peter);
}

//#[derive(Debug)]
pub struct Point2D {x:f64, y:f64}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn _print_test_point2d() {
    let origin = Point2D {x: 0.0, y: 0.0};
    println!("{}",origin);

    // following won't work as #derive(Debug) is not on
    //println!("{:?}",origin);
    //println!("{:#?}",origin);
}