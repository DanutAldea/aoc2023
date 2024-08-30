use std::cmp::min;

fn main() {
    let input = include_str!("input.b");
    let mut sum = 0;

    let no_cards = input.lines().count();
    let mut copies = vec![1usize; no_cards];

    for (idx, line) in input.lines().enumerate() {
        let mut val: usize = 0;

        let (_, numbers) = line.split_once(": ").unwrap();
        let (lucky, numbers) = numbers.split_once(" | ").unwrap();

        let lucky: Vec<u32> = lucky
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let numbers = numbers
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap());

        for num in numbers {
            if lucky.contains(&num) {
                val += 1
            }
        }

        let current_card_copies = copies[idx];

        if val != 0 {
            let end = min(no_cards - 1, idx + val);
            for card in &mut copies[(idx + 1)..=end] {
                *card += current_card_copies
            }
        }
        sum += current_card_copies;
    }

    println!("{sum}");
}
