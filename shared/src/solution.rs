use crate::Answer;

pub trait Solution {
    fn part_1(&self, input: &str) -> Answer;
    fn part_2(&self, input: &str) -> Answer;
}
