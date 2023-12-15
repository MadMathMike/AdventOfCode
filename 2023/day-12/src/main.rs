mod parser;

use std::time::Instant;

use crate::parser::*;
use day_12::*;

fn main() {
    let start = Instant::now();

    let input = include_str!("../part1.txt");
    let records = parse_input(input);

    let sample_input = include_str!("../sample.txt");
    let sample_records = parse_input(sample_input);

    let part1_sample_result = part1(&sample_records);
    dbg!(part1_sample_result);
    assert_eq!(part1_sample_result, 21);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    let part1_result = part1(&records);
    dbg!(part1_result);
    assert_eq!(part1_result, 8193);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    let part2_sample_result = part2(&sample_records);
    dbg!(part2_sample_result);
    assert_eq!(part2_sample_result, 525152);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    // let part2_result = part2(&records);
    // dbg!(part2_result);
    // //assert_eq!(part2_result, xxx);
    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
    
}

fn part1(records: &Vec<(&str, Vec<usize>)>) -> usize {
    records.iter()
        .map(|record| count_valid_arrangements(record.0, &record.1))
        .sum()
}

fn part2(records: &Vec<(&str, Vec<usize>)>) -> usize {
    let unfolded_records = records.iter()
        .map(|record| unfold(record.0, &record.1));

    let sum_part2: usize = unfolded_records
        .map(|(mask, damaged_segments)| 
            count_valid_arrangements(&mask, &damaged_segments))
        .sum();
    
    sum_part2
}