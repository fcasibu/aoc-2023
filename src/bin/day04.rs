mod utils;

struct Card {
    index: usize,
    matches: i32,
}

fn main() {
    println!("Part1: {}, Part2: {}", part1(), part2());
}

fn part1() -> i32 {
    let cards = get_cards();

    return cards.iter().fold(0, |acc, card| {
        if let Some(card) = card {
            if card.matches == 0 {
                return acc;
            }

            return acc + i32::pow(2, (card.matches - 1) as u32);
        }

        return acc;
    });
}

fn part2() -> i32 {
    let cards = get_cards();
    let mut copies_of_scratch_cards: Vec<&Card> = Vec::new();

    for card in &cards {
        if let Some(card) = card {
            copies_of_scratch_cards.push(card);

            for copy in copies_of_scratch_cards.clone() {
                if copy.index != card.index {
                    continue;
                }

                let start_index = copy.index + 1;
                let end_index = start_index + copy.matches as usize;

                for i in start_index..end_index {
                    if let Some(card) = &cards[i] {
                        copies_of_scratch_cards.push(card);
                    }
                }
            }
        }
    }

    return copies_of_scratch_cards.len() as i32;
}

fn get_cards() -> Vec<Option<Card>> {
    return utils::read_lines(|line, index| {
        if let Some(numbers) = line.split(": ").collect::<Vec<&str>>().last() {
            let numbers = numbers
                .split(" | ")
                .map(|nums| {
                    return nums
                        .split_whitespace()
                        .map(|num| num.parse::<i32>().unwrap_or(0))
                        .collect::<Vec<i32>>();
                })
                .collect::<Vec<Vec<i32>>>();

            if let Some(scratch_cards) = numbers.last() {
                let matches = scratch_cards
                    .iter()
                    .filter(|num| {
                        if let Some(winning_numbers) = numbers.first() {
                            return winning_numbers.contains(num);
                        }
                        return false;
                    })
                    .count();

                return Some(Card {
                    matches: matches as i32,
                    index,
                });
            }
        };

        return None;
    });
}
