use itertools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_adjacent_numbers(symbol: &Pos, numbers: &mut BTreeMap<String, Vec<Pos>>) -> Vec<u32> {
    let mut symbol_numbers: Vec<u32> = vec![];
    for (number, number_positions) in numbers {
        for number_pos in number_positions {
            if (number_pos.row == symbol.row || number_pos.row == symbol.row - 1 || number_pos.row == symbol.row + 1)
                && (number_pos.col == symbol.col || number_pos.col == symbol.col - 1 || number_pos.col == symbol.col + 1)
            {
                symbol_numbers.push(number.parse::<u32>().unwrap());
            }
        }
    }
    symbol_numbers
}

#[derive(Debug, Clone)]
struct Pos {
    row: i32,
    col: i32,
}

fn part1(input: &str) -> u32 {
    let mut engine_sum = 0;

    let mut ongoing_number = "".to_owned();
    let mut ongoing_pos: Vec<Pos> = vec![];
    let mut hash: BTreeMap<String, Vec<Pos>> = BTreeMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                ongoing_number.push(ch);
                ongoing_pos.push(Pos {
                    row: row as i32,
                    col: col as i32,
                })
            } else if ch == '.' {
                if ongoing_number != "" {
                    hash.entry(format!("{ongoing_number}"))
                        .and_modify(|x| x.extend(ongoing_pos.clone()))
                        .or_insert(ongoing_pos.clone());
                    ongoing_number = "".to_owned();
                    ongoing_pos.clear();
                }
            } else {
                hash.entry("x".to_owned())
                    .and_modify(|x| {
                        x.push(Pos {
                            row: row as i32,
                            col: col as i32,
                        })
                    })
                    .or_insert(vec![Pos {
                        row: row as i32,
                        col: col as i32,
                    }]);

                if ongoing_number != "" {
                    hash.entry(format!("{ongoing_number}"))
                        .and_modify(|x| x.extend(ongoing_pos.clone()))
                        .or_insert(ongoing_pos.clone());
                    ongoing_number = "".to_owned();
                    ongoing_pos.clear();
                }
            }
        }
    }

    // println!("{:?}", hash);

    let symbols = hash.remove("x").unwrap();
    for symbol in symbols {
        let symbol_numbers = find_adjacent_numbers(&symbol, &mut hash);
        // println!("{:?}", symbol_numbers);

        let is_gear = symbol_numbers.iter().unique().count() == 2;
        if is_gear {
            engine_sum += symbol_numbers.iter().unique().product::<u32>();
        }
    }

    engine_sum
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
        assert_eq!(result, 467835);
    }
}
