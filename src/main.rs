use lib::{
    implementation::{day_one::DayOne, day_two::DayTwo},
    solution::Solution,
};

mod lib {
    pub mod solution;
    pub mod implementation {
        pub mod day_one;
        pub mod day_two;
    }
}

fn main() {
    let mut day: DayOne = DayOne::new();

    println!("Day One, Part A: {}", day.part_a());
    println!("Day One, Part B: {}", day.part_b());

    let mut day: DayTwo = DayTwo::new();

    println!("Day Two, Part A: {}", day.part_a());
    println!("Day Two, Part B: {}", day.part_b());
}
