fn parse_line(line: &str) -> Option<u32> {
    let line = line.strip_prefix("Game ").unwrap();
    let (num, line) = parse_num(line);
    let line = line.strip_prefix(": ").unwrap();

    let mut ok = true;
    for game in line.split("; ") {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for pick in game.split(", ") {
            if let (Some(cnt), color) = parse_num(pick) {
                match color {
                    " red" => r += cnt,
                    " green" => g += cnt,
                    " blue" => b += cnt,
                    _ => unreachable!()
                }
            }
        }
        
        ok &= r <= RED;
        ok &= g <= GREEN;
        ok &= b <= BLUE;
    }

    if ok {
        num
    } else {
        None
    }
}

fn parse_num(line: &str) -> (Option<u32>, &str) {
    let idx = line.chars().take_while(|c| c.is_ascii_digit()).count();
    let (num, rest) = line.split_at(idx);

    let num: Option<u32> = num.parse::<u32>().ok(); 

    (num, rest)
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main() {
    let input = include_str!("input.a");
    let mut sum = 0;
    
    for line in input.lines() {
       sum += parse_line(line).unwrap_or_default();
    }

    println!("{sum}");
}

#[cfg(test)]
mod test{
    use crate::{parse_line, parse_num};

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("12 ani"), (Some(12), " ani"));
        assert_eq!(parse_num("n12as"), (None, "n12as"));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some(5));
        assert_eq!(parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), None);
    }
}
