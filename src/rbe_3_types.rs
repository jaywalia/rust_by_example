use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut year = "year";
        if self.age > 1 { year = "years"; }
        write!(f, "{} is {} {} old", self.name, self.age, year)
    }
}


struct Unit;

struct Pair(i32, f32);

struct Point{
    x: f32,
    y: f32
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

// activity 1
fn rect_area(r: &Rectangle) -> f32 {
    // destructure
    // syntax is really ugly
    //Point{x: top_left_x}   is assigning the value to top_left_x
    let Rectangle {
        top_left: Point{x: top_left_x, y: top_left_y}, 
        bottom_right: Point{x: bottom_right_x, y: bottom_right_y}} = r;

    let length: f32 = bottom_right_x - top_left_x;
    let width: f32 = top_left_y - bottom_right_y;
    
    return f32::abs(length * width);
}

// activity 2
fn square(p: Point, len: f32) -> Rectangle {
    //! Point p is bottom left,
    let r = Rectangle{
        top_left: Point{x:p.x, y:p.y+len},
        bottom_right: Point{x:p.x+len, y:p.y}
    };

    r
}

pub fn test_structures() {
    test_print_person();
    test_print_points();
    test_print_others();

    test_print_rectangle_activity();
}

fn test_print_points(){
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _rect_area = rect_area(&_rectangle);

    println!("Areas: {}", _rect_area);
}

fn test_print_others(){
    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(int_val, dec_val) = pair;

    println!("pair contains {:?} and {:?}", int_val, dec_val);
}

fn test_print_rectangle_activity() {
    activity_1();
    activity_2();
}

fn activity_1(){
    let _r2 = Rectangle {
        top_left: Point {x: 1.0, y: 4.0},
        bottom_right: Point {x: 4.0, y: 1.0}
    };
    let _r2_area = rect_area(&_r2);
    println!("Areas: {}", _r2_area);
}



fn activity_2(){
    let p = Point{x:2.0,y:2.0};
    let len = 10.0;
    let sq = square(p, len);
    println!("{}", rect_area(&sq));
}

fn test_print_person() {
    let name = String::from("Peter");
    let age = 1;
    let peter = Person {name, age};

    println!("{}", peter);
}