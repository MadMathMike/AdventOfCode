pub fn parse_input(input: &str) -> Vec<(&str, Vec<usize>)> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let mask = parts[0];
    let damaged_segments = parts[1]
        .split(',')
        .map(|num| 
            num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (mask, damaged_segments)
}

pub fn unfold(mask: &str, damaged_segments: &Vec<usize>) -> (String, Vec<usize>) {

    let repeated_mask = (0..5)
        .map(|_| mask)
        .collect::<Vec<&str>>()
        .join("?");

    let repeated_damaged_segements = (0..5)
        .map(|_| damaged_segments)
        .flatten()
        .map(|segment_length| *segment_length)
        .collect::<Vec<usize>>();

    (repeated_mask, repeated_damaged_segements)
}

#[cfg(test)]
mod tests {
    use assertx::assert_contains_exactly;

    use super::*;

    #[test]
    fn parse_line_returns_mask_and_damaged_segments() {
        let line = "?#.??????#??#?#?#?#? 1,1,15";
        let (mask, damaged_segments) = parse_line(line);

        assert_eq!(mask, "?#.??????#??#?#?#?#?");
        assert_contains_exactly!(damaged_segments, vec![1, 1, 15]);
    }

    #[test]
    fn parse_input_returns_list_of_mask_and_damaged_segments() {
        let input = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let records = parse_input(input);

        assert_eq!(records.len(), 6);

        let (mask, damaged_segments) = &records[0];
        assert_eq!(*mask, "???.###");
        assert_contains_exactly!(damaged_segments, vec![1, 1, 3]);

        let (mask, damaged_segments) = records.iter().last().unwrap();
        assert_eq!(*mask, "?###????????");
        assert_contains_exactly!(damaged_segments, vec![3, 2, 1]);
    }

    #[test]
    fn unfold_record_duplicates_4_times() {
        let record = ("???.###", vec![1,1,3]);
        let unfolded_record = unfold(record.0, &record.1);
        let (mask, damaged_segments) = unfolded_record;
        
        assert_eq!(mask, "???.###????.###????.###????.###????.###");
        assert_eq!(damaged_segments, vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]);
    }
}
