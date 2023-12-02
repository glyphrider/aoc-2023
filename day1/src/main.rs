fn main() {
    let mut total1 = 0;
    let mut total2 = 0;
    for line in std::fs::read_to_string("input.txt")
    .unwrap()
    .lines() {
        match part1(&line) {
            Some(value1) => total1 = total1 + value1,
            None => println!("Could not solve part one for line \"{}\"", line),
        }
        match part2(&line) {
            Some(value2) => total2 = total2 + value2,
            None => println!("Could not solve part two for line \"{}\"", line),
        }
    }

    println!("total1: {}", total1);
    println!("total2: {}", total2);
}

fn part1(line: &str) -> Option<u32> {
    let mut encountered_digit = false;
    let mut first = 0;
    let mut last = 0;
    for ch in line.chars() {
        if ch.is_ascii_digit() {
            let d = ch.to_digit(10)?;
            if encountered_digit == false {
                first = d;
            }
            encountered_digit = true;
            last = d;
        }
    }
    return Some(first * 10 + last);
}

fn part2(line: &str) -> Option<u32> {
    return Some(first(line)? * 10 + last(line)?);
}

fn first(line: &str) -> Option<u32> {
    let digits = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "0", /* this second zero is a place holder so that "one" is 1 after the mod 10 */
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut line_index: usize = 0;
    while line_index <= line.len() {
        let mut value: u32 = 0;
        let mut index: usize = 0;
        while index < digits.len() {
            let digit = digits[index];
            if line[line_index..].starts_with(digit) {
                return Some(value % 10);
            }
            index = index + 1;
            value = value + 1;
        }
        line_index = line_index + 1;
    }
    return None;
}

fn last(line: &str) -> Option<u32> {
    let digits = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "0", /* this second zero is a place holder so that "one" is 1 after the mod 10 */
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut line_index: usize = line.len();
    loop {
        let mut value: u32 = 0;
        let mut index: usize = 0;
        while index < digits.len() {
            let digit = digits[index];
            if line[line_index..].starts_with(digit) {
                return Some(value % 10);
            }
            index = index + 1;
            value = value + 1;
        }
        if line_index == 0 {
            break;
        };
        line_index = line_index - 1;
    }
    return None;
}

#[test]
fn part1_one_digit_test() {
    assert_eq!(part1("1"), Some(11));
}

#[test]
fn part1_two_digits_test() {
    assert_eq!(part1("23"), Some(23));
}

#[test]
fn part1_three_digits_test() {
    assert_eq!(part1("456"), Some(46));
}

#[test]
fn part1_leader_test() {
    assert_eq!(part1("xx78"), Some(78));
}

#[test]
fn part1_trailer_test() {
    assert_eq!(part1("78xx"), Some(78));
}

#[test]
fn part1_in_medias_res_test() {
    assert_eq!(part1("7xx8"), Some(78));
}

#[test]
fn part1_filler_everywhere_test() {
    assert_eq!(part1("x7x8x"), Some(78));
}

#[test]
fn part2_text_first_test() {
    assert_eq!(first("one"), Some(1));
}

#[test]
fn part2_numeral_first_test() {
    assert_eq!(first("2"), Some(2));
}

#[test]
fn part2_text_last_test() {
    assert_eq!(last("2three"), Some(3));
}

#[test]
fn part2_numeral_last_test() {
    assert_eq!(last("three2"), Some(2));
}

#[test]
fn part2_text_first_with_leader_test() {
    assert_eq!(first("threfour2"), Some(4));
}
