pub trait Day<T, U> {
    fn get_input(&self) -> T;
    fn part1(&self, input: &T) -> U;
    fn part2(&self, input: &T) -> U;
}