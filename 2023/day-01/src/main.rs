fn main() {
    // include_str!(""./part1.txt");
    println!("Hello, world!");
}

fn part1(input: &str) -> i32 {
    let mut digits_to_parse = String::new();
    let first = input.chars().next().unwrap();
    let last = input.chars().rev().next().unwrap();
    digits_to_parse.push(first);
    digits_to_parse.push(last);

    digits_to_parse.parse::<i32>().unwrap()
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
treb7uchet";

        let result = part1(sample);

        assert_eq!(result, 142)
    }
}