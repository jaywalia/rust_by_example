/// Rust by example is a walk through of rust via code
/// jatindera walia | 3rd July 2021

// https://doc.rust-lang.org/stable/rust-by-example/index.html


mod rbe_1_print;
mod rbe_1_print_list;
mod rbe_1_formatting;
mod rbe_2_primitives;

/// main funtion : entry point
fn main() {
    rbe_2();
}

// chapter 2
fn rbe_2(){
    // primitives
    rbe_2_primitives::_primitives();
    // literals and operators
    rbe_2_primitives::_literals_n_operators();
    // tuples
    rbe_2_primitives::_tuples();
    // matrix
    rbe_2_primitives::_test_matrix();
}

// chapter 1
fn _rbe_1(){
    //! Ocean for crabs!
    println!("Hello, ocean by Rustaceans!");

    // printing dates
    rbe_1_print::_print_days_left_in_year();

    // print struct
    rbe_1_print::_print_test_person();

    // print point
    rbe_1_print::_print_test_point2d();

    // print list of floats
    let lf = rbe_1_print_list::List(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{}", lf);

    // print colors
    rbe_1_formatting::_test_print_city_colors();
}
