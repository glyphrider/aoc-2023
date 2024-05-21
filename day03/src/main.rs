use regex::Regex;

struct Symbol {
    key: char,
    line: usize,
    column: usize,
}

struct Part {
    line: usize,
    start: usize,
    end: usize,
    number: u32,
}

fn main() {
    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    create_vectors(include_str!("../input.txt"), &mut parts, &mut symbols);
    
    let result1 = part1(&parts, &symbols);
    let result2 = part2(&parts, &symbols);

    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}

fn part1(parts: &Vec<Part>, symbols: &Vec<Symbol>) -> u32 {
    return parts
        .iter()
        .filter(|number| symbols.iter().any(|symbol| in_aura(number, symbol)))
        .fold(0, |total, number| return number.number + total);
    // let mut result1 = 0;
}

fn part2(parts: &Vec<Part>, symbols: &Vec<Symbol>) -> u32 {
    return symbols
        .iter()
        .filter(|symbol| symbol.key == '*')
        .fold(0, |ratios, symbol| {
            let gears = parts.iter().filter(|part| in_aura(part, symbol));
            if gears.clone().count() == 2 {
                return ratios
                    + parts
                        .iter()
                        .filter(|number| in_aura(number, symbol))
                        .fold(1, |acc, gear| acc * gear.number);
            }
            return ratios;
        });
}

fn create_vectors(input_data: &str, mut parts: &mut Vec<Part>, mut symbols: &mut Vec<Symbol>) {
    input_data
        .lines()
        .enumerate()
        .for_each(|(line_number, line)| {
            process_line(line, line_number, &mut parts, &mut symbols);
        });
}

fn process_line(
    line: &str,
    line_number: usize,
    numbers: &mut Vec<Part>,
    symbols: &mut Vec<Symbol>,
) {
    process_numbers_on_line(line, line_number, numbers);
    process_symbols_on_line(line, line_number, symbols);
}

fn process_numbers_on_line(line: &str, line_number: usize, parts: &mut Vec<Part>) {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|m| Part {
            line: line_number + 1,
            start: m.start() + 1,
            end: m.start() + m.len() + 1,
            number: m.as_str().parse().unwrap(),
        })
        .for_each(|p| {
            parts.push(p);
        });
}

fn process_symbols_on_line(line: &str, line_number: usize, symbols: &mut Vec<Symbol>) {
    let re = Regex::new(r"[^\.0-9]").unwrap();
    re.find_iter(line)
        .map(|m| Symbol {
            key: m.as_str().chars().next().unwrap(),
            line: line_number + 1,
            column: m.start() + 1,
        })
        .for_each(|s| {
            symbols.push(s);
        });
}

fn in_aura(part: &Part, symbol: &Symbol) -> bool {
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
