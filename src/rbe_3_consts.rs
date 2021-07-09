static _LANG: &str = "Rust";
const _THRESHOLD: i32 = 10;

fn _is_big(n: i32) -> bool {
    n > _THRESHOLD
}

pub fn _test_consts() {
    let n = 16;

    println!("This is {}", _LANG);
    println!("The threshold is {}", _THRESHOLD);
    println!("{} is {}", n, if _is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}