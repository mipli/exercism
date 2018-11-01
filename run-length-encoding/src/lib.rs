use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|c| c.clone())
        .into_iter()
        .map(|(c, group)| {
            let count = group.count();
            if count == 1 {
                c.to_string()
            } else {
                count.to_string() + &c.to_string()
            }
        })
        .join("")

}

fn repeat_char(c: char, n: usize) -> String {
    [c].iter().cycle().take(n).map(|c| c.to_string()).collect::<Vec<_>>().join("")
}

pub fn decode(source: &str) -> String {
    let (_, message) = source.chars().fold((String::new(), String::new()), |(mut cur, mut message), c| {
        if c.is_numeric() {
            cur.push(c)
        } else {
            if cur != "" {
                let count = cur.parse::<usize>().unwrap();
                message.push_str(&repeat_char(c, count));
            } else {
                message.push(c);
            }
            cur = String::new();
        }
        (cur, message)
    });
    message
}
