use std::cell::RefCell;
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use super::programmed_iterator::ProgrammedIterator;
use super::square::Square::{self, Fixed, Possible};

pub type Grid = Vec<RefCell<Square>>;

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
        self.clone().prune_rec(self.prune_step())
    }

    pub fn solve(&self) -> Option<Self> {
        let pruned = self.prune();

        if !pruned.is_valid() {
            None
        } else if pruned.is_solved() {
            Some(pruned)
        } else {
            let (left, right) = pruned.next_grids();

            left.solve().or_else(|| right.solve())
        }
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
        if let Some(ys) = self.squares.get(idx).map(|sq| sq.borrow().possibles()) {
            let (mut first, mut rest) = (self.clone(), self.clone());

            *first.squares[idx].borrow_mut() = Fixed(ys[0]);
            *rest.squares[idx].borrow_mut() = Square::new(&ys[1..])
                .unwrap_or_else(|| panic!("Illegal split_at index {}: {:?}", idx, self));

            (first, rest)
        } else {
            panic!("Attempt to split_at impossible index {}: {:?}", idx, self)
        }
    }

    fn prune_rec(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            let step = other.prune_step();
            other.prune_rec(step)
        }
    }

    fn prune_step(&self) -> Self {
        prune_groups(self.rows())
            .and_then(|brd| prune_groups(brd.cols()).map(|brd| brd.cols().into_board()))
            .and_then(|brd| prune_groups(brd.boxes()).map(|brd| brd.boxes().into_board()))
            .unwrap_or_else(|| self.clone())
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
    groups.all(|g| valid_group(&g))
}

fn valid_group(group: &[&Square]) -> bool {
    group
        .iter()
        .fold(Some(vec![]), |acc, c| match c {
            Possible(xs) => if xs.is_empty() {
                None
            } else {
                acc
            },
            Fixed(x) => acc.and_then(|mut a| {
                if a.contains(x) {
                    None
                } else {
                    a.push(*x);
                    Some(a)
                }
            }),
        }).is_some()
}

fn prune_groups(groups: ProgrammedIterator) -> Option<Board> {
    groups
        .map(|g| prune_group(&g))
        .collect::<Option<Vec<_>>>()
        .map(|g| Board::new(g.concat()))
}

fn prune_group(group: &[&Square]) -> Option<Vec<Square>> {
    let fixeds = group
        .iter()
        .filter_map(|c| match c {
            Fixed(x) => Some(x),
            _ => None,
        }).cloned()
        .collect::<Vec<_>>();

    group.iter().map(|c| c.minus(&fixeds)).collect()
}

fn read_square(c: char) -> Result<Square, String> {
    match c {
        '1'..='9' => c
            .to_digit(10)
            .map_or(Err("IMPOSSIBLE".into()), |x| Ok(Square::Fixed(x as u8))),
        '.' => Ok(Square::Possible(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])),
        _ => Err(format!("Invalid character '{}' (valid are 1-9 and .)", &c)),
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
            .map(|(i, square)| write!(f, "{}{}", &square, (if i % 9 == 8 { "\n" } else { " " })))
            .collect()
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err("Grids must be 81 squares long".into());
        }

        s.chars()
            .map(read_square)
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
