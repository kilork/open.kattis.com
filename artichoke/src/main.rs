use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let res: Result<[i64; 6], _> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into();
        if let Ok([p, a, b, c, d, n]) = res {
            let mut prices =
                (1..=n).map(|k| ((a * k + b) as f64).sin() + ((c * k + d) as f64).cos() + 2.0);
            if let Some(min_max) = prices.next() {
                let (mut min, mut max) = (min_max, min_max);
                let mut max_decline = 0.0;
                for price in prices {
                    if price < min {
                        min = price;
                    } else if price > max {
                        let new_max_decline = max - min;
                        if new_max_decline > max_decline {
                            max_decline = new_max_decline;
                        }
                        max = price;
                        min = price;
                    }
                }
                let new_max_decline = max - min;
                if new_max_decline > max_decline {
                    max_decline = new_max_decline;
                }
                println!("{:.6}", max_decline * p as f64);
            } else {
                panic!("wrong input")
            }
        } else {
            panic!("wrong input")
        }
    }
}
