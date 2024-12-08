use std::fs;

// Brute-force solution using recursion
fn search_grid(grid: &Vec<Vec<char>>, search_str: &str, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    // Base conditions
    if search_str.len() == 0 {
        return 1;
    }
    if x < 0 || x >= (grid[0].len() as i32) || y < 0 || y >= (grid.len() as i32) {
        return 0;
    }

    let grid_char = grid[y as usize][x as usize];
    let mut search_chars = search_str.chars();
    let first_char = search_chars.next().unwrap();

    if grid_char == first_char {
        let mut count = 0;

        if dx == 0 && dy == 0 {
            // Messy nested loop

            for j in -1..=1 {
                for i in -1..=1 {
                    if !(i == 0 && j == 0) {
                        count += search_grid(
                            grid,
                            search_chars.clone().collect::<String>().as_str(),
                            x + i,
                            y + j,
                            i,
                            j,
                        );
                    }
                }
            }
        } else {
            count += search_grid(
                grid,
                search_chars.clone().collect::<String>().as_str(),
                x + dx,
                y + dy,
                dx,
                dy,
            )
        }

        return count;
    }

    return 0;
}

fn part_1(grid: &Vec<Vec<char>>) {
    let mut total_count = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            total_count += search_grid(&grid, "XMAS", i as i32, j as i32, 0, 0);
        }
    }

    println!("Part 1: {total_count}");
}

fn find_mas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut mas_count = 0;

    for j in (-1..=1).step_by(2) {
        for i in (-1..=1).step_by(2) {
            if grid[(y - j) as usize][(x - i) as usize] == 'M'
                && grid[(y + j) as usize][(x + i) as usize] == 'S'
            {
                mas_count += 1;
            }
        }
    }

    if mas_count == 2 {
        return 1;
    }

    return 0;
}

fn part_2(grid: &Vec<Vec<char>>) {
    let mut total_count = 0;

    if grid[0].len() >= 3 && grid.len() >= 3 {
        for j in 1..grid.len() - 1 {
            for i in 1..grid[0].len() - 1 {
                if grid[j][i] == 'A' {
                    total_count += find_mas(&grid, i as i32, j as i32);
                }
            }
        }
    }

    println!("Part 2: {total_count}");
}

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = raw_input.lines().map(|x| x.chars().collect()).collect();
    part_1(&grid);
    part_2(&grid);
}
