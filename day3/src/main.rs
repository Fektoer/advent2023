use regex::Regex;

fn main() {
    //let parts = include_str!("../testInput.txt").lines();
    let parts = include_str!("../input.txt").lines();
    let parts3a = parts.clone();

    let matrix = parts
        .map(|part| part.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let sum_parts = parts3a
        .enumerate()
        .map(|(row_index, part)| {
            let re = Regex::new(r"(\d+)").unwrap();
            let mut intermediate_sum = 0;
            for cap in re.captures_iter(part) {
                let cap_index = cap.get(1).unwrap().start();
                let cap_value = cap.get(1).unwrap().as_str();

                let mut surrounding_chars: Vec<char> = Vec::new();

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

                if surrounding_chars
                    .iter()
                    .filter(|c| !c.is_digit(10) && c.to_string() != ".")
                    .collect::<Vec<_>>()
                    .len() > 0
                {
                    intermediate_sum += cap_value.parse::<u32>().unwrap();
                }
            }
            return intermediate_sum;
        })
        .sum::<u32>();

    println!("3A {:?}", sum_parts);
}

fn parse_line(matrix: Vec<Vec<char>>, row_index: usize, start: usize, end: usize) -> Vec<char> {
    let mut vec_chars: Vec<char> = Vec::new();
    if start > 0 {
        vec_chars.push(matrix[row_index][start - 1])
    }
    if end < matrix[row_index].len()-1 {
        vec_chars.push(matrix[row_index][end + 1])
    }

    for index in start..=end {
        vec_chars.push(matrix[row_index][index])
    }
    return vec_chars;
}
