/// https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html
/// 

use std::fmt;

pub struct List(pub Vec<f32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?; // continuing the print

        for (i, v) in vec.iter().enumerate() {
            if i != 0 { write!(f, ",")?; }
            write!(f, "{}:{:.2}",i, v)?;
        }

        write!(f,"]") // return expression
    }
}


