use crate::lib::solution::{input_lines, Answer, Solution};

pub struct DayTwo {
    data: Vec<Vec<u64>>,
}

impl DayTwo {
    pub fn new() -> DayTwo {
        let mut res = DayTwo { data: Vec::new() };

        for line in input_lines("DayTwo") {
            let line = line.unwrap();

            res.data.push(
                line.split_ascii_whitespace()
                    .map(|val| {
                        val.parse::<u64>()
                            .expect("Can't parse values in line as u64")
                    })
                    .collect::<Vec<u64>>(),
            );
        }
        res
    }

    fn is_valid_transition(
        &self,
        prev: u64,
        current: u64,
        increasing: &mut bool,
        decreasing: &mut bool,
    ) -> bool {
        let diff = current as i64 - prev as i64;

        *increasing &= diff > 0 && diff <= 3;
        *decreasing &= diff < 0 && diff.abs() <= 3;

        *increasing || *decreasing
    }

    fn is_valid_report(&self, report: &[u64]) -> Option<usize> {
        let mut increasing = true;
        let mut decreasing = true;

        let mut invalid_index: Option<usize> = None;

        let mut prev_level = report[0];

        for i in 1..report.len() {
            let is_valid =
                self.is_valid_transition(prev_level, report[i], &mut increasing, &mut decreasing);

            if !is_valid {
                invalid_index = Some(i);
                break;
            }

            prev_level = report[i];
        }

        invalid_index
    }

    fn is_sub_report_valid(&self, report: &[u64], skip_index: usize, next_index: usize) -> bool {
        let mut increasing = true;
        let mut decreasing = true;

        let mut prev = if skip_index == 0 {
            report[next_index]
        } else {
            report[skip_index - 1]
        };

        for i in next_index..report.len() {
            let current = report[i];
            if !self.is_valid_transition(prev, current, &mut increasing, &mut decreasing) {
                return false;
            }
            prev = current;
        }

        true
    }
}

impl Solution<u64> for DayTwo {
    fn part_a(&mut self) -> Answer<u64> {
        let mut safe_report_count = 0_u64;

        for report in self.data.iter() {
            if self.is_valid_report(report).is_none() {
                safe_report_count += 1;
            }
        }

        Answer::new(safe_report_count)
    }

    fn part_b(&mut self) -> Answer<u64> {
        let mut safe_report_count = 0_u64;

        for report in self.data.iter() {
            if let Some(invalid_index) = self.is_valid_report(report) {
                if self.is_sub_report_valid(report, invalid_index, invalid_index + 1) {
                    safe_report_count += 1;
                } else if self.is_sub_report_valid(report, invalid_index - 1, invalid_index) {
                    safe_report_count += 1;
                }
            } else {
                safe_report_count += 1;
            }
        }

        Answer::new(safe_report_count)
    }
}
