mod utils;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn main() {
    let input = utils::read_input();

    println!(
        "Part1: {}, Part2: {}",
        part1(&get_time_and_distance_vec(&input)),
        part2(&get_time_and_distance_vec2(&input))
    );
}

fn part1(races: &Vec<Race>) -> i64 {
    return get_num_of_ways(races);
}

fn part2(races: &Vec<Race>) -> i64 {
    return get_num_of_ways(races);
}

fn get_num_of_ways(races: &Vec<Race>) -> i64 {
    return races
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|&i| i * (race.time - i) > race.distance)
                .count() as i64
        })
        .product();
}

fn get_time_and_distance_vec(input: &String) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let distances: Vec<i64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let races: Vec<Race> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    return races;
}

fn get_time_and_distance_vec2(input: &String) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let time = lines[0].split_whitespace().skip(1).collect::<Vec<&str>>();
    let time = time.join("").parse::<i64>().unwrap_or(0);

    let distance = lines[1].split_whitespace().skip(1).collect::<Vec<&str>>();
    let distance = distance.join("").parse::<i64>().unwrap_or(0);

    return Vec::from([Race { time, distance }]);
}
