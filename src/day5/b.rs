use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::Range,
};

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
    fn pass_range(&self, range: &mut Range<u64>) -> (Range<u64>, Range<u64>) {
        let start = max(self.range.start, range.start);
        let end = min(self.range.end, range.end);

        let r = (
            range.start..start,
            ((start as i64 + self.delta) as u64)..((end as i64 + self.delta) as u64),
        );
        *range = max(end, range.start)..(range.end);
        r
    }
}

impl<'a> Layer<'a> {
    fn pass_range(&self, range: &mut Range<u64>) -> Vec<Range<u64>> {
        let mut sol_ranges = vec![];

        for gate in &self.gates {
            let (prev, int) = gate.pass_range(range);

            if !prev.is_empty() {
                sol_ranges.push(prev)
            }

            if !int.is_empty() {
                sol_ranges.push(int)
            }

            if range.is_empty() {
                break;
            }
        }

        if !range.is_empty() {
            sol_ranges.push(range.clone());
        }

        sol_ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
        sol_ranges
    }
}

impl<'a> Sieve<'a> {
    fn pass_ranges(
        &self,
        mut cat: &'a str,
        mut ranges: Vec<Range<u64>>,
    ) -> (&'a str, Vec<Range<u64>>) {
        while let Some(layer) = self.layers.get(cat) {
            cat = layer.next_cat;
            let mut new_ranges = vec![];
            for range in &mut ranges {
                new_ranges.append(&mut layer.pass_range(range))
            }

            new_ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
            ranges = new_ranges;
        }

        (cat, ranges)
    }

    fn add_layer(&mut self, cat: &'a str, layer: Layer<'a>) {
        self.layers.insert(cat, layer);
    }
}

fn main() {
    let input = include_str!("input.b");
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

    assert!(seeds.len() % 2 == 0);
    let mut seed_ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(seeds[i]..(seeds[i] + seeds[i + 1]));
    }

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
                layer.gates.sort_by(|a, b| {
                    a.range
                        .start
                        .cmp(&b.range.start)
                        .then(a.range.end.cmp(&b.range.end))
                })
            }
        }
    }
    sieve.add_layer(current_cat, current_layer.unwrap());

    let (cat, ranges) = sieve.pass_ranges("seed", seed_ranges);
    assert_eq!(cat, "location");

    assert!(!ranges.is_empty());
    println!("{}", ranges.first().unwrap().start);
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

        assert_eq!(layer.pass_range(&mut (79..93)), vec![(81..95)]);
    }
}
