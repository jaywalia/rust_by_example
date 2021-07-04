// https://doc.rust-lang.org/stable/rust-by-example/primitives.html

pub fn _primitives() {
    let _logical : bool = true;

    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;

    let _default_float = 3.0;
    let _default_int = 3;

    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64;

    let mut _mutable = 12;
    _mutable = 21;

    // following gives error
    //mutable = false;

    // but we can overwrite the variable with shadowing.
    let _mutable = true;
}
//==================================================
//#[allow(arithmetic_overflow)]
pub fn _literals_n_operators() {
    
    println!("1+2={}", 1u32+2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    //println!("1 - 2 = {}", 1u32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
//==================================================
fn reverse(pair:(i32, bool)) -> (bool, i32) { 
    let (a,b) = pair;
    (b,a)
}

pub fn _tuples(){
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
//==================================================

#[derive(Debug)]
pub struct Matrix(f32, f32, f32, f32);

use std::fmt;

// display (remember the ? for continuation)
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"({},{})", self.0, self.1)?;
        write!(f,"({},{})", self.2, self.3)
    }
}

// transpose the matrix (destructure the input)
pub fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

pub fn _test_matrix() {
    let m = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", m);
    let t = transpose(m);
    println!("Transpose:\n{}", t);

}

//==================================================
use std::mem;

fn analyze_slice(s: &[i32]) {
    println!("first element of the slice is: {}", s[0]);
    println!("the slice has {} elements", s.len());
}

pub fn _arrays_n_slices() {
     // Fixed-size array (type signature is superfluous)
     let xs: [i32; 5] = [1, 2, 3, 4, 5];

     // All elements can be initialized to the same value
     let ys: [i32; 500] = [0; 500];
 
     // Indexing starts at 0
     println!("first element of the array: {}", xs[0]);
     println!("second element of the array: {}", xs[1]);
     // y too
     println!("second element of the array: {}", ys[499]);
 
    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // smaller slice
    analyze_slice(&xs[2..]);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    //println!("{}", xs[5]);
}