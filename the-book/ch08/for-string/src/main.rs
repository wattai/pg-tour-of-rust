fn main() {
    for c in "こんにちは".chars() {
        println!("{c}");
    }
    println!();
    for b in "こんにちは".bytes() {
        println!("{b}");
    }
}
