use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    map: Vec<Vec<Vec<i64>>>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: vec![],
            map: vec![vec![]],
        }
    }
}

fn parse_almanac(input: String) -> Almanac {
    let mut almanac: Almanac = Almanac::new();
    let mut i = 0;
    for line in input.lines() {
        if let Some(map_title) = line.chars().nth(0) {
            if map_title.is_alphabetic() {
                i += 1;
            } else {
                let digits: Vec<i64> = line.split(" ").into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
                println!("{} {:?} {:?}", i, digits, almanac);
                if i == 1 {
                    almanac.seeds = digits;
                } else {
                    if almanac.map.len() <= i - 2 {
                        almanac.map.push(vec![digits]);
                    } else {
                        almanac.map[i - 2].push(digits);
                    }
                }
            }
        }
    }
    almanac
}

fn transpose_vec(v: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<i64>>())
        .collect()
}

// struct Link {
//     source: i64,
//     dest: i64,
// }

fn find_location(source: i64, maps: Vec<Vec<Vec<i64>>>) {
    for (i, map) in maps.iter().enumerate() {
        println!("seed: {}, map {:?}", source, map);

        let transposed = transpose_vec(maps[i].clone());
        // println!("check seed {} in {:?}", source, transposed);

        let mut destination = transposed[1]
            .iter()
            .enumerate()
            .map(|(idx, seed)| {
                if (seed..&(seed + &transposed[2][idx])).contains(&&source) {
                    // println!("  found seed {} between {} {}", source, seed, &(seed + &transposed[2][idx]));
                    // get destination value
                    let offset = source as usize - (*seed as usize..(seed + &transposed[2][idx]) as usize).start;
                    // println!("  pos {}", pos);
                    // println!("  val {}", transposed[0][idx] as usize + pos);
                    return transposed[0][idx] as usize + offset;
                }
                return 0;
            })
            .max();

        if destination.unwrap() == 0 {
            destination = Some(source as usize)
        };
        println!("  destination {:?}", destination);
        continue;
    }

    // dest.unwrap() as i64
}

fn part1(input: &str) -> i64 {
    let input = input.replace("seeds: ", "seeds: \n");

    let almanac = parse_almanac(input);
    println!("{:?}", almanac);

    // let mut map: BTreeMap<i64, Link> = BTreeMap::new();
    for seed in almanac.seeds {
        find_location(seed, almanac.map.clone())
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 35);
    }
}
