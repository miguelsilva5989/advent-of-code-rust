fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

static NUMBER: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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

fn get_written_number(input_line: &str, current_line_pos: usize) -> Option<char> {
    let line = &input_line[current_line_pos..];

    for num in NUMBER {
        if num.len() <= line.len() && &line[..num.len()] == num {
            return match_num(num);
        }
    }
    None
}

fn get_written_number_reverse(input_line: &str, current_line_pos: usize) -> Option<char> {
    let rev_line = input_line.chars().rev().collect::<String>();
    let line = &rev_line[current_line_pos..];

    for num in NUMBER {
        if num.len() <= line.len() && &line[..num.len()] == num.chars().rev().collect::<String>() {
            return match_num(num);
        }
    }
    None
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for (idx, line) in input.lines().enumerate() {
        let mut nums: String = "".to_string();

        for (idx, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                nums.push(ch);
                break;
            }
            if let Some(written_num) = get_written_number(line, idx) {
                nums.push(written_num);
                break;
            }
        }
        for (idx, ch) in line.chars().rev().enumerate() {
            if ch.is_digit(10) {
                nums.push(ch);
                break;
            }
            if let Some(written_num) = get_written_number_reverse(line, idx) {
                nums.push(written_num);
                break;
            }
        }
        sum += nums.parse::<u32>().unwrap();
        println!("input line {idx} - {sum} -- {nums}")
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
