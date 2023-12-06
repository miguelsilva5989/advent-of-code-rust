use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_text(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (_, _, time, _, _, _, distance)) = tuple((
        tag("Time:"),
        multispace0,
        separated_list1(space1, digit1),
        line_ending,
        tag("Distance:"),
        multispace0,
        separated_list1(space1, digit1),
    ))(input)?;

    Ok((
        input,
        (
            time.iter().map(|x| x.parse::<u32>().unwrap()).collect(),
            distance.iter().map(|x| x.parse::<u32>().unwrap()).collect(),
        ),
    ))
}

fn part1(input: &str) -> u32 {
    let (_, (times, records)) = parse_text(input).unwrap();
    println!("{:?} {:?}", times, records);

    let mut ways: Vec<u32> = vec![];
    for (idx, time) in times.into_iter().enumerate() {
        let mut beat_record = 0;
        for ms in 0..=time {
            if (time - ms) * (ms) > records[idx] {
                beat_record += 1;
            }
        }
        ways.push(beat_record)
    }
    // println!("ways {:?}", ways);

    ways.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
