use std::iter::{zip};
use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");

    let re = Regex::new(r"Time:\s+((?:\s*\d+)+)\nDistance:\s+((?:\s*\d+)+)").expect("Invalid regex");
    let captures = re.captures(input).expect("No captures found");
    let times = captures.get(1).expect("No times found").as_str().split_whitespace().map(|t| t.parse::<u32>().expect("Invalid time")).collect::<Vec<u32>>();
    let distances = captures.get(2).expect("No distances found").as_str().split_whitespace().map(|d| d.parse::<u32>().expect("Invalid distance")).collect::<Vec<u32>>();
    let races = zip(times, distances);

    let mut result = 1;
    for (time, dist) in races {
        let possible = possible_races(time);
        let winning = possible.iter().filter(|&d| *d > dist);
        result *= winning.count() as u32;
    }
    println!("{}", result);
}

fn possible_races(time: u32) -> Vec<u32> {
    (0..=time).map(|t|{
        let hold_time = time - t; // hold_time = speed
        let run_time = t;
        hold_time * run_time
    }).collect::<Vec<u32>>()
}
