enum _WebEvent {
    // either unit like
    PageLoad,
    PageUnload,
    // or tuple
    KeyPress(char),
    Paste(String),
    // c like structs
    Click { x: i64, y:i64 },
}

fn _inspect_web_events(event: _WebEvent) {
    match event {
        _WebEvent::PageLoad => println!("Page loaded!"),
        _WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        _WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        _WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        _WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn _test_inspect_web_events(){
    let pressed = _WebEvent::KeyPress('s');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = _WebEvent::Paste("my text".to_owned());
    let click   = _WebEvent::Click { x: 20, y: 80 };
    let load    = _WebEvent::PageLoad;
    let unload  = _WebEvent::PageUnload;

    _inspect_web_events(pressed);
    _inspect_web_events(pasted);
    _inspect_web_events(click);
    _inspect_web_events(load);
    _inspect_web_events(unload);
}

enum _WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers
{
    Add,
    Subtract
}

type _MathOps = _WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers;

pub fn _test_long_enums() {
    let a = _MathOps::Add;
    let s = _MathOps::Subtract;

    let y = _MathOps::_run(&a, 2, 3);
    let z = _MathOps::_run(&s, 3, 2);
    println!("{} : {}",y, z);
}


impl _WhoMakesSuchLongNameTypesOhProbablyTheOOPDevelopers {
    fn _run(&self, x: i32, y: i32 ) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

