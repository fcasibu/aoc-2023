#![allow(dead_code)]

use std::env;
use std::fs;
use std::process::exit;

pub fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = args[1].to_string();
        if let Ok(input) = fs::read_to_string(file_path) {
            return input;
        }

        eprintln!("Something went wrong with reading file");
    } else {
        eprintln!("Provide a file path to input");
    }

    exit(1);
}

pub fn read_lines<T>(callback: impl Fn(&str, usize) -> T) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let input = read_input();

    for (index, line) in input.lines().filter(|x| !x.is_empty()).enumerate() {
        result.push(callback(line, index))
    }

    return result;
}

pub fn get_sum_of(nums: &Vec<i32>) -> i32 {
    return nums.iter().sum();
}

pub fn get_product_of(nums: &Vec<i32>) -> i32 {
    return nums.iter().product();
}

pub fn is_digit(ch: &char) -> bool {
    return ch.is_digit(10);
}
