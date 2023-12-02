use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct GameColour {
    colour: Colour,
    num: usize,
}

#[derive(Debug)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn new() -> Self {
        GameSet { red: 0, green: 0, blue: 0 }
    }
}

#[derive(Debug)]
struct Games {
    num: usize,
    game_sets: Vec<GameSet>,
}

fn game_nr_colour(input: &str) -> IResult<&str, GameColour> {
    let (input, (_, num, _, colour)) = tuple((multispace0, digit1, multispace0, alt((tag("red"), tag("green"), tag("blue")))))(input)?;

    let colour = match colour {
        "red" => Colour::Red,
        "green" => Colour::Green,
        "blue" => Colour::Blue,
        _ => todo!(),
    };

    Ok((
        input,
        GameColour {
            colour,
            num: num.parse::<usize>().unwrap(),
        },
    ))
}

fn game_set(input: &str) -> IResult<&str, GameSet> {
    let (input, game_colours) = separated_list1(tag(","), game_nr_colour)(input)?;

    let mut game_set = GameSet::new();

    for game_colour in game_colours {
        match game_colour.colour {
            Colour::Red => game_set.red = game_colour.num,
            Colour::Green => game_set.green = game_colour.num,
            Colour::Blue => game_set.blue = game_colour.num,
        }
    }

    Ok((input, game_set))
}

fn parse_game_sets(input: &str) -> IResult<&str, Vec<GameSet>> {
    Ok(separated_list1(tag(";"), game_set)(input))?
}

fn parse_line(input: &str) -> IResult<&str, Games> {
    let (input, (_, _, game_number, _, game_sets)) = tuple((tag("Game"), multispace0, digit1, tag(":"), parse_game_sets))(input)?;
    Ok((
        input,
        Games {
            num: game_number.parse::<usize>().unwrap(),
            game_sets,
        },
    ))
}

fn check_cube_sum(games: Games) -> (usize, bool) {
    let mut game_set_sum = GameSet::new();

    for game_set in games.game_sets {
        game_set_sum.red += game_set.red;
        game_set_sum.green += game_set.green;
        game_set_sum.blue += game_set.blue;
    }

    println!("{:?}", game_set_sum);

    if game_set_sum.red <= 12 && game_set_sum.green <= 13 && game_set_sum.blue <= 14 {
        return (games.num, true);
    }

    (0, false)
}

fn part1(input: &str) -> usize {
    let mut game_numbers: usize = 0;
    for line in input.lines() {
        let (_, games) = parse_line(line).unwrap();
        let (game_num, result) = check_cube_sum(games);
        if result {
            game_numbers += game_num
        };
    }

    game_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
