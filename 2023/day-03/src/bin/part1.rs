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

fn has_adjacent_symbols(
    engine: &Vec<Vec<PositionType>>,
    rows_len: usize,
    cols_len: usize,
    i: usize,
    j: usize,
    ongoing_number: &mut String,
) -> bool {
    if let PositionType::Digit(num) = engine[i][j] {
        ongoing_number.push_str(num.to_string().as_str());
    }

    for dx in (if i > 0 { i - 1 } else { i })..=(if i < rows_len { i + 1 } else { i }) {
        for dy in (if j > 0 { j - 1 } else { j })..=(if j < cols_len { j + 1 } else { j }) {
            if dx == i && dy == j {
                continue;
            }
            // println!("  pos {:?}-{:?}: number {:?}", dx, dy, engine[dx][dy]);

            if engine[dx][dy] == PositionType::Symbol {
                // println!("      pos {:?}-{:?}    type {:?}", dx, dy, engine[dx][dy]);
                return true;
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

    let rows_len = engine.len() - 1;
    let cols_len = engine[0].len() - 1;
    let mut ongoing_number = "".to_owned();
    let mut is_valid_number: bool = false;
    let mut engine_numbers = 0;

    for (i, row) in engine.iter().enumerate() {
        // let mut is_continuous_nr = false;
        for (j, x) in row.iter().enumerate() {
            if x == &PositionType::Dot {
                if ongoing_number != "" && is_valid_number {
                    engine_numbers += ongoing_number.parse::<u32>().unwrap();
                }
                is_valid_number = false;
                ongoing_number = "".to_owned();
            }

            if matches!(x, PositionType::Digit(_))
                && has_adjacent_symbols(&engine, rows_len, cols_len, i, j, &mut ongoing_number)
            {
                is_valid_number = true;
            }
        }
    }

    engine_numbers
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
