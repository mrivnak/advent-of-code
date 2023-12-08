use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../input.txt").lines();

    let grid = input
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for i in 0..grid.len() {
        let mut current = 0;
        let mut digits = 0;
        for j in 0..grid[i].len() {
            if grid[i][j].is_digit(10) {
                digits += 1;
                current = current * 10 + grid[i][j].to_digit(10).unwrap();
            }
            if current != 0 && (!grid[i][j].is_digit(10) || j == grid[i].len() - 1) {
                if has_adjacent_symbol(&grid, i, j - digits..=(j - 1)) {
                    sum += current;
                }

                current = 0;
                digits = 0;
            }
        }
    }

    println!("{}", sum);
}

fn has_adjacent_symbol(grid: &Vec<Vec<char>>, i: usize, j: RangeInclusive<usize>) -> bool {
    for il in i..=i + 2 {
        if il == 0 || il > grid.len() {
            continue;
        }
        for jl in *j.start()..=*j.end() + 2 {
            if jl == 0 || jl > grid[il - 1].len() {
                continue;
            }
            if is_symbol(grid[il - 1][jl - 1]) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    match c {
        '$' | '/' | '*' | '+' | '-' | '=' | '&' | '#' | '@' | '%' => true,
        '.' => false,
        x if x.is_digit(10) => false,
        _ => panic!("Unknown symbol: {}", c),
    }
}
