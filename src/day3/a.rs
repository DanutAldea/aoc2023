use std::cmp::min;

const N: usize = 140;

fn main() {
    let input = include_str!("input.a");
    let schematic: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut current_number = None;
        let mut current_number_length = 0;

        for (j, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                current_number = current_number.map_or(char.to_digit(10), |num| {
                    char.to_digit(10).map(|dig| dig + num * 10)
                });
                current_number_length += 1;
            } else if let Some(num) = current_number {
                let mut should_add = false;

                let start_line = j.saturating_sub(current_number_length + 1);
                let end_line = j;

                let start = i.saturating_sub(1);
                let end = min(i + 1, N - 1);

                for l in schematic.iter().take(end + 1).skip(start) {
                    for c in l[start_line..=end_line].chars() {
                        should_add |= (!c.is_ascii_digit()) & (c != '.');
                    }
                }

                if should_add {
                    sum += num;
                    // println!("{num}");
                }

                current_number = None;
                current_number_length = 0;
            }
        }

        if let Some(num) = current_number {
            let mut should_add = false;

            let start = i.saturating_sub(1);
            let end = min(i + 1, N - 1);

            let start_line = N.saturating_sub(current_number_length + 1);

            for l in schematic.iter().take(end + 1).skip(start) {
                for c in l[start_line..].chars() {
                    should_add |= (!c.is_ascii_digit()) & (c != '.');
                }
            }

            if should_add {
                sum += num;
                // println!("{num}");
            }
        }
    }

    println!("{sum}");
}
