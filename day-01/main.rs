fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line = parse_part2(line);
            let v: Vec<_> = line.chars().filter(|chr| chr.is_numeric()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn parse_part2(input: &str) -> String {
    let possible_values: &[(&str, char); 18] = &[
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ];

    let mut numbers = String::new();

    for i in 0..input.len() {
        let text = &input[i..];

        for (ident, number) in possible_values.iter() {
            if text.starts_with(ident) {
                numbers.push(*number);
            }
        }
    }

    numbers
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let input_file = args.next().unwrap();
    let input = std::fs::read_to_string(input_file).unwrap();

    let result = part1(&input);
    println!("Part 1: {result}");

    let result = part2(&input);
    println!("Part 2: {result}");
}
