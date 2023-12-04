use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct GameNumbers {
    card: u32,
    // copies: u32,
    winning: Vec<u32>,
    full: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, card_numbers) = separated_pair(
        separated_list1(multispace1, digit1),
        tag(" | "),
        separated_list1(multispace1, digit1),
    )(input)?;

    Ok((input, (card_numbers.0, card_numbers.1)))

    // Ok((input))?
}

fn parse_line(input: &str) -> IResult<&str, GameNumbers> {
    let (input, (_, _, card_number, _, _, (left, right))) =
        tuple((tag("Card"), multispace0, digit1, tag(":"), multispace0, parse_card))(input)?;

    Ok((
        input,
        GameNumbers {
            card: card_number.parse::<u32>().unwrap(),
            // copies: 1,
            winning: left.iter().map(|x| x.parse::<u32>().unwrap()).collect(),
            full: right.iter().map(|x| x.parse::<u32>().unwrap()).collect(),
        },
    ))
}

fn count_winning_numbers(card: &GameNumbers) -> u32 {
    let mut winning_sum = 0;
    for winning in &card.winning {
        if card.full.contains(&winning) {
            winning_sum += 1;
        }
    }
    winning_sum
}

fn part1(input: &str) -> usize {
    let mut games: Vec<GameNumbers> = vec![];
    let mut hash: BTreeMap<u32, usize> = BTreeMap::new();

    for line in input.lines() {
        let bind = line;
        let line = bind.replace("  ", " ");
        let (_, game_numbers) = parse_line(line.as_str()).unwrap();
        // println!("{:?}", game_numbers);
        hash.entry(game_numbers.card).or_insert(1);
        games.push(game_numbers);
    }

    for game in games.clone() {
        // println!("{:?}", game);
        let winning_number = count_winning_numbers(&game);
        if winning_number > 0 {
            for _ in 1..=*hash.get(&game.card).unwrap() {
                for i in 1..=winning_number {
                    hash.entry(game.card + i).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }
    }

    hash.iter().map(|(_, values)| values).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
