fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    //let first = &v[0];
    v.push(6);
    //println!("The first element is: {first:?}");

    let v = vec![100, 32, 57];
    for idx in &v {
        println!("{idx}");
    }

    let mut v = vec![100, 32, 57];
    for idx in &mut v {
        *idx += 50;
    }
    println!("{v:?}")
}
