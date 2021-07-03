/// Rust by example is a walk through of rust via code
/// jatindera walia | 3rd July 2021



mod rbe_1_print;

/// main funtion : entry point
fn main() {
    //! Ocean for crabs!
    println!("Hello, ocean by Rustaceans!");

    // printing dates
    rbe_1_print::print_days_left_in_year();

    // print struct
    rbe_1_print::print_test_person();
}
