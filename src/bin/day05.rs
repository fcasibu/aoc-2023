mod utils;

#[derive(Debug)]
struct Conversion {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
}

fn main() {
    let input = utils::read_input();
    let almanac = input.split("\n\n").collect::<Vec<&str>>();

    println!("Part1: {}, Part2: {}", part1(&almanac), part2(&almanac));
}

fn part1(almanac: &Vec<&str>) -> i64 {
    let mut seed_numbers = get_seed_numbers(&almanac);
    let mappings = get_mappings(&almanac);
    return find_lowest_location(&mut seed_numbers, &mappings);
}

fn part2(almanac: &Vec<&str>) -> i64 {
    let seed_numbers = get_seed_numbers(&almanac);
    let mut seeds = Vec::new();

    for i in (0..seed_numbers.len()).step_by(2) {
        let start = seed_numbers[i];
        let len = seed_numbers[i + 1];
        seeds.extend(start..start + len);
    }

    let mappings = get_mappings(&almanac);
    return find_lowest_location(&mut seeds, &mappings);
}

fn get_seed_numbers(almanac: &Vec<&str>) -> Vec<i64> {
    let seeds = almanac[0].split(": ");
    let seed_numbers: Vec<i64> = seeds.last().map_or_else(Vec::new, |seeds| {
        seeds
            .split_whitespace()
            .flat_map(str::parse)
            .collect::<Vec<i64>>()
    });

    return seed_numbers;
}

fn get_mappings(almanac: &Vec<&str>) -> Vec<Vec<Conversion>> {
    let mut mappings: Vec<Vec<Conversion>> = Vec::new();

    for i in 1..almanac.len() {
        let line = almanac[i].split(":").collect::<Vec<&str>>();

        let conversions: Vec<Conversion> = line.last().map_or_else(Vec::new, |numbers| {
            numbers
                .split("\n")
                .flat_map(convert_to_ranges)
                .collect::<Vec<Conversion>>()
        });

        mappings.push(conversions);
    }

    return mappings;
}

fn convert_to_ranges(mapping: &str) -> Option<Conversion> {
    let iter = mapping
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    if iter.len() != 3 {
        return None;
    }

    let destination_range_start = iter.get(0).unwrap();
    let source_range_start = iter.get(1).unwrap();
    let range_length = iter.get(2).unwrap();

    let conversion = Conversion {
        destination_range_start: *destination_range_start,
        source_range_start: *source_range_start,
        range_length: *range_length,
    };

    return Some(conversion);
}

fn convert_number(number: i64, conversions: &Vec<Conversion>) -> i64 {
    for conversion in conversions.iter() {
        if number >= conversion.source_range_start
            && number < conversion.source_range_start + conversion.range_length
        {
            return conversion.destination_range_start + (number - conversion.source_range_start);
        }
    }

    return number;
}

fn find_lowest_location(seed_numbers: &mut Vec<i64>, mappings: &Vec<Vec<Conversion>>) -> i64 {
    for conversions in mappings {
        for number in seed_numbers.iter_mut() {
            *number = convert_number(*number, conversions);
        }
    }

    return *seed_numbers.iter().min().unwrap();
}
