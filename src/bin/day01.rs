use std::collections::HashMap;
mod utils;

fn main() {
    let part1 = utils::read_line(get_digits);

    let part2 = utils::read_line(|line| get_digits(replace_words_with_digits(&line).as_str()));

    println!(
        "Part1: {:?}, Part2: {:?}",
        utils::get_sum_of_nums(&part1),
        utils::get_sum_of_nums(&part2)
    );
}

fn get_digits(characters: &str) -> i32 {
    let mut digits = Vec::new();
    for char in characters.chars() {
        if is_digit(char) {
            digits.push(char);
        }
    }

    match concat_first_and_last_characters(&digits).parse::<i32>() {
        Ok(value) => value,
        _ => 0,
    }
}

fn concat_first_and_last_characters(characters: &Vec<char>) -> String {
    match (characters.first(), characters.last()) {
        (Some(first), Some(last)) => {
            return first.to_string() + &last.to_string();
        }
        _ => String::new(),
    }
}

fn replace_words_with_digits(characters: &str) -> String {
    let digit_map: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "fr4"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9n"),
    ]);

    let mut result = characters.to_string();

    for (key, value) in digit_map.iter() {
        result = result.replace(key, value);
    }

    return result;
}

fn is_digit(ch: char) -> bool {
    return ch >= '0' && ch <= '9';
}
