use std::{collections::HashMap, fs};

const INPUT_FILE_PATH: &str = "input.txt";

fn distance(a: i32, b: i32) -> i32 {
    return (a - b).abs();
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let raw_lines = raw_input.lines();

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for raw_line in raw_lines {
        let line_strs: Vec<&str> = raw_line.split("   ").collect();

        v1.push(line_strs.get(0).unwrap().parse::<i32>().unwrap());
        v2.push(line_strs.get(1).unwrap().parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let mut distance_sum = 0;
    for i in 0..v1.len() {
        distance_sum += distance(*v1.get(i).unwrap(), *v2.get(i).unwrap());
    }

    println!("Distance Sum: {}", distance_sum);

    // Part 2
    let mut buckets = HashMap::new();
    for num in v2 {
        let str_num = num.to_string();
        let count = buckets.entry(str_num).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;
    for num in v1 {
        let str_num = num.to_string();
        similarity_score += num * buckets.get(&str_num).copied().unwrap_or(0);
    }

    println!("Similarity Score: {}", similarity_score);
}
