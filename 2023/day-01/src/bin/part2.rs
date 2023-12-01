fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

static NUMBERS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn match_num(num: &str) -> Option<char> {
    match num {
        "one" => return Some('1'),
        "two" => return Some('2'),
        "three" => return Some('3'),
        "four" => return Some('4'),
        "five" => return Some('5'),
        "six" => return Some('6'),
        "seven" => return Some('7'),
        "eight" => return Some('8'),
        "nine" => return Some('9'),
        _ => return None,
    }
}

#[derive(PartialEq)]
enum Direction {
    Normal,
    Reverse,
}

fn find_written_num(line: &str, direction: Direction) -> (usize, &str) {
    let mut pos = 9999;
    let mut written_num = "";
    if direction == Direction::Reverse {
        pos = 0;
    }

    for num in NUMBERS {
        match direction {
            Direction::Normal => {
                if let Some(found) = line.find(num) {
                    if found <= pos {
                        pos = found;
                        written_num = num;
                    }
                };
            }
            Direction::Reverse => {
                if let Some(found) = line.rfind(num) {
                    if found > pos {
                        pos = found;
                        written_num = num;
                    }
                };
            }
        }
    }
    return (pos, written_num);
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        println!("{line}");
        let mut nums: String = "".to_string();

        let left_digit = line.find(|c| char::is_digit(c, 10));
        let right_digit = line.rfind(|c| char::is_digit(c, 10));

        let (left_written_digit, written_num_left) = find_written_num(line, Direction::Normal);
        let (right_written_digit, written_num_right) = find_written_num(line, Direction::Reverse);

        if left_digit.is_some() && left_digit < Some(left_written_digit) {
            nums.push(line.chars().nth(left_digit.unwrap()).unwrap());
        } else {
            nums.push(match_num(written_num_left).unwrap());
        }

        if right_digit.is_some() && right_digit > Some(right_written_digit) {
            nums.push(line.chars().nth(right_digit.unwrap()).unwrap());
        } else {
            if let Some(right) = match_num(written_num_right) {
                nums.push(right)
            } else {
                nums.push(line.chars().nth(left_digit.unwrap()).unwrap());
            }
        }

        sum += nums.parse::<u32>().unwrap();
        // println!("input line {idx} - {sum} -- {nums}");
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
