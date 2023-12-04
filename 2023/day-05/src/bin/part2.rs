fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let result = part2("");
        assert_eq!(result, "asd");
    }
}
