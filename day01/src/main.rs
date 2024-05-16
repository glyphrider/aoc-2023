fn main() {
    let input_data = include_str!("../input.txt");
    let part1_result = part1(input_data);
    let part2_result = part2(input_data);
    println!("Part 1 -> {}",part1_result);
    println!("Part 2 -> {}",part2_result);
}

fn part1(data: &'static str) -> u32 {
    return data.lines().fold(0,|result,line| result + part1_line(&line));
}

fn part2(data: &'static str) -> u32 {
    return data.lines().fold(0,|result,line| result + part2_line(&line));
}

fn part1_line(line: &str) -> u32 {
    return line.chars().into_iter().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap() * 10
        + line.chars().into_iter().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
}

fn part2_line(line: &str) -> u32 {
    return first(line).unwrap() * 10 + last(line).unwrap();
}

fn get_digits(backward: bool) -> &'static [&'static str; 20] {
    if backward {
        &[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin" ]
    } else {
        &[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ]
    }
}

fn is_pointing_at_digit(section: &str) -> Option<u32> {
    return get_digits(false).iter().enumerate().fold(None,|current,(value,digit)| {
        if section.starts_with(digit) {Some(u32::try_from(value).unwrap() % 10)} else {current}
    });
}

fn is_pointing_at_backwards_digit(section: &str) -> Option<u32> {
    return get_digits(true).iter().enumerate().fold(None,|current,(value,digit)| {
        if section.starts_with(digit) {Some(u32::try_from(value).unwrap() % 10)} else {current}
    });
}

fn first(line: &str) -> Option<u32> {
    let mut result: Option<u32> = None;
    let mut chars = line.chars();
    while result.is_none() {
        result = is_pointing_at_digit(chars.as_str()); // line[line_index..]);
        if chars.next().is_none() { break; }
    }
    return result;
}

fn last(line: &str) -> Option<u32> {
    let mut result = None;
    let rev = line.chars().rev().collect::<String>();
    let mut chars = rev.chars();
    while result.is_none() {
        result = is_pointing_at_backwards_digit(chars.as_str());
        if chars.next().is_none() { break; }
    }
    return result;
}

#[cfg(test)]
mod test_part1 {

    #[test]
    fn test_part1_line1_of_sample1_txt() {
        assert_eq!(crate::part1_line("1abc2"),12);
    }

    #[test]
    fn test_part1_line2_of_sample1_txt() {
        assert_eq!(crate::part1_line("pqr3stu8vwx"),38);
    }

    #[test]
    fn test_part1_line3_of_sample1_txt() {
        assert_eq!(crate::part1_line("a1b2c3d4e5f"),15);
    }

    #[test]
    fn test_part1_line4_of_sample1_txt() {
        assert_eq!(crate::part1_line("treb7uchet"),77);
    }

    #[test]
    fn test_part1_sample1_txt() {
        assert_eq!(crate::part1(include_str!("../sample1.txt")),142);
    }

    #[test]
    fn test_part1_input_txt() {
        assert_eq!(crate::part1(include_str!("../input.txt")),55029);
    }

}

#[cfg(test)]
mod test_part2 {
    #[test]
    fn test_part2_line1_of_sample2_txt() {
        assert_eq!(crate::part2("two1nine"),29);
    }
    #[test]
    fn test_part2_line2_of_sample2_txt() {
        assert_eq!(crate::part2("eightwothree"),83);
    }
    #[test]
    fn test_part2_line3_of_sample2_txt() {
        assert_eq!(crate::part2("abcone2threexyz"),13);
    }
    #[test]
    fn test_part2_line4_of_sample2_txt() {
        assert_eq!(crate::part2("xtwone3four"),24);
    }
    #[test]
    fn test_part2_line5_of_sample2_txt() {
        assert_eq!(crate::part2("4nineeightseven2"),42);
    }
    #[test]
    fn test_part2_line6_of_sample2_txt() {
        assert_eq!(crate::part2("zoneight234"),14);
    }
    #[test]
    fn test_part2_line7_of_sample2_txt() {
        assert_eq!(crate::part2("7pqrstsixteen"),76);
    }

    #[test]
    fn test_part2_sample2_txt() {
        assert_eq!(crate::part2(include_str!("../sample2.txt")),281);
    }

    #[test]
    fn test_part2_input_txt() {
        assert_eq!(crate::part2(include_str!("../input.txt")),55686);
    }

}