fn main() {
    let s = no_dangle();
    println!("Hello, world!, {s}");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    return s;
}
