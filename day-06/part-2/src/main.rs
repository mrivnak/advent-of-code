use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");

    let re = Regex::new(r"Time:\s+((?:\s*\d+)+)\nDistance:\s+((?:\s*\d+)+)").expect("Invalid regex");
    let captures = re.captures(input).expect("No captures found");
    let time = captures.get(1).expect("No times found").as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().expect("Invalid time");
    let dist = captures.get(2).expect("No distances found").as_str().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().expect("Invalid distance");
    let possible = possible_races(time);
    let winning = possible.iter().filter(|&d| *d > dist);
    let result = winning.count();
    println!("{}", result);
}

fn possible_races(time: u64) -> Vec<u64> {
    (0..=time).map(|t| {
        let hold_time = time - t; // hold_time = speed
        let run_time = t;
        hold_time * run_time
    }).collect()
}
