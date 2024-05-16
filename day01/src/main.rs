
fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut total1 = 0;
    let mut total2 = 0;
    for line in lines {
        let value1 = part1(&line);
        let value2 = part2(&line);
        println!("{} -> {}", line,value2);
        total1 = total1 + value1;
        total2 = total2 + value2;
    }

    println!("total1: {}", total1);
    println!("total2: {}", total2);
}

fn part1(line: &str) -> u32 {
    let mut encountered_digit = false;
    let mut first = 0;
    let mut last = 0;
    for ch in line.chars() {
        if ch.is_ascii_digit() {
            let d = ch.to_digit(10).unwrap();
            if encountered_digit ==  false {
                first = d;
            }
            encountered_digit = true;
            last = d;
        }
    }
    return first * 10 + last;
}

fn part2(line: &str) -> u32 {
    return first(line).unwrap() * 10 + last(line).unwrap();
}

fn first(line: &str) -> Option<u32> {
    let digits = &[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    let mut line_index: usize = 0;
    while line_index <= line.len() {
        let mut value: u32 = 0;
        let mut index: usize = 0;
            while index < digits.len() {
            let digit = digits[index];
            if line[line_index..].starts_with(digit) {return Some(value % 10);}
            index = index + 1;
            value = value + 1;
        }
        line_index = line_index + 1;
    }
    return None;
}

fn last(line: &str) -> Option<u32> {
    let digits = &[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    let mut line_index: usize = line.len();
    loop {
        let mut value: u32 = 0;
        let mut index: usize = 0;
            while index < digits.len() {
            let digit = digits[index];
            if line[line_index..].starts_with(digit) {return Some(value % 10);}
            index = index + 1;
            value = value + 1;
        }
        if line_index == 0 {break};
        line_index = line_index - 1;
    }
    return None;
}