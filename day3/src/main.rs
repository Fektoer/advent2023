use regex::Regex;
use std::collections::HashMap;

fn main() {
    //let parts = include_str!("../testInput.txt").lines();
    let parts = include_str!("../input.txt").lines();
    let cloned_parts = parts.clone();

    let matrix = parts
        .map(|part| part.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut gears: HashMap<String, Vec<&str>> = HashMap::new();

    let sum_parts = cloned_parts
        .enumerate()
        .map(|(row_index, part)| {
            let re = Regex::new(r"(\d+)").unwrap();
            let mut intermediate_sum = 0;
            for cap in re.captures_iter(part) {
                let cap_index = cap.get(1).unwrap().start();
                let cap_value = cap.get(1).unwrap().as_str();

                let mut surrounding_chars: Vec<(String, char)> = Vec::new();

                // line above
                if row_index > 0 {
                    surrounding_chars.append(&mut parse_line(
                        matrix.clone(),
                        row_index - 1,
                        cap_index,
                        cap_index + cap_value.len() - 1,
                    ));
                }

                // current line
                surrounding_chars.append(&mut parse_line(
                    matrix.clone(),
                    row_index,
                    cap_index,
                    cap_index + cap_value.len() - 1,
                ));

                // line below
                if row_index < matrix.len() - 1 {
                    surrounding_chars.append(&mut parse_line(
                        matrix.clone(),
                        row_index + 1,
                        cap_index,
                        cap_index + cap_value.len() - 1,
                    ));
                }

                let filtered_chars = surrounding_chars
                    .iter()
                    .filter(|(_, c)| !c.is_digit(10) && c.to_string() != ".")
                    .collect::<Vec<_>>();

                if filtered_chars.len() > 0 {
                    intermediate_sum += cap_value.parse::<u32>().unwrap();
                }

                let _ = filtered_chars
                    .iter()
                    .map(|(key, c)| {
                        if c.to_string() == "*" {
                            if let Some(values) = gears.get_mut(key) {
                                values.push(cap_value);
                            } else {
                                gears.insert(key.to_string(), vec![cap_value]);
                            }
                        }
                    })
                    .collect::<Vec<_>>();
            }
            return intermediate_sum;
        })
        .sum::<u32>();

    println!("3A {:?}", sum_parts);

    let sum_gears =
        gears
            .iter()
            .filter(|(_, values)| values.len() == 2)
            .fold(0, |mut acc, result| {
                acc += result.1[0].parse::<u32>().unwrap() * result.1[1].parse::<u32>().unwrap();
                acc
            });

    println!("3B {}", sum_gears);
}

fn parse_line(
    matrix: Vec<Vec<char>>,
    row_index: usize,
    start: usize,
    end: usize,
) -> Vec<(String, char)> {
    let mut vec_chars: Vec<(String, char)> = Vec::new();
    if start > 0 {
        vec_chars.push((
            row_index.to_string() + (start - 1).to_string().as_str(),
            matrix[row_index][start - 1],
        ))
    }
    if end < matrix[row_index].len() - 1 {
        vec_chars.push((
            row_index.to_string() + (end + 1).to_string().as_str(),
            matrix[row_index][end + 1],
        ))
    }

    for index in start..=end {
        vec_chars.push((
            row_index.to_string() + index.to_string().as_str(),
            matrix[row_index][index],
        ))
    }
    return vec_chars;
}
