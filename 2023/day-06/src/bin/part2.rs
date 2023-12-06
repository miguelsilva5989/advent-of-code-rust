use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0},
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_text(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (_, _, time, _, _, _, distance)) = tuple((
        tag("Time:"),
        multispace0,
        digit1,
        line_ending,
        tag("Distance:"),
        multispace0,
        digit1,
    ))(input)?;

    Ok((input, (time.parse::<usize>().unwrap(), distance.parse::<usize>().unwrap())))
}

fn part1(input: &str) -> usize {
    let (_, (time, record)) = parse_text(input).unwrap();
    println!("{:?} {:?}", time, record);

    let mut beat_record = 0;
    for ms in 0..=time {
        if (time - ms) * (ms) > record {
            beat_record += 1;
        }
    }

    beat_record
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "Time:      71530
Distance:  940200",
        );
        assert_eq!(result, 71503);
    }
}
