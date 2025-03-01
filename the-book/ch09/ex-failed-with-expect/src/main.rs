use std::fs::File;

fn main() {
    // hello.txt を開くのに失敗しました
    let f = File::open("hoghoeg.txt").expect("Failed to open hogehoge.txt");
}
