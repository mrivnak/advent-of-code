use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt").lines();

    let results = input
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

            nums.intersection(&winning_nums).count() as u32
        })
        .collect::<Vec<u32>>();

    let sum = results
        .iter()
        .enumerate()
        .map(|(i, _)| get_copies(&results, i))
        .sum::<u32>();

    println!("{}", sum);
}

fn get_copies(results: &Vec<u32>, index: usize) -> u32 {
    let mut sum = 1;
    for i in 1..=results[index] {
        sum += get_copies(results, index + i as usize);
    }
    sum
}
