fn main() {
    let input = include_str!("../../input.txt").lines();

    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let sum: u32 = input
        .map(|line| {
            let first = numbers
                .iter()
                .map(|n| (n, line.find(n)))
                .filter(|(_, index)| index.is_some())
                .map(|(&n, index)| (to_digit(n), index.unwrap()))
                .min_by_key(|&x| x.1)
                .unwrap()
                .0;
            let last = numbers
                .iter()
                .map(|n| (n, line.rfind(n)))
                .filter(|(_, index)| index.is_some())
                .map(|(&n, index)| (to_digit(n), index.unwrap()))
                .max_by_key(|&x| x.1)
                .unwrap()
                .0;

            (first * 10 + last) as u32
        })
        .sum();

    println!("{}", sum);
}

fn to_digit(number: &str) -> u8 {
    match number.parse::<u8>() {
        Ok(n) => n,
        Err(_) => match number {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => unreachable!(),
        },
    }
}
