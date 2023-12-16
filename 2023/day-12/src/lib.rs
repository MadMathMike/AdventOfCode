use std::iter::repeat;

pub fn count_valid_arrangements(mask: &str, damaged_segments: &[usize]) -> usize 
{
    if damaged_segments.len() == 0 {
        let mut arrangement_chunk = String::with_capacity(mask.len());
        repeat('.')
            .take(mask.len())
            .for_each(|f| arrangement_chunk.push(f));

        return if matches(mask, &arrangement_chunk) {
            1
        } else {
            0
        }
    }

    // For a given mask length, "??????????????".len() == 14,
    // and a given damaged segment list, [1,1,3], and knowing
    // that at least 1 working spring must go between each
    // damaged section, how many "free" or "available" working
    // springs do we have? We will try placing various amounts
    // of working springs before the next damaged segment, but
    // only up to this limit.
    let free_working_springs:usize = // 14 - 2 - 7 = 7 available working springs
        mask.len() // From example: 14
        - (damaged_segments.len() - 1) // From example: 3 - 1 = 2
        - damaged_segments.iter().sum::<usize>(); // From example: 1 + 1 + 3 = 5

    let mut valid_arrangement_count = 0;
    for i in 0..=free_working_springs {
        let mut arrangement_chunk = String::with_capacity(mask.len());
        repeat('.')
            .take(i)
            .for_each(|f| arrangement_chunk.push(f));

        if damaged_segments.len() > 0 { 
            repeat('#')
                .take(damaged_segments[0])
                .for_each(|f| arrangement_chunk.push(f));

            if damaged_segments.len() > 1 {
                arrangement_chunk.push('.');
            }
        }

        valid_arrangement_count += 
            if matches(mask, &arrangement_chunk) {
                count_valid_arrangements(
                    &mask[arrangement_chunk.len()..],
                    &damaged_segments[1..]
                )
            } else {
                0
            }
    }
    valid_arrangement_count
}

fn matches(mask: &str, arrangement: &str) -> bool {
    let mask_chars = &mask[..arrangement.len()];

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