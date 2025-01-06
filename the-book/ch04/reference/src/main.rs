fn main() {
    let s1 = String::from("hello, how are you?");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
