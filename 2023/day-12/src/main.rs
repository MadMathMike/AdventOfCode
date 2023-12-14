mod parser;

use std::time::Instant;
use crate::parser::parse_input;

fn main() {
    let start = Instant::now();

    let input = include_str!("../part1.txt");
    let records = parse_input(input);

    let sum = sum_valid_arrangment_counts(&records);

    dbg!(sum);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn sum_valid_arrangment_counts(records: &Vec<(&str, Vec<usize>)>) -> usize {
    records.iter()
        .map(count_valid_arrangements)
        .sum()
}

fn count_valid_arrangements(record: &(&str, Vec<usize>)) -> usize {
    let (mask, damaged_segments) = record;

    let num_of_segment_gaps = damaged_segments.len() - 1;
    let num_of_damaged_springs:usize = damaged_segments.iter().sum();
    let num_of_unassigned_working_springs = 
        mask.len() 
        - num_of_segment_gaps // Each segment gap must have at least one working spring
        - num_of_damaged_springs;

    if num_of_unassigned_working_springs == 0 {
        return 1;
    }

    if num_of_unassigned_working_springs == 1 {
        return num_of_segment_gaps;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_arrangements_returns_1() {
        let record = ("???.###", vec![1,1,3]);
        let arrangment_count = count_valid_arrangements(&record);
        assert_eq!(arrangment_count, 1);

        let record = ("????.#..", vec![4,1,1]);
        let arrangment_count = count_valid_arrangements(&record);
        assert_eq!(arrangment_count, 1);
    }

    #[test]
    fn count_valid_arrangements_returns_number_of_gaps() {
        let record = ("?#.??????#??#?#?#?#?", vec![1,1,15]);
        let arrangment_count = count_valid_arrangements(&record);
        assert_eq!(arrangment_count, 2);
        
        let record = ("??????##????#?.?.??.", vec![1,7,4,1,2]);
        let arrangment_count = count_valid_arrangements(&record);
        assert_eq!(arrangment_count, 4);
    }

    #[test]
    fn sum_valid_arrangment_counts_returns_sum() {
        let records = vec![
            ("???.###", vec![1,1,3]),
            ("????.#..", vec![4,1,1])
        ];

        let sum = sum_valid_arrangment_counts(&records);

        assert_eq!(sum, 2);
    }
}