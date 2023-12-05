use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_soil: Vec<Vec<u32>>,
    soil_to_fertilizer: Vec<Vec<u32>>,
    fertilizer_to_water: Vec<Vec<u32>>,
    water_to_light: Vec<Vec<u32>>,
    light_to_temperature: Vec<Vec<u32>>,
    temperature_to_humidity: Vec<Vec<u32>>,
    humidity_to_location: Vec<Vec<u32>>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: vec![],
            seed_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        }
    }
}

fn parse_almanac(input: String) -> Almanac {
    let mut almanac: Almanac = Almanac::new();
    let mut i = -1;
    for line in input.lines() {
        if let Some(map_title) = line.chars().nth(0) {
            if map_title.is_alphabetic() {
                i += 1;
            } else {
                let digits: Vec<u32> = line.split(" ").into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
                match i {
                    0 => almanac.seeds = digits,
                    1 => almanac.seed_soil.push(digits),
                    2 => almanac.soil_to_fertilizer.push(digits),
                    3 => almanac.fertilizer_to_water.push(digits),
                    4 => almanac.water_to_light.push(digits),
                    5 => almanac.light_to_temperature.push(digits),
                    6 => almanac.temperature_to_humidity.push(digits),
                    7 => almanac.humidity_to_location.push(digits),
                    _ => todo!(),
                }
            }
        }
    }
    almanac
}

fn transpose_vec(v: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<u32>>())
        .collect()
}

struct Link {
    source: u32,
    dest: u32,
}

fn find_location(source: u32, dest: Vec<Vec<u32>>) {
    let transposed = transpose_vec(dest);
    println!("check seed {} in {:?}", source, transposed);

    let dest = transposed[1]
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            if (x..&(x + &transposed[2][idx])).contains(&&source) {
                println!("  found seed {} between {} {}", source, x, &(x + &transposed[2][idx]));
                // get destination value
                let pos = (*x as usize..(x + &transposed[2][idx]) as usize)
                    .position(|x| x == source as usize)
                    .unwrap();
                // println!("  pos {}", pos);
                // println!("  val {}", transposed[0][idx] as usize + pos);
                return transposed[0][idx] as usize + pos;
            }
            return 0;
        })
        .min();
    println!("dest {:?}", dest)
}

fn part1(input: &str) -> u32 {
    let input = input.replace("seeds: ", "seeds: \n");

    let almanac = parse_almanac(input);
    println!("{:?}", almanac);

    let mut map: BTreeMap<u32, Link> = BTreeMap::new();
    for seed in almanac.seeds {
        find_location(seed, almanac.seed_soil.clone())
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
