use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, multispace0, multispace1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct GameNumbers<'a> {
    card: u32,
    winning: Vec<&'a str>,
    full: Vec<&'a str>,
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
            winning: left,
            full: right,
        },
    ))
}

fn part1(input: &str) -> &str {
    for line in input.lines() {
        let (_, game_numbers) = parse_line(line).unwrap();
        println!("{:?}", game_numbers);
        // println!("{:?}", asd);
    }

    ""
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
        assert_eq!(result, "asd");
    }
}
