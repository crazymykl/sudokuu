use self::Square::{Fixed, Possible};
use std::fmt::{self, Debug, Display};

#[cfg(feature = "parallelism")]
pub type SquareCell = atomic_refcell::AtomicRefCell<Square>;
#[cfg(not(feature = "parallelism"))]
pub type SquareCell = std::cell::RefCell<Square>;

bitflags! {
    pub struct Possibilities: u32 {
        const _1 = 0b0000_0000_0000_0001;
        const _2 = 0b0000_0000_0000_0010;
        const _3 = 0b0000_0000_0000_0100;
        const _4 = 0b0000_0000_0000_1000;
        const _5 = 0b0000_0000_0001_0000;
        const _6 = 0b0000_0000_0010_0000;
        const _7 = 0b0000_0000_0100_0000;
        const _8 = 0b0000_0000_1000_0000;
        const _9 = 0b0000_0001_0000_0000;
    }
}

impl Possibilities {
    fn new_from_u32(x: u32) -> Self {
        match x {
            1 => Self::_1,
            2 => Self::_2,
            3 => Self::_3,
            4 => Self::_4,
            5 => Self::_5,
            6 => Self::_6,
            7 => Self::_7,
            8 => Self::_8,
            9 => Self::_9,
            _ => unreachable!(),
        }
    }

    fn pop_count(self) -> u32 {
        self.bits().count_ones()
    }

    fn single_selection(self) -> u32 {
        match self {
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_3 => 3,
            Self::_4 => 4,
            Self::_5 => 5,
            Self::_6 => 6,
            Self::_7 => 7,
            Self::_8 => 8,
            Self::_9 => 9,
            _ => unreachable!(),
        }
    }

    fn show_possibles(self, candidate: Possibilities, val: &'static str) -> &'static str {
        if self.intersects(candidate) {
            val
        } else {
            " "
        }
    }

    fn least_set(self) -> u32 {
        match self.bits().trailing_zeros() {
            32 => 0,
            n => n + 1,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Square {
    Fixed(Possibilities),
    Possible(Possibilities),
}

impl Square {
    pub fn new_fixed(x: u32) -> Self {
        Fixed(Possibilities::new_from_u32(x))
    }

    pub fn any() -> Self {
        Possible(Possibilities::all())
    }

    pub fn probability_count(&self) -> u32 {
        match self {
            Fixed(_) => 1,
            Possible(xs) => xs.pop_count(),
        }
    }

    pub fn fixed(&self) -> Option<Possibilities> {
        match *self {
            Fixed(x) => Some(x),
            Possible(_) => None,
        }
    }

    pub fn possibles(&self) -> Option<Possibilities> {
        match *self {
            Fixed(_) => None,
            Possible(xs) => Some(xs),
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed().is_some()
    }

    pub fn minus(&self, to_remove: Possibilities) -> Option<Self> {
        match *self {
            Fixed(x) => Some(Fixed(x)),
            Possible(xs) => Self::from_possibilities(xs - to_remove),
        }
    }

    pub fn split(&self) -> (Self, Self) {
        let poss = self.possibles().expect("Called split_at on Fixed");
        let val = poss.least_set();

        (
            Self::new_fixed(val),
            Self::from_possibilities(poss - Possibilities::new_from_u32(val))
                .expect("Called split_at on invalid Possible"),
        )
    }

    fn from_possibilities(values: Possibilities) -> Option<Self> {
        match values.pop_count() {
            0 => None,
            1 => Some(Fixed(values)),
            _ => Some(Possible(values)),
        }
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fixed(x) => write!(f, "{}          ", x.single_selection()),
            Possible(xs) => write!(
                f,
                "[{}{}{}{}{}{}{}{}{}]",
                xs.show_possibles(Possibilities::_1, "1"),
                xs.show_possibles(Possibilities::_2, "2"),
                xs.show_possibles(Possibilities::_3, "3"),
                xs.show_possibles(Possibilities::_4, "4"),
                xs.show_possibles(Possibilities::_5, "5"),
                xs.show_possibles(Possibilities::_6, "6"),
                xs.show_possibles(Possibilities::_7, "7"),
                xs.show_possibles(Possibilities::_8, "8"),
                xs.show_possibles(Possibilities::_9, "9"),
            ),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fixed(n) => write!(f, "{}", n.single_selection()),
            Possible(_) => write!(f, "."),
        }
    }
}

pub fn from_char(c: char) -> Result<SquareCell, String> {
    match c {
        '1'..='9' => c
            .to_digit(10)
            .map(Square::new_fixed)
            .ok_or_else(|| "IMPOSSIBLE".into()),
        '.' => Ok(Square::any()),
        _ => Err(format!("Invalid character '{}' (valid are 1-9 and .)", &c)),
    }.map(SquareCell::new)
}

fn fixed_values(group: &[&SquareCell]) -> Possibilities {
    group
        .iter()
        .filter_map(|sq| sq.borrow().fixed())
        .fold(Possibilities::empty(), |acc, sq| acc | sq)
}

pub fn valid_group(group: &[&SquareCell]) -> bool {
    fixed_values(group).pop_count()
        == group.iter().filter(|sq| sq.borrow().is_fixed()).count() as u32
}

pub fn prune_group(group: &[&SquareCell]) {
    let fixeds = fixed_values(group);

    for sq in group {
        let mut sq = sq.borrow_mut();
        if let Some(new_sq) = sq.minus(fixeds) {
            *sq = new_sq;
        }
    }
}
