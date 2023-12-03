fn main() {
    let part1_input = include_str!("./part1.txt");
    let result = part1(part1_input);

    dbg!(result);
}

fn part1(input: &str) -> i32 {
    input.lines()
        .filter(|line| !line.is_empty())
        .map( |line| {
            let mut digits_to_parse = String::new();
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
            digits_to_parse.push(first);
            digits_to_parse.push(last);

            digits_to_parse.parse::<i32>().unwrap()
        }
    ).sum()    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_when_first_and_last_character_are_numbers() {
        let result = part1("1abc2");
        
        assert_eq!(result, 12);
    }

    #[test]
    fn works_when_numbers_are_in_the_middle() {
        let result = part1("pqr3stu8vwx");
        
        assert_eq!(result, 38);
    }

    #[test]
    fn works_when_more_than_two_numbers() {
        let result = part1("a1b2c3d4e5f");
        
        assert_eq!(result, 15);
    }

    #[test]
    fn works_when_only_one_number() {
        let result = part1("treb7uchet");

        assert_eq!(result, 77);
    }

    #[test]
    fn sums_results_from_multiple_lines() {
        let sample = 
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

";

        let result = part1(sample);

        assert_eq!(result, 142)
    }
}