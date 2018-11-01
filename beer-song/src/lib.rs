fn num_bottle(n: i32) -> String {
    if n >= 2 {
        format!("{} bottles", n)
    } else if n == 1 {
        "1 bottle".to_string()
    } else {
        "no more bottles".to_string()
    }
}

fn take_word(n: i32) -> String {
    if n == 1 {
        "it".to_string()
    } else {
        "one".to_string()
    }
}

pub fn verse(n: i32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else {
        format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", num_bottle(n), num_bottle(n), take_word(n), num_bottle(n - 1))
    }

}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
