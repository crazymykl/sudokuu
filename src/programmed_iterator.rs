use crate::square::SquareCell;

#[derive(Debug)]
pub struct ProgrammedIterator<'a> {
    squares: &'a [SquareCell],
    program: &'static [usize],
    position: usize,
}

impl<'a> ProgrammedIterator<'a> {
    pub fn new(squares: &'a [SquareCell], program: &'static [usize]) -> Self {
        Self {
            squares,
            program,
            position: 0,
        }
    }
}

impl<'a> Iterator for ProgrammedIterator<'a> {
    type Item = Vec<&'a SquareCell>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.program.len() {
            return None;
        }
        let indexes = self.program.iter().skip(self.position).take(9);
        self.position += 9;

        Some(indexes.map(|&i| &self.squares[i]).collect())
    }
}
