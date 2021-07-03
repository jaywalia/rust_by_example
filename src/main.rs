/// Rust by example is a walk through of rust via code
/// jatindera walia | 3rd July 2021



mod rbe_1_print;
mod rbe_1_print_list;
mod rbe_1_formatting;

/// main funtion : entry point
fn main() {
    //! Ocean for crabs!
    println!("Hello, ocean by Rustaceans!");

    // printing dates
    rbe_1_print::print_days_left_in_year();

    // print struct
    rbe_1_print::print_test_person();

    // print point
    rbe_1_print::print_test_point2d();

    // print list of floats
    let lf = rbe_1_print_list::List(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{}", lf);

    // print colors
    rbe_1_formatting::test_print_city_colors();
}
