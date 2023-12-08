mod intervals;
mod parser;
mod transform;

use crate::intervals::*;
use crate::parser::*;
use crate::transform::*;

fn main() {
    let input = include_str!("../part1.txt");

    let farm_maps = parse_input(input);
    assert_eq!(farm_maps.seeds.len(), 20);
    assert_eq!(farm_maps.seed_to_soil_map.len(), 17);
    assert_eq!(farm_maps.soil_to_fertilizer_map.len(), 9);
    assert_eq!(farm_maps.fertilizer_to_water_map.len(), 40);
    assert_eq!(farm_maps.water_to_light_map.len(), 24);
    assert_eq!(farm_maps.light_to_temperature_map.len(), 20);
    assert_eq!(farm_maps.temperature_to_humidity_map.len(), 44);
    assert_eq!(farm_maps.humidity_to_location_map.len(), 41);

    let transform_layers = convert_maps_to_transform_layers(&farm_maps.collect());

    let part1_result = part1(&farm_maps.seeds, &transform_layers);
    dbg!(part1_result);
    assert_eq!(part1_result, 806029445);

    let seed_intervals = map_each_seed_to_trivial_interval(&farm_maps.seeds);
    assert_eq!(seed_intervals.len(), 20);
    let part1_result = partx(&seed_intervals, &transform_layers);
    dbg!(part1_result);
    assert_eq!(part1_result, 806029445);

    let seed_intervals = map_seed_pairs_to_intervals(&farm_maps.seeds);
    assert_eq!(seed_intervals.len(), 10);
    let part2_result = partx(&seed_intervals, &transform_layers);
    dbg!(part2_result);
    assert_eq!(part2_result, 59370572);
}

fn part1(seeds: &Vec<i64>, transform_layers: &Vec<Vec<IntervalTransform>>) -> i64 {
    seeds.iter().map(|seed| {
        let mut location = *seed;
        for layer in transform_layers.iter() {
            location = apply_transform_layer_to_point(&layer, location);
        }
        
        location
    })
    .min()
    .unwrap()
}

fn partx(seed_intervals: &[Interval], transform_layers: &Vec<Vec<IntervalTransform>>) -> i64 {
    let mut location_intervals = seed_intervals
        .iter()
        .map(|interval|*interval)
        .collect::<Vec<Interval>>();

    for layer in transform_layers.iter() {
        location_intervals = apply_transform_layer_to_intervals(&layer, &location_intervals);

        assert!(location_intervals.len() > 0);
    }

    let part2_result = location_intervals.iter()
        .map(|location_interval| location_interval.0)
        .min()
        .unwrap();

    part2_result
}

fn map_seed_pairs_to_intervals(seeds: &[i64]) -> Vec<Interval> {
    (0..seeds.len()/2)
        .map(|i| {
            let index = i * 2;
            let start = seeds[index];
            let end = seeds[index] + seeds[index + 1] - 1;
            Interval(start, end)
        })
        .collect::<Vec<Interval>>()
}

fn to_interval_transform(map_entry: &MapEntry) -> IntervalTransform {
    IntervalTransform {
        interval: Interval(map_entry.source_start, map_entry.source_start + map_entry.range - 1),
        addend: map_entry.destination_start - map_entry.source_start
    }   
}

fn convert_maps_to_transform_layers(maps: &Vec<&Vec<MapEntry>>) -> Vec<Vec<IntervalTransform>> {
    let layers = maps
        .iter()
        .map(|m| 
            m.iter().map(to_interval_transform).collect::<Vec<IntervalTransform>>())
        .collect::<Vec<Vec<IntervalTransform>>>();

    for layer in layers.iter() {
        for i in 0..layer.len() - 1 {
            assert!(layer[i].interval.1 < layer[i+1].interval.0);
        }
    }

    layers
}

fn map_each_seed_to_trivial_interval(seeds: &Vec<i64>) -> Vec<Interval>{
    seeds.iter()
        .map(|seed| Interval(*seed, *seed + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_sample() {
        let sample_input = include_str!("../sample_input.txt");

        let farm_maps = parse_input(sample_input);
        let seeds = &farm_maps.seeds;
        let transformation_layers = convert_maps_to_transform_layers(&farm_maps.collect());

        let result = part1(&seeds, &transformation_layers);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_as_interval_with_sample() {
        let sample_input = include_str!("../sample_input.txt");

        let farm_maps = parse_input(sample_input);

        let seed_intervals = map_each_seed_to_trivial_interval(&farm_maps.seeds);
        let transformation_layers = convert_maps_to_transform_layers(&farm_maps.collect());

        let result = partx(&seed_intervals, &transformation_layers);

        assert_eq!(result, 35);
    }
    
    #[test]
    fn test_part2_with_sample() {
        let sample_input = include_str!("../sample_input.txt");

        let farm_maps = parse_input(sample_input);

        let seed_intervals = map_seed_pairs_to_intervals(&farm_maps.seeds);
        let transformation_layers = convert_maps_to_transform_layers(&farm_maps.collect());

        let result = partx(&seed_intervals, &transformation_layers);

        assert_eq!(result, 46);
    }

    #[test]
    fn to_interval_transform_works() {
        assert_eq!(
            to_interval_transform(&MapEntry::new(50, 98, 2)), 
            IntervalTransform{
                interval: Interval(98, 99),
                addend: -48
            }
        );

        assert_eq!(
            to_interval_transform(&MapEntry::new(52, 50, 48)), 
            IntervalTransform{
                interval: Interval(50, 97),
                addend: 2
            }
        );
    }
}