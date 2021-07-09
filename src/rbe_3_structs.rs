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


struct _Unit;

struct _Pair(i32, f32);

struct _Point{
    x: f32,
    y: f32
}

struct _Rectangle {
    top_left: _Point,
    bottom_right: _Point
}

// activity 1
fn _rect_area(r: &_Rectangle) -> f32 {
    // destructure
    // syntax is really ugly
    //Point{x: top_left_x}   is assigning the value to top_left_x
    let _Rectangle {
        top_left: _Point{x: top_left_x, y: top_left_y}, 
        bottom_right: _Point{x: bottom_right_x, y: bottom_right_y}} = r;

    let length: f32 = bottom_right_x - top_left_x;
    let width: f32 = top_left_y - bottom_right_y;
    
    return f32::abs(length * width);
}

// activity 2
fn _square(p: _Point, len: f32) -> _Rectangle {
    //! Point p is bottom left,
    let r = _Rectangle{
        top_left: _Point{x:p.x, y:p.y+len},
        bottom_right: _Point{x:p.x+len, y:p.y}
    };

    r
}

pub fn _test_structures() {
    _test_print_person();
    _test_print_points();
    _test_print_others();

    _test_print_rectangle_activity();
}

fn _test_print_points(){
    // Instantiate a `Point`
    let point: _Point = _Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = _Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let _Point { x: top_edge, y: left_edge } = point;

    let _rectangle = _Rectangle {
        // struct instantiation is an expression too
        top_left: _Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _rect_area = _rect_area(&_rectangle);

    println!("Areas: {}", _rect_area);
}

fn _test_print_others(){
    // Instantiate a unit struct
    let _unit = _Unit;

    // Instantiate a tuple struct
    let pair = _Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let _Pair(int_val, dec_val) = pair;

    println!("pair contains {:?} and {:?}", int_val, dec_val);
}

fn _test_print_rectangle_activity() {
    _activity_1();
    _activity_2();
}

fn _activity_1(){
    let _r2 = _Rectangle {
        top_left: _Point {x: 1.0, y: 4.0},
        bottom_right: _Point {x: 4.0, y: 1.0}
    };
    let _r2_area = _rect_area(&_r2);
    println!("Areas: {}", _r2_area);
}



fn _activity_2(){
    let p = _Point{x:2.0,y:2.0};
    let len = 10.0;
    let sq = _square(p, len);
    println!("{}", _rect_area(&sq));
}

fn _test_print_person() {
    let name = String::from("Peter");
    let age = 1;
    let peter = Person {name, age};

    println!("{}", peter);
}