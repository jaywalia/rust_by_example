#![allow(unreachable_code)]

pub fn _test_if_else(n: u32){
    if n%2 == 0 {
        println!("{} is even", n);
    } else {
        println!("{} is odd", n);
    }
}

pub fn _test_loop(n: u32){
    let mut cnt = n;
    loop {
        if cnt == 0 {
            break;
        }

        println!("{}", cnt);
        cnt -= 1;
    }
}

pub fn _test_nested_labels(){
    'step_1: loop {
        print!("hello");
        print!(" ");
        //'step_2: loop {
        loop {
            print!("world");
            // break 'step_2;
            break 'step_1;
        }
    }
    println!("!!!!");
}

pub fn _test_loop_returns() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 40 {
            break counter/2;
        }
    };
    assert_eq!(result, 20);
}

pub fn _test_while() {
    let mut n = 1;
    // can't return values from break for while
    while n < 1000 {
        if n % 100 == 0 {
            println!("{}", n/100);
        }
        n += 1;
    };

}
