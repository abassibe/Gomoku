use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash};
use std::ops::Neg;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Fscore {
    Uninitialized,
    Value(isize),
    Win
}

impl Fscore {
    pub const MIN: Self = Fscore::Value(isize::MIN);
    pub const MAX: Self = Fscore::Value(isize::MAX);

    pub fn is_win(&self) -> bool {
        *self == Fscore::Win
    }

    pub fn is_initialized(&self) -> bool {
        *self != Fscore::Uninitialized
    }
}

impl Default for Fscore {
    fn default() -> Self {
        Fscore::Uninitialized
    }
}

impl Neg for Fscore{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Fscore::Uninitialized => self,
            Fscore::Win => self,
            Fscore::Value(x) => Fscore::Value(x.neg())
        }
    }
}

impl Ord for Fscore {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_has_value = matches!(self, Fscore::Value(_));
        let other_has_value = matches!(other, Fscore::Value(_));
        if !self_has_value || !other_has_value {
            let self_as_u8: u8 = self.into();
            return self_as_u8.cmp(&other.into());
        }
        // At this point both `self` and `other` should be of type `Fscore::Value`
        let self_value = if let Fscore::Value(x) = self {
            *x
        } else {
            isize::MIN
        };
        let other_value = if let Fscore::Value(x) = other {
            *x
        } else {
            isize::MIN
        };
        self_value.cmp(&other_value)
    }
}

impl PartialOrd for Fscore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<u8> for Fscore {
    fn into(self) -> u8 {
        match self {
            Fscore::Uninitialized => 0,
            Fscore::Value(_) => 1,
            Fscore::Win => 2
        }
    }
}

impl Into<u8> for &Fscore {
    fn into(self) -> u8 {
        match self {
            Fscore::Uninitialized => 0,
            Fscore::Value(_) => 1,
            Fscore::Win => 2
        }
    }
}

impl fmt::Display for Fscore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fscore::Uninitialized => "Uninitialized".into(),
                Fscore::Value(x) => x.to_string(),
                Fscore::Win => "Win".into()
            }
        )
    }
}