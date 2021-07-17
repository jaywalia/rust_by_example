pub fn _test_variables(){
    let _an_int = 1u32;
    let _a_bool = true;
    let _unit = ();

    let _copy_int = _an_int;

    println!("int: {:?}", _copy_int);

    let _set_in_my_ways = 99;
    let mut _go_with_the_flow = 120;

    _go_with_the_flow = 130;
    println!("before: {}",_go_with_the_flow);

    {
        // shadowing, changing type too
        let _go_with_the_flow = "blue";
        println!("inside: {}",_go_with_the_flow);

        //_go_with_the_flow = "turnip";

        // shadowing !mutable
        let _set_in_my_ways = "green";
        println!("fixed: {}", _set_in_my_ways);

        //_set_in_my_ways = "pink";
    }

    println!("after: {}",_go_with_the_flow);
    println!("fixed: {}", _set_in_my_ways);
}