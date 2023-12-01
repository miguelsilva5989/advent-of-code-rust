fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut nums: String = "".to_string();
        for ch in line.chars() {
            if ch.is_digit(10) {
                nums.push(ch);
                break;
            }
        }
        for ch in line.chars().rev() {
            if ch.is_digit(10) {
                nums.push(ch);
                break;
            }
        }
        sum += nums.parse::<u32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
