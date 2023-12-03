mod utils;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct AdjacencyInfo {
    point: Point,
    is_adjacent: bool,
}

const DEFAULT_POINT: Point = Point { x: 0, y: 0 };

fn main() {
    let engine = utils::read_lines(|line, _| return line.chars().collect::<Vec<char>>());
    println!("Part1: {}, Part2: {}", part1(&engine), part2(&engine));
}

fn part1(engine: &Vec<Vec<char>>) -> i32 {
    let result = process_engine(&engine, is_symbol);
    return result.iter().map(|(digit, _)| *digit).sum();
}

fn part2(engine: &Vec<Vec<char>>) -> i32 {
    let result = process_engine(&engine, is_gear);

    let products = result
        .iter()
        .enumerate()
        .flat_map(|(index, first)| {
            result[index + 1..].iter().map(|second| {
                if is_close(&first.1, &second.1) {
                    return utils::get_product_of(&Vec::from([first.0, second.0]));
                } else {
                    return 0;
                }
            })
        })
        .sum();

    return products;
}

fn process_engine(engine: &Vec<Vec<char>>, predicate: impl Fn(char) -> bool) -> Vec<(i32, Point)> {
    let mut symbol_adjacency_info = AdjacencyInfo {
        is_adjacent: false,
        point: DEFAULT_POINT,
    };
    let mut digit = String::new();

    let result: Vec<(i32, Point)> = engine
        .into_iter()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.into_iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (col_index, ch)| {
                    if is_symbol(*ch) || *ch == '.' {
                        if symbol_adjacency_info.is_adjacent {
                            acc.push((
                                digit.parse().unwrap_or_default(),
                                symbol_adjacency_info.point,
                            ));
                        }
                        digit.clear();
                        symbol_adjacency_info.is_adjacent = false;
                        symbol_adjacency_info.point = DEFAULT_POINT;
                    } else if utils::is_digit(ch) {
                        let adjacency = get_symbol_adjacency_info(
                            row_index as i32,
                            col_index as i32,
                            &engine,
                            &predicate,
                        );
                        if adjacency.is_adjacent {
                            symbol_adjacency_info = adjacency;
                        }
                        digit.push(*ch);
                    }
                    return acc;
                })
        })
        .collect();

    return result;
}

fn is_close(a: &Point, b: &Point) -> bool {
    return (a.x - b.x).abs() <= 2 && (a.y - b.y).abs() <= 2;
}

fn get_symbol_adjacency_info(
    row: i32,
    col: i32,
    engine: &Vec<Vec<char>>,
    predicate: impl Fn(char) -> bool,
) -> AdjacencyInfo {
    let offsets = get_offsets(row, col);

    let valid_coordinates = offsets
        .iter()
        .filter_map(|&(x, y)| {
            let x = x as usize;
            let y = y as usize;
            if x < engine.len() && y < engine.len() {
                if predicate(engine[y][x]) {
                    return Some((x as i32, y as i32));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let (is_adjacent, point) =
        valid_coordinates
            .into_iter()
            .next()
            .map_or((false, DEFAULT_POINT), |(x, y)| {
                return (
                    true,
                    Point {
                        x: col + x,
                        y: row + y,
                    },
                );
            });

    return AdjacencyInfo { is_adjacent, point };
}

fn get_offsets(row: i32, col: i32) -> Vec<(i32, i32)> {
    let offsets = Vec::from([
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ]);

    return offsets
        .iter()
        .map(|(x, y)| (x + col, y + row))
        .collect::<Vec<(i32, i32)>>();
}

fn is_symbol(ch: char) -> bool {
    return !ch.is_ascii_alphanumeric() && ch != '.';
}

fn is_gear(ch: char) -> bool {
    return ch == '*';
}
