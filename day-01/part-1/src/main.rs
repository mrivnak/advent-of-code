fn main() {
    let input = include_str!("../../input.txt").lines();

    let sum: u32 = input
        .map(|line| {
            let digits = line
                .chars()
                .filter(|&c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();

    println!("{}", sum);
}
