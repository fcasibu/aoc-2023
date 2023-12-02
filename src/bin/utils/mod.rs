use std::env;
use std::fs;
use std::process::exit;

pub fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match Some(args[1].to_string()) {
            Some(file_path) => {
                return fs::read_to_string(file_path)
                    .expect("Something went wrong reading the file");
            }
            _ => exit(1),
        };
    } else {
        eprintln!("No arguments provided");
        exit(1);
    }
}

pub fn read_line<T>(callback: impl Fn(&str, usize) -> T) -> Vec<T> {
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
