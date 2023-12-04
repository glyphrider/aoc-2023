use std::{fs::File, io::{BufReader,prelude::*}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut bufr = BufReader::new(file);
    let mut result1: u32 = 0;
    let mut result2: u32 = 0;
    let mut card_line = String::new();
    let mut line_number: usize = 0;
    let mut copies: Vec<u32> = vec![];
    loop {
        match bufr.read_line(&mut card_line) {
            Ok(0) => break,
            Err(e) => println!("unexpected error {}", e),

            Ok(_n) => {
                while copies.len() <= line_number {
                    copies.push(0);
                }
                let winner_count = process_card(&card_line.trim());
                if winner_count > 0 {
                    let base: u32 = 2;
                    result1 = result1 + base.pow(winner_count - 1);
                    while copies.len() - line_number <= winner_count.try_into().unwrap() {
                        copies.push(0);
                    }
                    for copy in line_number..line_number+(<u32 as TryInto<usize>>::try_into(winner_count).unwrap()) {
                        copies[copy+1] = copies[copy+1] + (copies[line_number] + 1);
                    }
                    // println!("line number {} has {} copies and {} winners",line_number,copies[line_number],winner_count);
                }
                result2 = result2 + (copies[line_number] + 1);
                line_number = line_number + 1;
            },
        }
        card_line.clear();
    }
    println!("result1 = {}",result1);
    println!("result2 = {}",result2);
}

fn process_card(card_line: &str) -> u32 {
    let mut my_winner_count = 0;
    match card_line.find(':') {
        None => return 0,
        Some(colon) => {
            let _card_number = card_line[5..colon].trim().parse::<u32>().unwrap();
            let mut winning_vec = vec![];
            // println!("processing card #{}", card_number);
            match card_line.find('|') {
                None => return 0,
                Some(separator) => {
                    let winning_str = card_line[colon+1..separator].trim();
                    let my_numbers_str = card_line[separator+1..].trim();
                    let _count_of_winners = process_winning_str(winning_str,&mut winning_vec);
                    // println!("  winning numbers -> {}",count_of_winners);
                    for my_number_str in my_numbers_str.split_ascii_whitespace() {
                        let candidate = my_number_str.parse::<u32>().unwrap();
                        if winning_vec.iter().any(|&winner| winner == candidate) {
                            my_winner_count = my_winner_count + 1;
                        };
                    }
                }
            }
        }
    }
    // if my_winner_count > 0 {
    //     let base: u32 = 2;
    //     value  = base.pow(my_winner_count - 1);
    // }
    // println!("  found {} winners",my_winner_count);
    return my_winner_count;
}

fn process_winning_str(winning_str: &str, winning_vec: &mut Vec<u32>) -> u32 {
    let mut winners = 0;
    for winning_number_str  in winning_str.split_ascii_whitespace() {
        let winner = winning_number_str.parse::<u32>().unwrap();
        winning_vec.insert(winning_vec.len(),winner);
        winners = winners + 1;
    }
    return winners;
}