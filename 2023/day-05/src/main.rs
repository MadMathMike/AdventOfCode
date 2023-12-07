fn main() {
    let part1_input = include_str!("./part1.txt");
    let (seeds, maps) = parse_input(part1_input);
    
    let result = part1(&seeds, &maps);
    dbg!(result);

    use std::time::Instant;
    let now = Instant::now();

    let result = part2(&seeds, &maps);
    dbg!(result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn part2(seeds: &Vec<i64>, maps: &[Vec<RangeMap>; 7]) -> i64 {
    let mut seeds_iter = seeds.iter();
    let mut smallest_location = i64::MAX;
    let mut seed_candidate = seeds_iter.next();
    while seed_candidate != None {
        let seed_start = seed_candidate.unwrap();
        let range = seeds_iter.next().unwrap();
        let stop = seed_start + range;

        for seed in *seed_start .. stop {
            let location = traverse_maps(maps, seed);
            if location < smallest_location {
                smallest_location = location;
            }
        }

        seed_candidate = seeds_iter.next();
    }

    smallest_location
}

fn part1(seeds: &Vec<i64>, maps: &[Vec<RangeMap>; 7]) -> i64 {
    seeds.iter()
        .map(|seed| traverse_maps(maps, *seed))
        .min().unwrap()
}

fn parse_input(input: &str) -> (Vec<i64>, [Vec<RangeMap>; 7]) {
    let mut lines = input.lines().filter(|l| !l.is_empty());

    let seeds = lines
        .next().unwrap()
        .split_once(' ').unwrap()
        .1
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap());

    let mut line = lines.next().unwrap();

    // get seed-to-soil map
    assert_eq!(line, "seed-to-soil map:");
    let mut seed_to_soil = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "soil-to-fertilizer map:" {
        seed_to_soil.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get soil-to-fertilizer map
    assert_eq!(line, "soil-to-fertilizer map:");
    let mut soil_to_fertilizer = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "fertilizer-to-water map:" {
        soil_to_fertilizer.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get fertilizer-to-water map
    assert_eq!(line, "fertilizer-to-water map:");
    let mut fertilizer_to_water = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "water-to-light map:" {
        fertilizer_to_water.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get water-to-light map
    assert_eq!(line, "water-to-light map:");
    let mut water_to_light = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "light-to-temperature map:" {
        water_to_light.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get light-to-temperature map
    assert_eq!(line, "light-to-temperature map:");
    let mut light_to_temperature = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "temperature-to-humidity map:" {
        light_to_temperature.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get temperature-to-humidity map
    assert_eq!(line, "temperature-to-humidity map:");
    let mut temperature_to_humidity = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "humidity-to-location map:" {
        temperature_to_humidity.push(parse_range_map(line));

        line = lines.next().unwrap();
    }

    // get humidity-to-location map
    assert_eq!(line, "humidity-to-location map:");
    let mut humidity_to_location = Vec::<RangeMap>::new();

    line = lines.next().unwrap();
    while line != "humidity-to-location map:" {
        humidity_to_location.push(parse_range_map(line));

        let next_line_candidate = lines.next();
        if next_line_candidate == None {
            break;
        }

        line = next_line_candidate.unwrap();
    }

    let maps = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location
    ];

    (seeds.collect(), maps.map(collapse_maps))
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct RangeMap {
    start_inclusive: i64,
    end_exclusive: i64,
    addend: i64
}

fn parse_range_map(input: &str) -> RangeMap {
    let mut input_parts = input.split(' ');
    let target_start = input_parts.next().unwrap().parse::<i64>().unwrap();
    let source_start = input_parts.next().unwrap().parse::<i64>().unwrap();
    let range = input_parts.next().unwrap().parse::<i64>().unwrap();

    RangeMap { start_inclusive: source_start, end_exclusive: source_start + range, addend: target_start - source_start }
}

fn collapse_maps(mut range_maps: Vec<RangeMap>) -> Vec<RangeMap> {
    range_maps.sort_by(|a, b| a.start_inclusive.cmp(&b.start_inclusive));

    range_maps
        .iter()
        .fold(Vec::<RangeMap>::new(), |mut acc, range_map|{
            if acc.len() == 0 {
                acc.push(*range_map);

                return acc;
            }

            let previous = acc.last().unwrap();

            if previous.end_exclusive == range_map.start_inclusive && previous.addend == range_map.addend {
                // pop the old one off the list
                let previous = acc.remove(acc.len() - 1);

                // collapse the two
                let collapsed = collapse(&previous, range_map);

                // add the new collapsed one back on
                acc.push(collapsed);
            } else {
                acc.push(*range_map);
            }

            acc
        })
}

fn collapse(first: &RangeMap, second: &RangeMap) -> RangeMap {
    RangeMap { start_inclusive: first.start_inclusive, end_exclusive: second.end_exclusive, addend: first.addend }
}

fn find_addend(range_maps: &Vec<RangeMap>, input: i64) -> i64 {
    for range_map in range_maps.iter() {
        if input >= range_map.start_inclusive && input < range_map.end_exclusive {
            return range_map.addend;
        }
    }

    return 0;
}

fn traverse_maps(maps_array: &[Vec<RangeMap>], start: i64) -> i64 {
    let mut return_val = start;
    for maps in maps_array.iter() {
        let addend = find_addend(&maps, return_val);
        return_val += addend;
    }

    return_val
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn can_parse_range_map() {
        let result = parse_range_map("50 98 2");
        assert_eq!(result, RangeMap { start_inclusive: 98, end_exclusive: 100, addend: -48});
    }

    #[test]
    fn finds_correct_addend() {
        let mut maps = Vec::<RangeMap>::new();
        maps.push(parse_range_map("50 98 2"));
        maps.push(parse_range_map("52 50 48"));
        
        let addend = find_addend(&maps, 79);
        assert_eq!(addend, 2);

        let addend = find_addend(&maps, 14);
        assert_eq!(addend, 0);

        let addend = find_addend(&maps, 55);
        assert_eq!(addend, 2);

        let addend = find_addend(&maps, 13);
        assert_eq!(addend, 0);
    }

    #[test]
    fn correctly_collapses_maps() {
        let mut soil_to_fertilizer = Vec::<RangeMap>::new();
        soil_to_fertilizer.push(parse_range_map("0 15 37"));
        soil_to_fertilizer.push(parse_range_map("37 52 2"));
        soil_to_fertilizer.push(parse_range_map("39 0 15"));

        let result = collapse_maps(soil_to_fertilizer);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], RangeMap {start_inclusive: 0, end_exclusive: 15, addend: 39});
        assert_eq!(result[1], RangeMap {start_inclusive: 15, end_exclusive: 54, addend: -15});
    }

    #[test]
    fn traverses_maps() {
        let mut seed_to_soil = Vec::<RangeMap>::new();
        seed_to_soil.push(parse_range_map("50 98 2"));
        seed_to_soil.push(parse_range_map("52 50 48"));

        let mut soil_to_fertilizer = Vec::<RangeMap>::new();
        soil_to_fertilizer.push(parse_range_map("0 15 37"));
        soil_to_fertilizer.push(parse_range_map("37 52 2"));
        soil_to_fertilizer.push(parse_range_map("39 0 15"));

        let mut fertilizer_to_water = Vec::<RangeMap>::new();
        fertilizer_to_water.push(parse_range_map("49 53 8"));
        fertilizer_to_water.push(parse_range_map("0 11 42"));
        fertilizer_to_water.push(parse_range_map("42 0 7"));
        fertilizer_to_water.push(parse_range_map("57 7 4"));

        let maps_array = [
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water
        ];

        let destination = traverse_maps(&maps_array, 79);
        assert_eq!(destination, 81);

        let destination = traverse_maps(&maps_array, 14);
        assert_eq!(destination, 49);

        let destination = traverse_maps(&maps_array, 55);
        assert_eq!(destination, 53);

        let destination = traverse_maps(&maps_array, 13);
        assert_eq!(destination, 41);
    }

    #[test]
    fn part1_finds_lowest_location() {
        let input = 
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
56 93 4";

        let (seeds, maps) = parse_input(input);
        let result = part1(&seeds, &maps);
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_finds_lowest_location() {
        let input = 
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
56 93 4";

        let (seeds, maps) = parse_input(input);
        let result = part2(&seeds, &maps);
        assert_eq!(result, 46);
    }
}
