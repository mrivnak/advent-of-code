fn main() {
    let input = include_str!("../../input.txt").lines();

    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let sum: u32 = input
        .map(|line| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let mut current = 0;
            let mut prev = ' ';

            for char in line.chars() {
                if char.is_digit(10) {
                    current = current * 10 + char.to_digit(10).unwrap();
                }
                if char == ':' {
                    current = 0;
                }

                if char == 'r' && prev == ' ' {
                    if current > red {
                        red = current;
                    }
                    current = 0;
                }
                if char == 'g' && prev == ' ' {
                    if current > green {
                        green = current;
                    }
                    current = 0;
                }
                if char == 'b' && prev == ' ' {
                    if current > blue {
                        blue = current;
                    }
                    current = 0;
                }

                prev = char;
            }

            red * green * blue
        })
        .sum();

    println!("{}", sum);
}
