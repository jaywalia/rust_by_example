// if lets

pub fn _test_if_let(){
    _awkward_match();
    _better_if_let();

    _awkward_loop_match();
    _awesome_while_let();
}

fn _awkward_match() {
    let opt = Some(42);
    match opt {
        Some(i) => println!("Eternal answer {}", i),
        _ => println!("Don't worry, be happy"),
    }
}

fn _better_if_let() {
    let opt = Some(42);
    if let Some(i) = opt {
        println!("Answer to all : {}", i);
    } else {
        println!("This is not the one you are looking for");
    }
}

fn  _awkward_loop_match() {
    let mut opt = Some(0);
    loop {
        match opt {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    opt = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    opt = Some(i + 1);
                }
            },
            _ => { break; }
        }
    }
}

fn _awesome_while_let() {
    let mut opt = Some(0);

    while 
    let Some(i) = opt {
        if i > 9 {
            println!("Greater than 9, quit!");
            opt = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            opt = Some(i + 1);
        }
    }
}