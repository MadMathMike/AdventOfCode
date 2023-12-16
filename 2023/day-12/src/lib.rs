use std::iter::repeat;

pub fn count_valid_arrangements(mask: &str, damaged_segment_lengths: &[usize]) -> usize {
    // println!("Mask: {}", mask);
    // let start = std::time::Instant::now();

    let num_of_segment_gaps = damaged_segment_lengths.len() - 1;
    let num_of_damaged_springs:usize = damaged_segment_lengths.iter().sum();
    let num_of_unassigned_working_springs = 
        mask.len() 
        - num_of_segment_gaps // Each segment gap must have at least one working spring
        - num_of_damaged_springs;

    let mut damaged_segments = Vec::<String>::new();
    for i in 0..damaged_segment_lengths.len() {
        let segment_length = damaged_segment_lengths[i];

        let mut damaged_segment = repeat('#').take(segment_length).collect::<Vec<char>>();

        if i < damaged_segment_lengths.len() - 1 {
            damaged_segment.push('.');
        }

        let segment = damaged_segment.iter().collect::<String>();

        damaged_segments.push(segment);
    }

    let count = count_valid_arrangements_recursive(
        mask, 
        num_of_unassigned_working_springs,
        &damaged_segments
    );

    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);

    count
}

// TODO: MAY need to turn remaining_damaged_segments back into &[usize]
fn count_valid_arrangements_recursive(
    remaining_mask: &str,
    num_of_unassigned_working_springs: usize,
    remaining_damaged_segments: &[String]) 
    -> usize 
{
    if remaining_damaged_segments.len() == 0 {
        let mut arrangement_chunk = String::with_capacity(remaining_mask.len());
        repeat('.')
            .take(num_of_unassigned_working_springs)
            .for_each(|f| arrangement_chunk.push(f));

        return if matches(remaining_mask, &arrangement_chunk) {
            1
        } else {
            0
        }
    }

    let mut valid_arrangement_count = 0;
    for i in 0..(num_of_unassigned_working_springs + 1) {
        let mut arrangement_chunk = String::with_capacity(remaining_mask.len());
        repeat('.')
            .take(i)
            .for_each(|f| arrangement_chunk.push(f));
        
        if remaining_damaged_segments.len() > 0 { 
            arrangement_chunk.push_str(&remaining_damaged_segments[0]);
        }

        valid_arrangement_count += 
            if matches(remaining_mask, &arrangement_chunk) {
                count_valid_arrangements_recursive(
                    &remaining_mask[arrangement_chunk.len()..],
                    num_of_unassigned_working_springs - i, 
                    &remaining_damaged_segments[1..]
                )
            } else {
                0
            }
    }

    valid_arrangement_count
}

fn matches(mask: &str, arrangement: &str) -> bool {
    let mask_chars = &mask[0..arrangement.len()];

    arrangement.char_indices()
        .all(|(i, char)| 
        mask_chars.chars().nth(i) == Some('?')
            || char.eq(&mask_chars.chars().nth(i).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_arrangements_returns_1() {
        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#.?", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);
    }

    #[test]
    fn count_valid_arrangements_works_on_part1_sample_records() {
        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = (".??..??...?##.", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 4);

        let record = ("?#?#?#?#?#?#?#?", vec![1,3,1,6]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#...#...", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.######..#####.", vec![1,6,5]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 4);

        let record = ("?###????????", vec![3,2,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 10);
    }
    
    #[test]
    fn count_valid_arrangements_works_on_part2_sample_records() {
        let record = (
            "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...", 
            vec![4,1,1,4,1,1,4,1,1,4,1,1,4,1,1]
        );
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 16);

        let record = (
            "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.", 
            vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5]
        );
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 2500);

        let record = (
            "?###??????????###??????????###??????????###??????????###????????", 
            vec![3,2,1,3,2,1,3,2,1,3,2,1,3,2,1]
        );
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 506250);
    }

    #[test]
    fn matches_returns_true() {
        let mask = "?###????????";
        
        let arrangement = ".###.##.#...";
        assert!(matches(mask, arrangement));
        
        let arrangement = ".###..##...#";
        assert!(matches(mask, arrangement));
    }
}