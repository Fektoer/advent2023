pub fn main() {
    
    let a = include_str!("../input.txt")
        .lines()
        .map(|calibration| extract_digits(calibration, false))
        .sum::<u32>();

    let b = include_str!("../input.txt")
    .lines()
    .map(|calibration| extract_digits(calibration, true))
    .sum::<u32>();
 
    println!("{:?}", a);
    println!("{:?}", b);
}

fn extract_digits(line: &str, parse: bool) -> u32 {
    
    let line_to_parse = if parse { parse_words(line) } else { line.to_string() };
    let digits = line_to_parse
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {
        return first_digit* 10 + last_digit   
    } else {
        return 0;
    }
}

fn parse_words(line: &str) -> String {
    return line.to_lowercase()
    .replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e");
}