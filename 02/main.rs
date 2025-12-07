fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (id, game) = line.split_once(": ").unwrap();
            let mut val: i32 = id[5..].parse().unwrap();
            let game = game.replace(';', ",");
            let cubes: Vec<&str> = game.split(", ").collect();
            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();
                match color {
                    "red" => {
                        if amount > 12 {
                            val = 0
                        }
                    }
                    "green" => {
                        if amount > 13 {
                            val = 0
                        }
                    }
                    "blue" => {
                        if amount > 13 {
                            val = 0
                        }
                    }
                    _ => panic!("invalid input"),
                }
            }
            val
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let (_, sets) = line.split_once(": ").unwrap();
            for set in sets.split("; ") {
                for pair in set.split(", ") {
                    let (num, color) = pair.split_once(' ').unwrap();
                    let num: i32 = num.parse().unwrap();
                    match color {
                        "red" => red = red.max(num),
                        "green" => green = green.max(num),
                        "blue" => blue = blue.max(num),
                        _ => panic!("invalid input"),
                    }
                }
            }
            red * green * blue
        })
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let _ = args.next();
    let input_file = args.next().unwrap();
    let input = std::fs::read_to_string(input_file)?;

    let result = part1(&input);
    println!("Part 1: {result}");

    let result = part2(&input);
    println!("Part 2: {result}");

    Ok(())
}
