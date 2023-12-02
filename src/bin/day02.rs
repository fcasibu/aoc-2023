mod utils;

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let config = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let part1 = utils::read_line(|line, index| {
        let sets = get_sets(line);
        let game_result = sets
            .iter()
            .map(|set| check_if_possible_to_play(&set, &config))
            .collect::<Vec<bool>>();

        match game_result
            .into_iter()
            .filter(|x| x == &true)
            .collect::<Vec<bool>>()
            .len() as i32
        {
            0 => (index + 1) as i32,
            _ => 0,
        }
    });

    let part2 = utils::read_line(|line, _| {
        let sets = get_sets(line);
        let game_result = sets
            .iter()
            .map(|set| count_cubes(&set))
            .collect::<Vec<Cubes>>();
        let max_values = get_max_values_of(&game_result);
        return utils::get_product_of(&max_values);
    });

    println!(
        "Part1: {:?}, Part2: {:?}",
        utils::get_sum_of(&part1),
        utils::get_sum_of(&part2)
    );
}

fn get_sets(line: &str) -> Vec<&str> {
    return line.split(":").collect::<Vec<&str>>()[1]
        .split(";")
        .collect();
}

fn check_if_possible_to_play(set: &str, config: &Cubes) -> bool {
    let cubes = count_cubes(set);
    return cubes.red > config.red || cubes.green > config.green || cubes.blue > config.blue;
}

fn count_cubes(set: &str) -> Cubes {
    return set.split(", ").fold(
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        },
        |mut cubes, val| {
            let parts = val.split_whitespace().collect::<Vec<&str>>();
            if let (Some(count_str), Some(color)) = (parts.first(), parts.last()) {
                if let Ok(count) = count_str.parse::<i32>() {
                    match color {
                        &"red" => cubes.red += count,
                        &"green" => cubes.green += count,
                        &"blue" => cubes.blue += count,
                        _ => {}
                    }
                }
            }

            return cubes;
        },
    );
}

fn get_max_values_of(game: &Vec<Cubes>) -> Vec<i32> {
    let max_values = game.iter().fold(
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        },
        |acc, color| Cubes {
            red: acc.red.max(color.red),
            green: acc.green.max(color.green),
            blue: acc.blue.max(color.blue),
        },
    );

    return Vec::from([max_values.red, max_values.green, max_values.blue]);
}
