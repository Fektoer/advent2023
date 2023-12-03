use regex::Regex;

pub fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let sum_legit = include_str!("../input.txt")
        .lines()
        .map(|game| {
            let first_split: Vec<&str> = Regex::new(r": ").unwrap().split(game).collect(); // game # - set of three draws
            let mut second_split: Vec<&str> =
                Regex::new(r"; ").unwrap().split(&first_split[1]).collect(); // separate sets of three draws

            let mut game_valid = true;
            let mut min_red = 1;
            let mut min_green = 1;
            let mut min_blue = 1;

            for index in 0..second_split.len() {
                let set = &mut second_split[index];
                let mut third_split: Vec<&str> = Regex::new(r", ").unwrap().split(set).collect(); // separate draws
                let mut draw_valid = true;

                for index in 0..third_split.len() {
                    let draw = &mut third_split[index];
                    let fourth_split: Vec<&str> = draw.split(' ').collect(); // split in number and color
                    let number = fourth_split[0].parse::<u32>().unwrap();

                    match fourth_split[1] {
                        "red" => {
                            draw_valid = draw_valid && (number <= max_red);
                            if number > min_red {
                                min_red = number
                            };
                        }
                        "green" => {
                            draw_valid = draw_valid && (number <= max_green);
                            if number > min_green {
                                min_green = number
                            };
                        }
                        "blue" => {
                            draw_valid = draw_valid && (number <= max_blue);
                            if number > min_blue {
                                min_blue = number
                            };
                        }
                        _ => draw_valid = false,
                    }
                }
                game_valid = game_valid && draw_valid;
            }

            let game_number = Regex::new(r"Game ")
                .unwrap()
                .split(first_split[0])
                .collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();

            return (game_number, game_valid, min_red * min_green * min_blue);
        })
        .collect::<Vec<_>>()
        .iter()
        .fold((0, 0), |(mut acc_a, mut acc_b), result| {
            if result.1 {
                acc_a += result.0
            } else {
                acc_a += 0
            };
            acc_b += result.2;
            (acc_a, acc_b)
        });

    println!("2A {:?}", sum_legit.0);
    println!("2B {:?}", sum_legit.1);
}
