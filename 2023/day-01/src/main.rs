fn main() {
    let part1_input = include_str!("./part1.txt");
    let result = part1(part1_input);

    dbg!(result);

    let part2_input = include_str!("./part2.txt");
    let result = part2(part2_input);

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
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input.lines()
        .filter(|line| !line.is_empty())
        .map( |line| part2_process_line(line))
        .sum()
}

// Each input line will have either at least one numeric character or 
// at least one word that spells a number (e.g., "one", "five", etc.)
// Only works on ASCII.
fn part2_process_line(line: &str) -> i32 {
    let mut first_number: Option<char> = None;
    let mut start_word_index = 0;
    let mut end_word_index = 0;

    for c in line.chars() {
        let word_window_size = end_word_index - start_word_index + 1; // +1 to include boundary character
        if word_window_size >= 3 {
            // check for digit in slice
            let word = &line[start_word_index..=end_word_index];
            let first_number_candidate = part2_get_digit_char_from_word(word);
            if first_number_candidate != None {
                first_number = first_number_candidate;
                break;
            }
        } else if word_window_size == 5 {
            start_word_index += 1;
        }

        end_word_index += 1;

        if c.is_numeric() {
            first_number = Some(c);
            break;
        }
    }

    assert_ne!(first_number, None);

    let mut second_number: Option<char> = None;
    end_word_index = line.len() - 1;
    start_word_index = line.len() - 1;

    for c in line.chars().rev() {
        let word_window_size = end_word_index - start_word_index + 1; // +1 to include boundary character

        if word_window_size >= 3 {
            // check for digit in slice
            let word = &line[start_word_index..=end_word_index];
            let second_number_candidate = part2_get_digit_char_from_word(word);
            if second_number_candidate != None {
                second_number = second_number_candidate;
                break;
            }
        } else if word_window_size == 5 {
            end_word_index -= 1;
        }

        if c.is_numeric() {
            second_number = Some(c);
            break;
        }

        start_word_index -= 1;
    }

    assert_ne!(second_number, None);

    let mut digits_to_parse = String::new();
    digits_to_parse.push(first_number.unwrap());
    digits_to_parse.push(second_number.unwrap());

    digits_to_parse.parse::<i32>().unwrap()
}

fn part2_get_digit_char_from_word(word: &str) -> Option<char> {
    if word.contains("one") {
        return Some('1');
    } else if word.contains("two") {
        return Some('2');
    } else if word.contains("three") {
        return Some('3');
    } else if word.contains("four") {
        return Some('4');
    } else if word.contains("five") {
        return Some('5');
    } else if word.contains("six") {
        return Some('6');
    } else if word.contains("seven") {
        return Some('7');
    } else if word.contains("eight") {
        return Some('8');
    } else if word.contains("nine") {
        return Some('9');
    } else {
        return None;
    }
}

#[cfg(test)]
mod part1_tests {
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn works_when_first_and_last_words_are_numbers() {
        let result = part2_process_line("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn works_when_no_numeric_characters() {
        let result = part2_process_line("eightwothree");
        assert_eq!(result, 83);
    }

    #[test]
    fn works_when_beginning_and_end_characters_do_not_spell_number() {
        let result = part2_process_line("abcone2threexyz");
        assert_eq!(result, 13);
    }

    #[test]
    fn works_for_small_line() {
        let result = part2_process_line("one");
        assert_eq!(result, 11);
    }

    #[test]
    fn works_for_small_number_at_beginning() {
        let result = part2_process_line("onezyx");
        assert_eq!(result, 11);
    }

    #[test]
    fn works_for_single_digit() {
        let result = part2_process_line("1");
        assert_eq!(result, 11);
    }

    #[test]
    fn sums_results_from_multiple_lines() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = part2(input);
        assert_eq!(result, 281);
    }
}