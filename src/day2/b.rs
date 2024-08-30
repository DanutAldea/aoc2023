use std::cmp;

fn parse_line(line: &str) -> u32 {
    let (_, line) = line.split_once(": ").unwrap();

    let mut r = None;
    let mut g = None;
    let mut b = None;

    for game in line.split("; ") {
        for pick in game.split(", ") {
            if let (Some(cnt), color) = parse_num(pick) {
                match color {
                    " red" => r = r.map_or(Some(cnt), |r| Some(cmp::max(r, cnt))),
                    " green" => g = g.map_or(Some(cnt), |g| Some(cmp::max(g, cnt))),
                    " blue" => b = b.map_or(Some(cnt), |b| Some(cmp::max(b, cnt))),
                    _ => unreachable!(),
                }
            }
        }
    }

    r.unwrap_or(1) * g.unwrap_or(1) * b.unwrap_or(1)
}

fn parse_num(line: &str) -> (Option<u32>, &str) {
    let idx = line.chars().take_while(|c| c.is_ascii_digit()).count();
    let (num, rest) = line.split_at(idx);

    let num: Option<u32> = num.parse::<u32>().ok();

    (num, rest)
}

fn main() {
    let input = include_str!("input.b");
    let mut sum = 0;

    for line in input.lines() {
        sum += parse_line(line);
    }

    println!("{sum}");
}

#[cfg(test)]
mod test {
    use crate::{parse_line, parse_num};

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("12 ani"), (Some(12), " ani"));
        assert_eq!(parse_num("n12as"), (None, "n12as"));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }
}
