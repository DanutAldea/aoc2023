use std::collections::HashSet;

fn idx_from_node(node: &str) -> Result<usize, ()> {
    if node.len() != 3 {
        Err(())
    } else {
        Ok(node
            .chars()
            .fold(0, |acc, val| acc * 100 + (val as usize - b'A' as usize)))
    }
}

const END: usize = (b'Z' - b'A') as usize * 10101;

fn is_end(node: usize) -> bool {
    node % 100 == 25
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let aux = b;
        b = a % b;
        a = aux;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = include_str!("input.b");
    let mut lines = input.lines();

    let direction = lines.next().unwrap();
    let _ = lines.next();

    let mut graph: Vec<(usize, usize)> = vec![(0, 0); END + 1];

    let mut starts = vec![];

    for line in lines {
        let (node, children) = line.split_once(" = (").unwrap();
        let children = children.strip_suffix(')').unwrap();
        let (left, right) = children.split_once(", ").unwrap();

        let node = idx_from_node(node).unwrap();
        let left = idx_from_node(left).unwrap();
        let right = idx_from_node(right).unwrap();

        graph[node] = (left, right);
        if node % 100 == 0 {
            starts.push(node);
        }
    }

    let mut steps: Vec<usize> = vec![];
    for mut node in starts {
        let mut ends: HashSet<usize> = HashSet::new();
        let mut cnt = 0;
        let mut step_iter = direction.chars().cycle();

        loop {
            let step = step_iter.next().unwrap();

            if is_end(node) {
                if ends.contains(&(cnt / 2)) {
                    steps.push(cnt / 2);
                    break;
                } else {
                    ends.insert(cnt);
                }
            }

            cnt += 1;
            match step {
                'L' => {
                    node = graph[node].0;
                }
                'R' => {
                    node = graph[node].1;
                }
                _ => unreachable!(),
            }
        }
    }

    let sol = steps.into_iter().fold(1usize, lcm);

    println!("{sol}");
}

#[cfg(test)]
mod test {
    use crate::{gcd, lcm};

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(10, 13), 1);
        assert_eq!(gcd(10, 12), 2);
        assert_eq!(lcm(10, 13), 130);
        assert_eq!(lcm(10, 12), 60);
    }
}
