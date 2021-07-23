
pub fn _test_match(){
    for age in 10..15 {
        let msg: &str;
        match age {
            1..=12 => { msg = "still a kid, "},
            13..=19 => { msg = "a teen" },
            20..=35 => { msg = "young man" },
            36..=50 => { msg = "getting there"},
            51..=65 => { msg = "almost there"},
            66..=80 => { msg = "not bad" },
            81..=100 => { msg = "hang in there" },
            _ => { msg = "immortal" }
        }
        println!("{}: {}", age, msg);
    }

    for age in 1..5 {
        println!("{}", age);
    }
}