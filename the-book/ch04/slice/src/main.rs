fn main() {
    let binding = String::from("ai hey 123hogehoge");
    let o = first_word(&binding[..]);
    let o2 = second_word(&binding[..]);
    println!("FIRST : {o}");
    println!("SECOND: {o2}");

    let binding = "ai hey 123hogehoge";
    let o = first_word(&binding[..]);
    let o2 = second_word(&binding[..]);
    println!("FIRST : {o}");
    println!("SECOND: {o2}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut cnt = 0;
    let mut i_start = 0;
    let mut i_end = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            cnt += 1;
            if cnt == 1 {
                i_start = i + 1;
            }
            if cnt == 2 {
                i_end = i;
                return &s[i_start..i_end];
            }
        }
    }
    &s[..]
}
