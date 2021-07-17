pub fn _test_types(){
    // no implicit conversion

    let _decimal = 55.432_f32;
    //let int: u32 = decimal;

    let _casted_dec_u32 : u32 = _decimal as u32;
    let _casted_dec_u8: u8 = _decimal as u8;
    println!("{}, {}", _casted_dec_u32, _casted_dec_u8);

    println!("{:b}",1000);

    // size
    // Suffixed literals, their types are known at initialization
    let _x = 1u8;
    let _y = 2u32;
    let _z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let _i = 1;
    let _f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&_x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&_y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&_z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&_i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&_f));

    // inference
    let _elem = 5u8;

    let mut _vec = Vec::new();
    // vec of u8s
    _vec.push(_elem);
    println!("{:?}", _vec);

    // aliasing
    type Nanosecond = u64;
    type Inch = u64;

    let _nanoseconds : Nanosecond = 5;
    let _inches: Inch = 2;

    // aliasing doesn't provide type safety
    // time + distance = velocity ;)
    println!("{}+{}={}", _nanoseconds, _inches, _nanoseconds+_inches);
}
