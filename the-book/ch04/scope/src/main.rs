fn main() {
    let s = "hello";

    let mut s = String::from("hello");
    s.push_str(", world!!");
    println!("{s}");

    let s2 = s.clone();
    println!("{s}, world!");
}
