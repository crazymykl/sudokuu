use std::fmt::{self, Debug, Display};

use self::Square::{Fixed, Possible};

#[derive(PartialEq, Eq, Clone)]
pub enum Square {
    Fixed(u8),
    Possible(Vec<u8>),
}

impl Square {
    pub fn new(values: &[u8]) -> Option<Self> {
        match values {
            [] => None,
            [x] => Some(Fixed(*x)),
            xs => Some(Possible(xs.to_vec())),
        }
    }

    pub fn probability_count(&self) -> usize {
        match self {
            Fixed(_) => 1,
            Possible(arr) => arr.len(),
        }
    }

    pub fn possibles(&self) -> Option<Vec<u8>> {
        match self {
            Fixed(_) => None,
            Possible(ys) => Some(ys.clone()),
        }
    }

    pub fn is_fixed(&self) -> bool {
        match self {
            Fixed(_) => true,
            _ => false,
        }
    }

    pub fn minus(&self, to_remove: &[u8]) -> Option<Self> {
        match self {
            Fixed(x) => Some(Fixed(*x)),
            Possible(xs) => Square::new(
                &xs.iter()
                    .filter(|i| !to_remove.contains(i))
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fixed(n) => write!(f, "{}          ", n),
            Possible(arr) => write!(
                f,
                "[{}{}{}{}{}{}{}{}{}]",
                if arr.contains(&1) { "1" } else { " " },
                if arr.contains(&2) { "2" } else { " " },
                if arr.contains(&3) { "3" } else { " " },
                if arr.contains(&4) { "4" } else { " " },
                if arr.contains(&5) { "5" } else { " " },
                if arr.contains(&6) { "6" } else { " " },
                if arr.contains(&7) { "7" } else { " " },
                if arr.contains(&8) { "8" } else { " " },
                if arr.contains(&9) { "9" } else { " " },
            ),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fixed(n) => write!(f, "{}", n),
            Possible(_) => write!(f, "."),
        }
    }
}
