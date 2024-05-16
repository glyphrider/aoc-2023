use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Symbol {
    key: char,
    line: usize,
    column: usize,
}

struct Number {
    line: usize,
    start: usize,
    end: usize,
    part: u32,
}

fn main() {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let file = File::open("input.txt").unwrap();
    let mut bufr = BufReader::new(file);
    let mut line: String = String::new();
    let mut line_number: usize = 0;
    loop {
        match bufr.read_line(&mut line) {
            Ok(0) => break,
            Ok(_n) => process_line(&line.trim(),line_number,&mut numbers,&mut symbols),
            Err(e) => println!("unexpected error: {}", e),
        }
        line.clear();
        line_number = line_number + 1;
    }
    // println!("read {} lines",line_number);
    let mut result1 = 0;
    let mut result2 = 0;
    let numbers_iter1 = &numbers;
    for number in numbers_iter1 {
        result1 = result1 + part_number_value(&number,&symbols);
    }
    let numbers_iter2 = &numbers;
    for symbol in symbols {
        if symbol.key == '*' {
            let mut gears: Vec<u32> = vec![];
            for number in numbers_iter2 {
                if is_adjacent(&number, &symbol) {
                    gears.insert(gears.len(),number.part);
                }
            }
            if gears.len() ==  2 {
                result2 = result2 + gears[0] * gears[1];
            }
        }
    }
    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}

fn process_line(line: &str, line_number: usize, numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>) {
    // println!("processing => {}",line);
    let mut reading_digit_string = false;
    let mut number_start: usize = 0;
    let mut number_end: usize = 0;
    for (i,c) in line.chars().enumerate() {
        number_end = i;
        if c.is_digit(10) {
            if !reading_digit_string {
                reading_digit_string = true;
                number_start = i;
            }
            continue;
        } else {
            if reading_digit_string {
                reading_digit_string = false;
                let part: u32 = line[number_start..i].parse().unwrap();
                numbers.insert(numbers.len(),Number{line: line_number, start: number_start, end: i, part: part});
                // println!("  found number {} on line {} at {}..{}",part,line_number,number_start,i);
            }
        }
        if c == '.' {continue;}
        // symbol
        let symbol  = Symbol{line: line_number, column: i, key: c};
        // println!("  found symbol at ({},{})",symbol.line,symbol.column);
        symbols.insert(symbols.len(), symbol);
    }
    if reading_digit_string {
        number_end = number_end + 1;
        let part: u32 = line[number_start..number_end].parse().unwrap();
        numbers.insert(numbers.len(),Number{line: line_number, start: number_start, end: number_end, part: part});
        // println!("  found number {} on line {} at {}..{}",part,line_number,number_start,i);
    }
}

fn part_number_value(part: &Number, symbols: &Vec<Symbol>) -> u32 {
    for symbol in symbols {
        if is_adjacent(part,symbol) { return part.part }
    }
    // println!("part number {} ({},{}..{}) is invalid",part.part,part.line,part.start,part.end);
    return 0;
}

fn is_adjacent(part: &Number, symbol: &Symbol) -> bool {
    if symbol.line + 1 == part.line && symbol.column + 1 == part.start { return true }
    if symbol.line == part.line && symbol.column + 1 == part.start { return true }
    if symbol.line == part.line + 1 && symbol.column + 1 == part.start { return true }
    //
    for part_run in part.start..part.end {
        if symbol.line + 1 == part.line && symbol.column == part_run { return true }
        if symbol.line == part.line + 1 && symbol.column == part_run { return true }
    }
    //
    if symbol.line + 1 == part.line && symbol.column == part.end { return true }
    if symbol.line == part.line && symbol.column == part.end { return true }
    if symbol.line == part.line + 1 && symbol.column == part.end { return true }
    return false;
}

#[test]
fn find_one_symbol_test() {
    let line_number = 0;
    let line = "....#...";
    let mut numbers = vec![];
    let mut symbols = vec![];
    process_line(line, line_number, &mut numbers, &mut symbols);
    assert_eq!(numbers.len(),0);
    assert_eq!(symbols.len(),1);
    assert_eq!(symbols[0].line,0);
    assert_eq!(symbols[0].column,4);
}

#[test]
fn find_one_number_test() {
    let line_number = 0;
    let line = "...12...";
    let mut numbers = vec![];
    let mut symbols = vec![];
    process_line(line, line_number, &mut numbers, &mut symbols);
    assert_eq!(numbers.len(),1);
    assert_eq!(symbols.len(),0);
    assert_eq!(numbers[0].line,0);
    assert_eq!(numbers[0].start,3);
    assert_eq!(numbers[0].end,5);
}