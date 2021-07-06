enum WebEvent {
    // either unit like
    PageLoad,
    PageUnload,
    // or tuple
    KeyPress(char),
    Paste(String),
    // c like structs
    Click { x: i64, y:i64 },
}

fn inspect_web_events(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn test_inspect_web_events(){
    let pressed = WebEvent::KeyPress('s');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect_web_events(pressed);
    inspect_web_events(pasted);
    inspect_web_events(click);
    inspect_web_events(load);
    inspect_web_events(unload);
}

enum WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers
{
    Add,
    Subtract
}

type MathOps = WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers;

pub fn test_long_enums() {
    let a = MathOps::Add;
    let s = MathOps::Subtract;

    let y = MathOps::run(&a, 2, 3);
    let z = MathOps::run(&s, 3, 2);
    println!("{} : {}",y, z);
}


impl WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers {
    fn run(&self, x: i32, y: i32 ) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

