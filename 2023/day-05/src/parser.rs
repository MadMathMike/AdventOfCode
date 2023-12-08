pub struct FarmMaps {
    pub seeds: Vec<i64>,
    pub seed_to_soil_map: Vec<MapEntry>,
    pub soil_to_fertilizer_map: Vec<MapEntry>,
    pub fertilizer_to_water_map: Vec<MapEntry>,
    pub water_to_light_map: Vec<MapEntry>,
    pub light_to_temperature_map: Vec<MapEntry>,
    pub temperature_to_humidity_map: Vec<MapEntry>,
    pub humidity_to_location_map: Vec<MapEntry>
}

impl FarmMaps {
    pub fn collect(self: &Self) -> Vec<&Vec<MapEntry>> {
        vec![
            &self.seed_to_soil_map,
            &self.soil_to_fertilizer_map,
            &self.fertilizer_to_water_map,
            &self.water_to_light_map,
            &self.light_to_temperature_map,
            &self.temperature_to_humidity_map,
            &self.humidity_to_location_map
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapEntry {
    pub source_start: i64,
    pub destination_start: i64,
    pub range: i64,
}

impl MapEntry {
    pub fn new(destination_start: i64, source_start: i64, range: i64) -> MapEntry {
        MapEntry {destination_start, source_start, range,}
    }
}

pub fn parse_input(input: &str) -> FarmMaps {
    let mut lines = input.lines().filter(|l| !l.is_empty());

    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|part| part.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut header = lines.next().unwrap();

    assert_eq!(header, "seed-to-soil map:");
    let mut seed_to_soil_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            seed_to_soil_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    seed_to_soil_map.sort();

    assert_eq!(header, "soil-to-fertilizer map:");
    let mut soil_to_fertilizer_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            soil_to_fertilizer_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    soil_to_fertilizer_map.sort();

    assert_eq!(header, "fertilizer-to-water map:");
    let mut fertilizer_to_water_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            fertilizer_to_water_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    fertilizer_to_water_map.sort();

    assert_eq!(header, "water-to-light map:");
    let mut water_to_light_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            water_to_light_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    water_to_light_map.sort();

    assert_eq!(header, "light-to-temperature map:");
    let mut light_to_temperature_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            light_to_temperature_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    light_to_temperature_map.sort();

    assert_eq!(header, "temperature-to-humidity map:");
    let mut temperature_to_humidity_map = Vec::<MapEntry>::new();

    loop {
        let line = lines.next().unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            temperature_to_humidity_map.push(parse_line(line));
        } else {
            header = line;
            break;
        }
    }
    temperature_to_humidity_map.sort();

    assert_eq!(header, "humidity-to-location map:");
    let mut humidity_to_location_map = Vec::<MapEntry>::new();

    loop {
        let next_line = lines.next();

        if next_line == None {
            break;
        }

        let line = next_line.unwrap();

        if line.chars().nth(0).unwrap().is_numeric() {
            humidity_to_location_map.push(parse_line(line));
        } else {
            panic!("What?!");
        }
    }
    humidity_to_location_map.sort();

    FarmMaps {
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map
    }
}

fn parse_line(line: &str) -> MapEntry {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let destination_start = parts[0].parse::<i64>().unwrap();
    let source_start = parts[1].parse::<i64>().unwrap();
    let range = parts[2].parse::<i64>().unwrap();

    MapEntry::new(destination_start, source_start, range)
}

#[cfg(test)]
mod tests {
    use assertx::assert_contains_exactly;

    use super::*;

    #[test]
    fn parses_sample_input() {
        let sample_input = include_str!("../sample_input.txt");

        let farm_maps = parse_input(sample_input);

        assert_eq!(farm_maps.seeds.len(), 4);
        assert_contains_exactly!(farm_maps.seeds, vec![79i64, 14i64, 55i64, 13i64]);

        assert_eq!(farm_maps.seed_to_soil_map.len(), 2);
        assert_contains_exactly!(
            farm_maps.seed_to_soil_map,
            vec![MapEntry::new(52, 50, 48), MapEntry::new(50, 98, 2)]
        );

        assert_eq!(farm_maps.soil_to_fertilizer_map.len(), 3);
        assert_contains_exactly!(
            farm_maps.soil_to_fertilizer_map,
            vec![MapEntry::new(39, 0, 15), MapEntry::new(0, 15, 37), MapEntry::new(37, 52, 2)]
        );

        assert_eq!(farm_maps.fertilizer_to_water_map.len(), 4);
        assert_contains_exactly!(
            farm_maps.fertilizer_to_water_map,
            vec![MapEntry::new(42, 0, 7), MapEntry::new(57, 7, 4), MapEntry::new(0, 11, 42), MapEntry::new(49, 53, 8)]
        );

        assert_eq!(farm_maps.water_to_light_map.len(), 2);
        assert_contains_exactly!(
            farm_maps.water_to_light_map,
            vec![MapEntry::new(88, 18, 7), MapEntry::new(18, 25, 70)]
        );

        assert_eq!(farm_maps.light_to_temperature_map.len(), 3);
        assert_contains_exactly!(
            farm_maps.light_to_temperature_map,
            vec![MapEntry::new(81, 45, 19), MapEntry::new(68, 64, 13), MapEntry::new(45, 77, 23)]
        );

        assert_eq!(farm_maps.temperature_to_humidity_map.len(), 2);
        assert_contains_exactly!(
            farm_maps.temperature_to_humidity_map,
            vec![MapEntry::new(1, 0, 69), MapEntry::new(0, 69, 1)]
        );

        assert_eq!(farm_maps.humidity_to_location_map.len(), 2);
        assert_contains_exactly!(
            farm_maps.humidity_to_location_map,
            vec![MapEntry::new(60, 56, 37), MapEntry::new(56, 93, 4)]
        );
    }
}
