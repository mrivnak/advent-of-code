use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../input.txt").lines();

    let grid = input
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut gears = HashMap::new();
    for i in 0..grid.len() {
        let mut current = 0;
        let mut digits = 0;
        for j in 0..grid[i].len() {
            if grid[i][j].is_digit(10) {
                digits += 1;
                current = current * 10 + grid[i][j].to_digit(10).unwrap();
            }
            if current != 0 && (!grid[i][j].is_digit(10) || j == grid[i].len() - 1) {
                find_gears(&grid, &mut gears, current, i, j - digits..=(j - 1));

                current = 0;
                digits = 0;
            }
        }
    }

    let sum = gears
        .values()
        .filter(|gear| gear.count == 2)
        .map(|gear| gear.value)
        .sum::<u32>();

    println!("{}", sum);
}

struct Gear {
    count: u8,
    value: u32,
}

fn find_gears(
    grid: &Vec<Vec<char>>,
    gears: &mut HashMap<(usize, usize), Gear>,
    val: u32,
    i: usize,
    j: RangeInclusive<usize>,
) {
    for il in i..=i + 2 {
        if il == 0 || il > grid.len() {
            continue;
        }
        for jl in *j.start()..=*j.end() + 2 {
            if jl == 0 || jl > grid[il - 1].len() {
                continue;
            }
            if grid[il - 1][jl - 1] == '*' {
                let point = (il - 1, jl - 1);
                if gears.contains_key(&point) {
                    let mut gear = gears.get_mut(&point).unwrap();
                    gear.count += 1;
                    gear.value *= val;
                } else {
                    gears.insert(
                        point,
                        Gear {
                            count: 1,
                            value: val,
                        },
                    );
                }
            }
        }
    }
}
