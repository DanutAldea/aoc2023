fn main() -> Result<(), Box<std::io::Error>> {
    let lines = include_str!("input.b");
    let mut sum: u32 = 0;

    let strings: Vec<(&str, u32)> = vec![
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    for line in lines.lines() {
        let mut first = None;
        let mut last = None;

        for (str, val) in strings.iter() {
            if let Some(idx) = line.find(str) {
                first = first.map_or(Some((idx, val)), |(old_idx, old_val)| {
                    if idx < old_idx {
                        Some((idx, val))
                    } else {
                        Some((old_idx, old_val))
                    }
                });
            }

            if let Some(idx) = line.rfind(str) {
                last = last.map_or(Some((idx, val)), |(old_idx, old_val)| {
                    if idx > old_idx {
                        Some((idx, val))
                    } else {
                        Some((old_idx, old_val))
                    }
                });
            }
        }

        sum += first.unwrap().1 * 10 + last.unwrap().1;
    }

    println!("{sum}");
    Ok(())
}
