use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let raw_input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for (_, [a_str, b_str]) in re.captures_iter(raw_input.as_str()).map(|c| c.extract()) {
        let a = a_str.parse::<i32>().unwrap();
        let b = b_str.parse::<i32>().unwrap();
        sum += a * b;
    }

    let mut sum2 = 0;
    let mut is_active = 1;
    let re2 = Regex::new(r"(don't\(\))|(do\(\))|mul\((\d+,\d+)\)").unwrap();

    for (_, [match_str]) in re2.captures_iter(raw_input.as_str()).map(|c| c.extract()) {
        if match_str == "don't()" {
            is_active = 0;
        } else if match_str == "do()" {
            is_active = 1;
        } else {
            let (a_str, b_str) = match_str.split_once(",").unwrap();
            let a = a_str.parse::<i32>().unwrap();
            let b = b_str.parse::<i32>().unwrap();
            sum2 += a * b * is_active;
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
