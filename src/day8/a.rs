fn idx_from_node(node: &str) -> Result<usize, ()> {
    if node.len() != 3 {
        Err(())
    } else {
        Ok(node
            .chars()
            .fold(0, |acc, val| acc * 100 + (val as usize - b'A' as usize)))
    }
}

const START: usize = 0;
const END: usize = (b'Z' - b'A') as usize * 10101;

fn main() {
    let input = include_str!("input.a");
    let mut lines = input.lines();

    let direction = lines.next().unwrap();
    let _ = lines.next();

    let mut graph: Vec<(usize, usize)> = vec![(0, 0); END + 1];

    for line in lines {
        let (node, children) = line.split_once(" = (").unwrap();
        let children = children.strip_suffix(')').unwrap();
        let (left, right) = children.split_once(", ").unwrap();

        let node = idx_from_node(node).unwrap();
        let left = idx_from_node(left).unwrap();
        let right = idx_from_node(right).unwrap();

        graph[node] = (left, right);
    }

    let mut step_iter = direction.chars().cycle();
    let mut node = START;
    let mut counter = 0;

    while node != END {
        match step_iter.next().unwrap() {
            'L' => {
                node = graph[node].0;
                counter += 1;
            }
            'R' => {
                node = graph[node].1;
                counter += 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{counter}")
}

#[cfg(test)]
mod test {
    use crate::{idx_from_node, START};

    #[test]
    fn test_idx() {
        assert_eq!(idx_from_node("AAA"), Ok(START));
        assert_eq!(idx_from_node("BAA"), Ok(01_00_00));
        assert_eq!(idx_from_node("ABA"), Ok(00_01_00));
        assert_eq!(idx_from_node("ABBA"), Err(()));
        assert_eq!(idx_from_node("ZZZ"), Ok(25_25_25));
    }
}
