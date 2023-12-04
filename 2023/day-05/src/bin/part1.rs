fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1("");
        assert_eq!(result, "asd");
    }
}
