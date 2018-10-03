use crate::programmed_iterator::ProgrammedIterator;
use crate::square::{self, SquareCell};
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

pub type Grid = Vec<SquareCell>;

#[derive(Clone, PartialEq)]
pub struct Board {
    squares: Grid,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static ROWS_PROGRAM : &'static [usize] = &[
     0, 1, 2, 3, 4, 5, 6, 7, 8,
     9,10,11,12,13,14,15,16,17,
    18,19,20,21,22,23,24,25,26,
    27,28,29,30,31,32,33,34,35,
    36,37,38,39,40,41,42,43,44,
    45,46,47,48,49,50,51,52,53,
    54,55,56,57,58,59,60,61,62,
    63,64,65,66,67,68,69,70,71,
    72,73,74,75,76,77,78,79,80];

#[cfg_attr(rustfmt, rustfmt_skip)]
static COLS_PROGRAM : &'static [usize] = &[
     0, 9,18,27,36,45,54,63,72,
     1,10,19,28,37,46,55,64,73,
     2,11,20,29,38,47,56,65,74,
     3,12,21,30,39,48,57,66,75,
     4,13,22,31,40,49,58,67,76,
     5,14,23,32,41,50,59,68,77,
     6,15,24,33,42,51,60,69,78,
     7,16,25,34,43,52,61,70,79,
     8,17,26,35,44,53,62,71,80];

#[cfg_attr(rustfmt, rustfmt_skip)]
static BOXES_PROGRAM : &'static [usize] = &[
     0, 1, 2, 9,10,11,18,19,20,
     3, 4, 5,12,13,14,21,22,23,
     6, 7, 8,15,16,17,24,25,26,
    27,28,29,36,37,38,45,46,47,
    30,31,32,39,40,41,48,49,50,
    33,34,35,42,43,44,51,52,53,
    54,55,56,63,64,65,72,73,74,
    57,58,59,66,67,68,75,76,77,
    60,61,62,69,70,71,78,79,80];

impl Board {
    pub fn new(squares: Grid) -> Self {
        Self { squares }
    }

    pub fn prune(&self) -> Self {
        loop {
            let old = self.clone();

            prune_groups(self.rows());
            prune_groups(self.cols());
            prune_groups(self.boxes());

            if old == *self {
                return old;
            }
        }
    }

    pub fn solve(&self) -> Option<Self> {
        let pruned = self.prune();

        if !pruned.is_valid() {
            None
        } else if pruned.is_solved() {
            Some(pruned)
        } else {
            pruned.solve_rec()
        }
    }

    #[cfg(feature = "parallelism")]
    fn solve_rec(&self) -> Option<Self> {
        let (left, right) = self.next_grids();
        let (left, right) = rayon::join(|| left.solve(), || right.solve());

        left.or(right)
    }

    #[cfg(not(feature = "parallelism"))]
    fn solve_rec(&self) -> Option<Self> {
        let (left, right) = self.next_grids();

        left.solve().or_else(|| right.solve())
    }

    fn is_valid(&self) -> bool {
        valid_groups(self.rows()) && valid_groups(self.cols()) && valid_groups(self.boxes())
    }

    fn is_solved(&self) -> bool {
        self.squares.iter().all(|x| x.borrow().is_fixed())
    }

    fn next_grids(&self) -> (Self, Self) {
        let best_candidate = self
            .squares
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.borrow().is_fixed())
            .min_by_key(|(_, c)| c.borrow().probability_count())
            .map(|(i, _)| i)
            .expect("Called next_grids on solved board?");

        self.split_at(best_candidate)
    }

    fn split_at(&self, idx: usize) -> (Self, Self) {
        let (fixed, possible) = self.squares[idx].borrow().split();
        let (mut first, mut rest) = (self.clone(), self.clone());

        *first.squares[idx].borrow_mut() = fixed;
        *rest.squares[idx].borrow_mut() = possible;

        (first, rest)
    }

    pub fn rows(&self) -> ProgrammedIterator {
        ProgrammedIterator::new(&self.squares, ROWS_PROGRAM)
    }

    pub fn cols(&self) -> ProgrammedIterator {
        ProgrammedIterator::new(&self.squares, COLS_PROGRAM)
    }

    pub fn boxes(&self) -> ProgrammedIterator {
        ProgrammedIterator::new(&self.squares, BOXES_PROGRAM)
    }
}

fn valid_groups(mut groups: ProgrammedIterator) -> bool {
    groups.all(|g| square::valid_group(&g))
}

fn prune_groups(groups: ProgrammedIterator) {
    for g in groups {
        square::prune_group(&g);
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.squares
            .iter()
            .enumerate()
            .map(|(i, square)| write!(f, "{:?}{}", &square, (if i % 9 == 8 { "\n" } else { " " })))
            .collect()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.squares
            .iter()
            .enumerate()
            .map(|(i, square)| {
                write!(
                    f,
                    "{}{}",
                    &square.borrow().clone(),
                    (if i % 9 == 8 { "\n" } else { " " })
                )
            }).collect()
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err("Grids must be 81 squares long".into());
        }

        s.chars()
            .map(square::from_char)
            .collect::<Result<Grid, Self::Err>>()
            .map(Board::new)
            .and_then(|brd| {
                if brd.is_valid() {
                    Ok(brd.prune())
                } else {
                    Err("Unsolvable board!".into())
                }
            })
    }
}
