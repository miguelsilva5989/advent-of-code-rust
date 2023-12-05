use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    map: Vec<Vec<Vec<usize>>>,
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
                let digits: Vec<usize> = line.split(" ").into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
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

fn transpose_vec(v: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<usize>>())
        .collect()
}

fn find_location(seed: usize, maps: Vec<Vec<Vec<usize>>>) -> usize {
    let location = maps.iter().fold(seed, |source, map| {
        let transposed = transpose_vec(map.clone());
        // println!("-> seed {} source {} tranposed {:?}", source, source, transposed);

        let mut destination = transposed[1]
            .iter()
            .enumerate()
            .map(|(idx, dest)| {
                if (dest..&(dest + transposed[2][idx])).contains(&&source) {
                    // println!("  found source {} between {} {}", source, dest, &(dest + &transposed[2][idx]));
                    // get destination value
                    let offset = source - (dest..&(dest + &transposed[2][idx])).start;
                    return transposed[0][idx] + offset;
                }
                return 0;
            })
            .max();
        if destination.unwrap() == 0 {
            destination = Some(source)
        };
        // println!("  - destination {:?}", destination);
        destination.unwrap()
    });

    location
}

fn part1(input: &str) -> usize {
    let input = input.replace("seeds: ", "seeds: \n");

    let almanac = parse_almanac(input);
    println!("{:?}", almanac);

    let seeds: Vec<Range<usize>> = almanac.seeds.chunks(2).map(|x| x[0]..(x[0] + x[1])).collect();
    println!("{:?}", seeds);

    // seeds
    //     .iter()
    //     .flat_map(|range| range.clone().into_iter())
    //     .map(|seed| println!("{seed}"))
    //     .min()
    //     .unwrap();

    // panic!();

    let min_location = seeds
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .map(|seed| find_location(seed, almanac.map.clone()))
        .min()
        .unwrap();
    min_location
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
        assert_eq!(result, 46);
    }
}
