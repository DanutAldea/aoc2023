fn main() {
    let input = include_str!("input.a");

    let mut lines = input.lines();
    let durations = lines.next().unwrap();
    let bests = lines.next().unwrap();

    let durations: Vec<f64> = durations
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<f64>().unwrap())
        .collect();
    let bests: Vec<f64> = bests
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<f64>().unwrap())
        .collect();

    let mut sol = 1;

    for (d, b) in durations.into_iter().zip(bests) {
        let delta = (d * d - 4.0 * b).sqrt();
        let s1 = (d - delta) / 2.0;
        let s2 = (d + delta) / 2.0;

        let start = if s1.fract() == 0.0 {
            s1.trunc() as u64 + 1
        } else {
            s1.ceil() as u64
        };

        let end = if s2.fract() == 0.0 {
            s2 as u64 - 1
        } else {
            s2.trunc() as u64
        };

        let r = end - start + 1;
        // println!("{r}: {s1} - {}", s2);

        sol *= r;
    }

    println!("{sol}");
}
