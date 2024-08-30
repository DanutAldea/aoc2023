use std::{cmp::min, collections::HashMap};

const N: usize = 140;

fn main() {
    let input = include_str!("input.b");
    let schematic: Vec<_> = input.lines().collect();

    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

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
                let start_line = j.saturating_sub(current_number_length + 1);
                let end_line = j;

                let start = i.saturating_sub(1);
                let end = min(i + 1, N - 1);

                for (i, l) in schematic.iter().enumerate().take(end + 1).skip(start) {
                    for (j, c) in l[start_line..=end_line].chars().enumerate() {
                        if c == '*' {
                            if let Some(vec) = map.get_mut(&(i, j + start_line)) {
                                vec.push(num);
                            } else {
                                map.insert((i, j + start_line), vec![num]);
                            }
                        }
                    }
                }

                current_number = None;
                current_number_length = 0;
            }
        }

        if let Some(num) = current_number {
            let start = i.saturating_sub(1);
            let end = min(i + 1, N - 1);

            let start_line = N.saturating_sub(current_number_length + 1);

            for (i, l) in schematic.iter().enumerate().take(end + 1).skip(start) {
                for (j, c) in l[start_line..].chars().enumerate() {
                    if c == '*' {
                        if let Some(vec) = map.get_mut(&(i, j + start_line)) {
                            vec.push(num);
                        } else {
                            map.insert((i, j + start_line), vec![num]);
                        }
                    }
                }
            }
        }
    }

    let sum = map
        .values()
        .filter_map(|vec| {
            if vec.len() == 2 {
                Some(vec[0] * vec[1])
            } else {
                None
            }
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or_default();

    println!("{sum}");
}
