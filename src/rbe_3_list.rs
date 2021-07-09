
enum _List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<_List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// looking at following syntax
// it is pretty convoluted and 
// will make Rust adoption slower

// need List::* to expose Con & Nil
use _List::*;
// Methods can be attached to an enum
impl _List {
    // Create an empty list
    fn _new() -> _List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn _prepend(self, elem: u32) -> _List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn _len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail._len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn _stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail._stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

// build a list
pub fn _test_list() {
    let mut list = _List::_new();

    list = list._prepend(1);
    list = list._prepend(2);
    list = list._prepend(3);

    println!("linked list has length: {}", list._len());
    println!("{}", list._stringify());
}