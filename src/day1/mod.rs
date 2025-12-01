use crate::problem::Problem;

pub struct Day1;

impl Problem for Day1 {
    type Input = ();
    type Answer1 = u8;
    type Answer2 = u8;

    const PATH: &str = "./src/day1/input.txt";
}
