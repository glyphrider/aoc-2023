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
    create_vectors(include_str!("../input.txt"), &mut numbers, &mut symbols);
    let result1 = part1(&numbers, &symbols);
    let result2 = part2(&numbers,&symbols);

    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}

fn part1(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    return numbers
        .iter()
        .filter(|number| symbols.iter().any(|symbol| in_aura(number, symbol)))
        .fold(0, |total, number| return number.part + total);
    // let mut result1 = 0;
}

fn part2(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    return symbols
        .iter()
        .filter(|symbol| symbol.key == '*')
        .fold(0,|ratios,symbol| {
            let gears = numbers.iter().filter(|number| in_aura(number,symbol));
            if gears.clone().count() == 2 {
                return ratios + numbers.iter().filter(|number| in_aura(number,symbol)).fold(1,|acc,gear| acc * gear.part);
            }
            return ratios;
        });
}

fn create_vectors(input_data: &str, mut numbers: &mut Vec<Number>, mut symbols: &mut Vec<Symbol>) {
    input_data
        .lines()
        .enumerate()
        .for_each(|(line_number, line)| {
            process_line(line, line_number, &mut numbers, &mut symbols);
        });
}

fn process_line(
    line: &str,
    line_number: usize,
    numbers: &mut Vec<Number>,
    symbols: &mut Vec<Symbol>,
) {
    // println!("processing => {}",line);
    let mut reading_digit_string = false;
    let mut number_start: usize = 0;
    let mut number_end: usize = 0;
    for (i, c) in line.chars().enumerate() {
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
                numbers.insert(
                    numbers.len(),
                    Number {
                        line: line_number + 1,
                        start: number_start + 1,
                        end: i + 1,
                        part: part,
                    },
                );
                // println!("  found number {} on line {} at {}..{}",part,line_number,number_start,i);
            }
        }
        if c == '.' {
            continue;
        }
        // symbol
        let symbol = Symbol {
            line: line_number + 1,
            column: i + 1,
            key: c,
        };
        // println!("  found symbol at ({},{})",symbol.line,symbol.column);
        symbols.insert(symbols.len(), symbol);
    }
    if reading_digit_string {
        number_end = number_end + 1;
        let part: u32 = line[number_start..number_end].parse().unwrap();
        numbers.insert(
            numbers.len(),
            Number {
                line: line_number + 1,
                start: number_start + 1,
                end: number_end + 1,
                part: part,
            },
        );
        // println!("  found number {} on line {} at {}..{}",part,line_number,number_start,i);
    }
}

// fn part_number_value(part: &Number, symbols: &Vec<Symbol>) -> u32 {
//     for symbol in symbols {
//         if is_adjacent(part, symbol) {
//             return part.part;
//         }
//     }
//     // println!("part number {} ({},{}..{}) is invalid",part.part,part.line,part.start,part.end);
//     return 0;
// }

fn in_aura(part: &Number, symbol: &Symbol) -> bool {
    return symbol.line >= part.line - 1
        && symbol.line <= part.line + 1
        && symbol.column >= part.start - 1
        && symbol.column <= part.end;
}

#[cfg(test)]
mod part1_test {
    #[test]
    fn find_one_symbol_test() {
        let line_number = 0;
        let line = "....#...";
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::process_line(line, line_number, &mut numbers, &mut symbols);
        assert_eq!(numbers.len(), 0);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].line, 1);
        assert_eq!(symbols[0].column, 5);
    }

    #[test]
    fn find_one_number_test() {
        let line_number = 0;
        let line = "...12...";
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::process_line(line, line_number, &mut numbers, &mut symbols);
        assert_eq!(numbers.len(), 1);
        assert_eq!(symbols.len(), 0);
        assert_eq!(numbers[0].line, 1);
        assert_eq!(numbers[0].start, 4);
        assert_eq!(numbers[0].end, 6);
    }

    #[test]
    fn find_10_parts_in_sample_data() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../sample1.txt"), &mut numbers, &mut symbols);
        assert_eq!(numbers.len(), 10);
    }

    #[test]
    fn find_6_symbols_in_sample_data() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../sample1.txt"), &mut numbers, &mut symbols);
        assert_eq!(symbols.len(), 6);
    }
    #[test]
    fn sample_data_test() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../sample1.txt"), &mut numbers, &mut symbols);
        let result1 = crate::part1(&numbers, &symbols);
        assert_eq!(result1, 4361);
    }
    #[test]
    fn input_data_test() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../input.txt"), &mut numbers, &mut symbols);
        let result1 = crate::part1(&numbers, &symbols);
        assert_eq!(result1, 543867);
    }
}

#[cfg(test)]
mod part2_test {
    #[test]
    fn sample_data_test() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../sample1.txt"), &mut numbers, &mut symbols);
        let result1 = crate::part2(&numbers, &symbols);
        assert_eq!(result1, 467835);
    }
    #[test]
    fn input_data_test() {
        let mut numbers = vec![];
        let mut symbols = vec![];
        crate::create_vectors(include_str!("../input.txt"), &mut numbers, &mut symbols);
        let result1 = crate::part2(&numbers, &symbols);
        assert_eq!(result1, 79613331);
    }
}