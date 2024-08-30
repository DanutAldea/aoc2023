fn main() -> Result<(), Box<std::io::Error>> {
    let lines = include_str!("input.a");
    let mut sum: u32 = 0;

    for line in lines.lines() {
        let digits: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();

        if let Some(d) = digits.last() {
            sum += d.to_digit(10).unwrap()
        }
        if let Some(d) = digits.first() {
            sum += 10 * d.to_digit(10).unwrap();
        }
    }

    println!("{sum}");
    Ok(())
}
