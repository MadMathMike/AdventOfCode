mod parser;

use std::iter::repeat;
use std::time::Instant;

use crate::parser::parse_input;

fn main() {
    let start = Instant::now();

    let input = include_str!("../part1.txt");
    let records = parse_input(input);

    let sum = sum_valid_arrangement_counts(&records);

    dbg!(sum);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn sum_valid_arrangement_counts(records: &Vec<(&str, Vec<usize>)>) -> usize {
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

    count_valid_arrangements_recursive(
        mask, 
        "", 
        num_of_unassigned_working_springs,
        &damaged_segments, 
        0
    )
}

fn count_valid_arrangements_recursive(
    mask: &str, 
    springs_arrangement: &str, 
    num_of_unassigned_working_springs: usize,
    damaged_segments: &Vec<usize>, 
    depth: usize) 
    -> usize 
{
    if !matches(mask, springs_arrangement) {
        return 0;
    }        

    let mut next_springs_arrangement = String::from(springs_arrangement);

    if depth == damaged_segments.len() {
        next_springs_arrangement.push_str(&repeat('.').take(num_of_unassigned_working_springs).collect::<String>());

        return if matches(mask, &next_springs_arrangement) {
            1
        } else {
            0
        }
    }

    if depth > 0 {
        next_springs_arrangement.push_str(".");

        if !matches(mask, &next_springs_arrangement) {
            return 0;
        }
    }

    let mut valid_arrangement_count = 0;
    for i in 0..(num_of_unassigned_working_springs + 1) {
        let mut next_springs_arrangement = String::from(&next_springs_arrangement);
        next_springs_arrangement.push_str(&repeat('.').take(i).collect::<String>());
        
        if depth < damaged_segments.len() { 
            next_springs_arrangement.push_str(&repeat('#').take(damaged_segments[depth]).collect::<String>());
        }

        valid_arrangement_count += count_valid_arrangements_recursive(
            mask, 
            &next_springs_arrangement, 
            num_of_unassigned_working_springs - i, 
            damaged_segments, 
            depth + 1
        );
    }

    valid_arrangement_count
}

fn matches(mask: &str, arrangement: &str) -> bool {
    arrangement.is_empty() 
    || arrangement.char_indices()
        .all(|(i, char)| 
            mask.chars().nth(i) == Some('?') 
            || char.eq(&mask.chars().nth(i).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_arrangements_returns_1() {
        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#..", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 1);
    }

    #[test]
    fn count_valid_arrangements_returns_number_of_gaps() {
        let record = ("?#.??????#??#?#?#?#?", vec![1,1,15]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 2);
        
        let record = ("??????##????#?.?.??.", vec![1,7,4,1,2]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 4);
    }

    #[test]
    fn count_valid_arrangements_works_on_sample_records() {
        /*        
            ???.### 1,1,3 - 1 arrangement
            .??..??...?##. 1,1,3 - 4 arrangements
            ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
            ????.#...#... 4,1,1 - 1 arrangement
            ????.######..#####. 1,6,5 - 4 arrangements
            ?###???????? 3,2,1 - 10 arrangements
        */

        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 1);

        let record = (".??..??...?##.", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 4);

        let record = ("?#?#?#?#?#?#?#?", vec![1,3,1,6]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#...#...", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 1);

        let record = ("????.######..#####.", vec![1,6,5]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 4);

        let record = ("?###????????", vec![3,2,1]);
        let arrangement_count = count_valid_arrangements(&record);
        assert_eq!(arrangement_count, 10);
    }

    #[test]
    fn matches_returns_true() {
        let mask = "?###????????";
        
        let arrangement = ".###.##.#...";
        assert!(matches(mask, arrangement));
        
        let arrangement = ".###..##...#";
        assert!(matches(mask, arrangement));
    }

    #[test]
    fn sum_valid_arrangement_counts_works_on_sample() {
        let input = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let records = parse_input(&input);

        let sum = sum_valid_arrangement_counts(&records);

        assert_eq!(sum, 21);
    }
}