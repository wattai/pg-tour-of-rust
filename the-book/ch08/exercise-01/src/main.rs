use std::collections::HashMap;

fn calc_mean(x: &Vec<i32>) -> f32 {
    return x.iter().sum::<i32>() as f32 / (x.len() as f32);
}

fn calc_median(x: &Vec<i32>) -> f32 {
    let mut sorted_x = x.clone();
    sorted_x.sort();
    let mid = sorted_x.len() / 2;
    if sorted_x.len() % 2 == 0 {
        return (sorted_x[mid - 1] + sorted_x[mid]) as f32 / 2.;
    } else {
        return sorted_x[mid] as f32;
    }
}

fn calc_mode(x: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();
    for &value in x {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    return occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap_or(0)
    ;
}

fn main() {
    // let inputs: Vec<i32> = vec![1, 2, 3, 4, 5, 3, 1, 1, 1];
    let inputs: Vec<i32> = vec![1, 2, 2, 3, 4, 6];

    let mean = calc_mean(&inputs);
    let median = calc_median(&inputs);
    let mode = calc_mode(&inputs);

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
