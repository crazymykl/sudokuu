use crate::possibilites::Possibilities;
use std::cell::Cell;
use std::fmt::{self, Debug, Display};

#[derive(PartialEq, Eq, Clone)]
pub struct Square(Cell<Possibilities>);

impl Square {
    fn new(possible: Possibilities) -> Self {
        Square(Cell::new(possible))
    }

    pub fn new_fixed(x: u32) -> Self {
        Self::new(Possibilities::new_from_u32(x))
    }

    pub fn any() -> Self {
        Self::new(Possibilities::all())
    }

    pub fn possibles_count(&self) -> u32 {
        self.get().pop_count()
    }

    pub fn fixed(&self) -> Option<Possibilities> {
        match self.possibles_count() {
            1 => Some(self.get()),
            _ => None,
        }
    }

    pub fn possibles(&self) -> Option<Possibilities> {
        match self.possibles_count() {
            1 => None,
            _ => Some(self.get()),
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed().is_some()
    }

    pub fn minus(&self, to_remove: Possibilities) {
        self.set(self.get() - to_remove)
    }

    pub fn split(&self) -> (Self, Self) {
        let poss = self.possibles().expect("Called split_at on a fixed square");
        let val = poss.least_set();

        (
            Self::new_fixed(val),
            Self::new(poss - Possibilities::new_from_u32(val)),
        )
    }

    #[inline]
    fn get(&self) -> Possibilities {
        self.0.get()
    }

    #[inline]
    fn set(&self, val: Possibilities) {
        self.0.set(val)
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.possibles_count() {
            0 => write!(f, "***********"),
            1 => write!(f, "{}          ", self.get().single_selection()),
            _ => write!(
                f,
                "[{}{}{}{}{}{}{}{}{}]",
                self.get().show_possibles(Possibilities::_1, "1"),
                self.get().show_possibles(Possibilities::_2, "2"),
                self.get().show_possibles(Possibilities::_3, "3"),
                self.get().show_possibles(Possibilities::_4, "4"),
                self.get().show_possibles(Possibilities::_5, "5"),
                self.get().show_possibles(Possibilities::_6, "6"),
                self.get().show_possibles(Possibilities::_7, "7"),
                self.get().show_possibles(Possibilities::_8, "8"),
                self.get().show_possibles(Possibilities::_9, "9"),
            ),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.possibles_count() {
            1 => write!(f, "{}", self.get().single_selection()),
            _ => write!(f, "."),
        }
    }
}

pub fn from_char(c: char) -> Result<Square, String> {
    match c {
        '1'..='9' => c
            .to_digit(10)
            .map(Square::new_fixed)
            .ok_or_else(|| "IMPOSSIBLE".into()),
        '.' => Ok(Square::any()),
        _ => Err(format!("Invalid character '{}' (valid are 1-9 and .)", &c)),
    }
}

fn fixed_values(group: &[&Square]) -> Possibilities {
    group
        .iter()
        .filter_map(|sq| sq.fixed())
        .fold(Possibilities::empty(), |acc, sq| acc | sq)
}

pub fn valid_group(group: &[&Square]) -> bool {
    group
        .iter()
        .fold(Some(Possibilities::empty()), |acc, sq| {
            acc.and_then(|fixeds| {
                let possibles = sq.get();
                if fixeds.contains(possibles) {
                    None
                } else if possibles.pop_count() == 1 {
                    Some(fixeds | possibles)
                } else {
                    Some(fixeds)
                }
            })
        }).is_some()
}

pub fn prune_group(group: &[&Square]) {
    let fixeds = fixed_values(group);

    group
        .iter()
        .filter(|sq| !sq.is_fixed())
        .for_each(|sq| sq.minus(fixeds));
}
