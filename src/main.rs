use lib::{implementation::day_one::DayOne, solution::Solution};

mod lib {
    pub mod solution;
    pub mod implementation {
        pub mod day_one;
    }
}

fn main() {
    let mut day: DayOne = DayOne::new();

    println!("Day One, Part A: {}", day.part_a());
    println!("Day One, Part B: {}", day.part_b());
}
