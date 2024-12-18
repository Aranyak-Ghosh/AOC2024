use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq)]
pub struct Answer<T> {
    inner: T,
}

impl<T> Answer<T> {
    pub fn new(val: T) -> Self {
        Self { inner: val }
    }
}
impl<T: Debug + PartialEq + Display> Debug for Answer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Answer")
            .field("value", &self.inner)
            .finish()
    }
}

impl<T: Debug + PartialEq + Display> Display for Answer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Answer")
            .field("value", &self.inner)
            .finish()
    }
}

fn file_name(day: &str) -> String {
    format!("input/{}/input.txt", day)
}

pub fn input_lines(day: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(file_name(day)).expect(&format!("Unable to open input file with path"));
    let reader = BufReader::new(file);

    reader.lines()
}

pub trait Solution<T: Debug + PartialEq> {
    fn part_a(&mut self) -> Answer<T>;

    fn part_b(&mut self) -> Answer<T>;
}
