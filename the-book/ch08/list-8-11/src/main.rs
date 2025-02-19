fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let mut s = String::from("こんｊんいちは");
    s.push_str("hogehoge");
    println!("Hello, world! {}", s);
}

