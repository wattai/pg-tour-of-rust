fn is_vowel(s: &str) -> bool {
    match s {
        "a" => true,
        "i" => true,
        "u" => true,
        "e" => true,
        "o" => true,
        _ => false,
    }
}


fn convert(s: &str) -> String {
    let mut s = s.clone().to_string();
    let first_char = s.chars().nth(0).unwrap().to_string();

    if is_vowel(&first_char) {
        return format!("{s}-hay");
    }
    s.push_str(&format!("-{first_char}ay").to_string());
    s = s[1..].to_string();
    return s;
}


fn main() {
    let out = convert("first");
    let out = convert("opple");
    println!("RESULT: {out:?}");
}
