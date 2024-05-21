struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let result1 = read_cards(include_str!("../input.txt"));
    println!("result1 = {}", result1);
    // println!("result2 = {}", result2);
}

fn read_card(input: &str) -> Card {
    let mut card1 = input.split(':').map(|elem| elem.trim());
    let header = card1.next().unwrap();
    let all_numbers_string = card1.next().unwrap();
    let re = regex::Regex::new(r"Card[[:space:]]+([[:digit:]]+)").unwrap();
    let id = re.captures(header).unwrap()[1].trim().parse().unwrap();
    let mut card2 = all_numbers_string.split('|').map(|elem| elem.trim());
    let winners_string = card2.next().unwrap();
    let numbers_string = card2.next().unwrap();
    let mut winners: Vec<u32> = vec![];
    let mut numbers: Vec<u32> = vec![];
    winners_string
        .split_whitespace()
        .into_iter()
        .map(|w| w.parse().unwrap())
        .for_each(|w| winners.push(w));
    numbers_string
        .split_whitespace()
        .into_iter()
        .map(|n| n.parse().unwrap())
        .for_each(|n| numbers.push(n));
    return Card {
        id: id,
        winners: winners,
        numbers: numbers,
    };
}

fn value(card: Card) -> u32 {
    return card
        .winners
        .iter()
        .filter(|w| card.numbers.iter().any(|n| &n == w))
        .fold(0, |acc, _| {
            if acc == 0 {
                return 1;
            } else {
                return acc * 2;
            }
        });
}

fn read_cards(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| crate::value(crate::read_card(line)))
        .fold(0, |acc, value| acc + value);
}

#[cfg(test)]
mod part1_test {
    #[test]
    fn read_card_test() {
        let card = crate::read_card("Card 13: 12 13 14 15  1 |  2  5 10 12 14 20 55 66");
        assert_eq!(card.id, 13);
        assert_eq!(card.winners.len(), 5);
        assert_eq!(card.numbers.len(), 8);
    }

    #[test]
    fn sample1_line1_test() {
        let card = crate::read_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(crate::value(card), 8);
    }
    #[test]
    fn sample1_test() {
        assert_eq!(crate::read_cards(include_str!("../sample1.txt")), 13);
    }

    #[test]
    fn input_test() {
        assert_eq!(crate::read_cards(include_str!("../input.txt")), 25004);
    }
}
