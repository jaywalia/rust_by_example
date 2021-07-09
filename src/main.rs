/// Rust by example is a walk through of rust via code
/// jatindera walia | 3rd July 2021

// https://doc.rust-lang.org/stable/rust-by-example/index.html

mod rbe_0_closures;
mod rbe_1_print;
mod rbe_1_print_list;
mod rbe_1_formatting;
mod rbe_2_primitives;
mod rbe_3_structs;
mod rbe_3_enums;
mod rbe_3_use;
mod rbe_3_list;
mod rbe_3_consts;
mod rbe_4_variables;
mod rbe_5_types;
mod rbe_6_conversions;

/// main funtion : entry point
fn main() {
    _rbe_7();
}

// chapter 7: expressions
fn _rbe_7(){}

// chapter 6: conversions
fn _rbe_6() {
    rbe_6_conversions::_test_from_into();
    rbe_6_conversions::_test_try_from_into();
    rbe_6_conversions::_test_circle();
}

// chapter 5 : types
fn _rbe_5() {
    rbe_5_types::_test_types();
}

// chapter 4 : variables
fn _rbe_4() {
    rbe_4_variables::_test_variables();
}

// chapter 3 : structs
fn _rbe_3(){
    //structures
    rbe_3_structs::_test_structures();
    rbe_3_enums::_test_inspect_web_events();
    rbe_3_enums::_test_long_enums();
    rbe_3_use::_test_enum_use();
    rbe_3_use::_test_c_like_enums();
    rbe_3_list::_test_list();
    rbe_3_consts::_test_consts();
}

// chapter 2 : primitives
fn _rbe_2(){
    // primitives
    rbe_2_primitives::_primitives();
    // literals and operators
    rbe_2_primitives::_literals_n_operators();
    // tuples
    rbe_2_primitives::_tuples();
    // matrix
    rbe_2_primitives::_test_matrix();
    // arrays n slices
    rbe_2_primitives::_arrays_n_slices();

}

// chapter 1 : hello
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

// chapter 0
fn _rbe_0() {
    // test closure
    rbe_0_closures::_test_closure();
}
