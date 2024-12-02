use std::fs;

// Forgive the bad code, I'm still learning rust

fn check_report(nums: &Vec<i32>) -> bool {
    let mut valid = true;
    let mut prev_diff: i32 = 0;
    let mut prev_num = nums.get(0).unwrap();

    for i in 1..nums.len() {
        let new_num = nums.get(i).unwrap();
        let new_diff = new_num - prev_num;

        if (prev_diff == 0 && (1..=3).contains(&new_diff.abs()))
            || (new_diff.signum() == prev_diff.signum() && (1..=3).contains(&new_diff.abs()))
        {
            prev_diff = new_diff;
            prev_num = new_num;
        } else {
            valid = false;
            break;
        }
    }

    return valid;
}

fn part_1() -> i32 {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let raw_lines = raw_input.lines();

    let mut counter = 0;

    for raw_line in raw_lines {
        let nums: Vec<i32> = raw_line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if check_report(&nums) {
            counter += 1;
        }
    }

    return counter;
}

fn part_2() -> i32 {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let raw_lines = raw_input.lines();

    let mut counter = 0;

    for raw_line in raw_lines {
        let mut safe = false;

        let nums: Vec<i32> = raw_line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        for i in 0..nums.len() {
            let damped_nums = [&nums[0..i], &nums[i + 1..nums.len()]].concat();

            if check_report(&damped_nums) {
                safe = true;
            }
        }

        if safe {
            counter += 1;
        }
    }

    return counter;
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
