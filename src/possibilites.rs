use bitflags::bitflags;

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
    pub fn new_from_u32(x: u32) -> Self {
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

    pub fn each() -> [Self; 9] {
        [
            Self::_1,
            Self::_2,
            Self::_3,
            Self::_4,
            Self::_5,
            Self::_6,
            Self::_7,
            Self::_8,
            Self::_9,
        ]
    }

    pub fn pop_count(self) -> u32 {
        self.bits().count_ones()
    }

    pub fn single_selection(self) -> u32 {
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

    pub fn show_possibles(self, candidate: Possibilities, val: &'static str) -> &'static str {
        if self.intersects(candidate) {
            val
        } else {
            " "
        }
    }

    pub fn least_set(self) -> u32 {
        match self.bits().trailing_zeros() {
            32 => 0,
            n => n + 1,
        }
    }
}

#[derive(Debug)]
pub struct Map(Vec<(Possibilities, Possibilities)>);

impl Map {
    pub fn new() -> Self {
        Map(vec![])
    }

    pub fn add_possible(&mut self, key: Possibilities, val: Possibilities) {
        if let Some(pair) = self.0.iter_mut().find(|(k, _v)| *k == key) {
            *pair = (key, pair.1 | val);
        } else {
            self.0.push((key, val))
        }
    }
}

impl IntoIterator for Map {
    type Item = (Possibilities, Possibilities);
    type IntoIter = ::std::vec::IntoIter<(Possibilities, Possibilities)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
