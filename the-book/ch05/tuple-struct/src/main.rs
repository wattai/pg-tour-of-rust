#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let point = Point(3, 4, 5);
    println!("{black:?}");
    println!("{point:?}");
    println!("point.0: {0}", point.0);
    println!("Hello, world!");
}
