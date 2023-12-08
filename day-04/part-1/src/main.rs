use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt").lines();

    let sum = input
        .map(|line| {
            let mut is_winning_nums = false;
            let mut winning_nums = HashSet::new();
            let mut nums = HashSet::new();
            let mut current = 0;
            for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    current = current * 10 + c.to_digit(10).unwrap();
                } else if c == ':' {
                    is_winning_nums = true;
                    current = 0;
                } else if c == '|' {
                    is_winning_nums = false;
                }
                if c == ' ' || i == line.len() - 1 {
                    if current == 0 {
                        continue;
                    }
                    if is_winning_nums {
                        winning_nums.insert(current);
                    } else {
                        nums.insert(current);
                    }
                    current = 0;
                }
            }

            let wins = nums.intersection(&winning_nums).count();
            calculate_score(wins as u32)
        })
        .sum::<u32>();

    println!("{}", sum);
}

fn calculate_score(wins: u32) -> u32 {
    match wins {
        0 => 0,
        1 => 1,
        x => u32::pow(2, x - 1),
    }
}
