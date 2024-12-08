use std::{cmp::Reverse, collections::BinaryHeap};

use crate::lib::solution::{input_lines, Answer, Solution};

/*
Using Binary heap was probably not the best idea in hindsight since you gotta
consume it to loop over the data. BTreeMap would have been a better choice here
possibly
*/

pub struct DayOne {
    set_a: BinaryHeap<Reverse<i64>>,
    set_b: BinaryHeap<Reverse<i64>>,
}

impl Solution<u128> for DayOne {
    fn part_a(&mut self) -> Answer<u128> {
        let mut sum: u128 = 0;

        let mut local_a = self.set_a.clone();
        let mut local_b = self.set_b.clone();
        while let Some((a, b)) = local_a.pop().zip(local_b.pop()) {
            sum += (a.0 - b.0).abs() as u128
        }

        Answer::new(sum)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut sum = 0u128;

        let mut current_val = Reverse(0);
        let mut current_similarity_score = 0;

        let mut local_a = self.set_a.clone();
        let mut local_b = self.set_b.clone();

        while let Some(a) = local_a.pop() {
            if current_val == a {
                sum += current_similarity_score;
                continue;
            }
            current_val = a;

            let mut count = 0;

            while let Some(b_val) = local_b.peek() {
                if (*b_val).0 > current_val.0 {
                    break;
                }

                if *b_val == current_val {
                    count += 1;
                }
                _ = local_b.pop();
            }

            current_similarity_score = (current_val.0 * count) as u128;

            sum += current_similarity_score;
        }

        Answer::new(sum)
    }
}

impl DayOne {
    pub fn new() -> DayOne {
        let mut res = DayOne {
            set_a: BinaryHeap::new(),
            set_b: BinaryHeap::new(),
        };
        for line in input_lines("DayOne") {
            let line = line.expect("Unable to read line from file");

            let data = line
                .split_ascii_whitespace()
                .map(|val| {
                    val.parse::<u64>()
                        .expect(&format!("Input value {} cannot be parsed into u64", val))
                })
                .collect::<Vec<u64>>();

            res.set_a.push(Reverse(data[0] as i64));
            res.set_b.push(Reverse(data[1] as i64));
        }

        res
    }
}
