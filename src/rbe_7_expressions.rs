
pub fn _test_expressions() {

    let x = 5u32;
    let y = {
        let x_sq = x * x;
        let x_cube = x_sq * x;
        // return expression
        x + x_sq + x_cube
    };
    let z = {
        2 * x
    };
    println!("x:{}| y:{}| z:{}", x, y, z);
}