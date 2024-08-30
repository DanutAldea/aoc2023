fn main() {
    let input = include_str!("input.a");
    let mut sum = 0;

    for line in input.lines() {
        let mut val: Option<u32> = None;

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
                val = val.map_or(Some(1), |val| Some(val * 2));
            }
        }

        sum += val.unwrap_or_default();
    }

    println!("{sum}");
}
