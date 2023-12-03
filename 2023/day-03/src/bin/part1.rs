fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum PositionType {
    Dot,
    Digit(u32),
    Symbol,
}

fn has_adjacent_symbols(engine: &Vec<Vec<PositionType>>, i: usize, j: usize) -> bool {
    let rows = engine.len();
    let cols = engine[0].len();
    // while i <= rows {
    //     while j <= cols {}
    // }

    for dx in (if i > 0 { -1 } else { 0 })..(if i < rows { 2 } else { 1 }) {
        for dy in (if j > 0 { -1 } else { 0 })..(if j < cols { 2 } else { 1 }) {
            if engine[dx][dy] == PositionType::Symbol {
                println!("symbol");
            }
        }
    }

    false
}

fn part1(input: &str) -> u32 {
    let mut engine: Vec<Vec<PositionType>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<PositionType> = Vec::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                row.push(PositionType::Digit(ch.to_digit(10).unwrap()))
            } else if ch == '.' {
                row.push(PositionType::Dot)
            } else {
                row.push(PositionType::Symbol)
            }
        }
        engine.push(row)
    }

    let mut engine_numbers = 0;
    for (i, row) in engine.iter().enumerate() {
        let mut is_continuous_nr = false;
        for (j, x) in row.iter().enumerate() {
            // if j + 1 <= engine[i].len() && matches!(engine[i][j + 1], PositionType::Digit(_)) {
            //     is_continuous_nr = true;
            // }

            println!("{:?}{:?}", i, j);
            if has_adjacent_symbols(&engine, i, j) {}
        }
    }

    9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
