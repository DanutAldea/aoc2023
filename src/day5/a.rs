use std::{cmp::min, collections::HashMap, ops::Range};

#[derive(Debug)]
struct Layer<'a> {
    gates: Vec<Gate>,
    next_cat: &'a str,
}

#[derive(Debug)]
struct Gate {
    range: Range<u64>,
    delta: i64,
}

#[derive(Debug)]
struct Sieve<'a> {
    layers: HashMap<&'a str, Layer<'a>>,
}

impl Gate {
    fn pass(&self, num: u64) -> Option<u64> {
        if self.range.contains(&num) {
            Some((num as i64 + self.delta) as u64)
        } else {
            None
        }
    }
}

impl<'a> Layer<'a> {
    fn pass(&self, num: u64) -> (&'a str, u64) {
        self.gates
            .iter()
            .find_map(|gate| gate.pass(num).map(|num| (self.next_cat, num)))
            .unwrap_or((self.next_cat, num))
    }
}

impl<'a> Sieve<'a> {
    fn pass(&self, mut cat: &'a str, mut num: u64) -> (&'a str, u64) {
        while let Some(layer) = self.layers.get(cat) {
            (cat, num) = layer.pass(num);
        }

        (cat, num)
    }

    fn add_layer(&mut self, cat: &'a str, layer: Layer<'a>) {
        self.layers.insert(cat, layer);
    }
}

fn main() {
    let input = include_str!("input.a");
    let mut sieve = Sieve {
        layers: HashMap::new(),
    };

    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let _ = lines.next();

    let mut current_cat = "";
    let mut current_layer = Some(Layer {
        gates: vec![],
        next_cat: "",
    });

    for line in lines {
        if line.is_empty() {
            sieve.add_layer(current_cat, current_layer.take().unwrap());
        } else if line.starts_with(|c: char| !c.is_ascii_digit()) {
            let line = line.strip_suffix(" map:").expect("Should end in \" map:\"");
            let (left, right) = line.split_once("-to-").expect("Should end in \" map:\"");

            current_cat = left;
            current_layer = Some(Layer {
                gates: vec![],
                next_cat: right,
            });
        } else {
            let nums: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .take(3)
                .collect();
            let end = nums[0];
            let start = nums[1];
            let len = nums[2];

            let gate = Gate {
                range: start..(start + len),
                delta: end as i64 - start as i64,
            };

            if let Some(layer) = &mut current_layer {
                layer.gates.push(gate);
            }
        }
    }
    sieve.add_layer(current_cat, current_layer.unwrap());

    let mut min_loc = u64::MAX;
    for seed in seeds {
        let (cat, loc) = sieve.pass("seed", seed);
        assert_eq!(cat, "location");
        min_loc = min(min_loc, loc);
    }

    println!("{min_loc}")
}

#[cfg(test)]
mod test {
    use crate::{Gate, Layer};

    #[test]
    fn test_layer() {
        let layer = Layer {
            gates: vec![
                Gate {
                    range: 50..52,
                    delta: -48,
                },
                Gate {
                    range: 52..100,
                    delta: 2,
                },
            ],
            next_cat: "soil",
        };

        assert_eq!(layer.pass(79), ("soil", 81));
    }
}
