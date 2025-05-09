use std::fmt::{Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    floor: u8,
    ceil: u8,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.floor, self.ceil)
    }
}

impl Range {
    fn _new(floor: u8, ceil: u8) -> Range {
        Range { floor, ceil }
    }

    pub fn from_str(s: &str) -> Range {
        let (floor, ceil) = s.split('-').collect_tuple().unwrap();
        Range {
            floor: floor.parse().unwrap(),
            ceil: ceil.parse().unwrap(),
        }
    }

    pub fn inside(&self, other: &Self) -> bool {
        if self.floor < other.floor { return false; }
        if self.ceil > other.ceil { return false ;}
        true
    }

    pub fn either_inside(&self, other: &Self) -> bool {
        self.inside(other) || other.inside(self)
    }

    pub fn overlap(&self, other: &Self) -> bool {
        if self.floor <= other.ceil && self.ceil >= other.floor {
            return true;
        }
        false
    }
}

#[test]
fn test_range() {
    let (r1, r2) = (Range::_new(50, 65), Range::_new(40, 70));
    assert!(r1.inside(&r2));
    assert!(!r2.inside(&r1));
    let (r1, r2) = (Range::from_str("10-20"), Range::from_str("10-20"));
    assert!(r1.inside(&r2));
    assert!(r2.inside(&r1));
    assert!(r1.overlap(&r2));
    let (r1, r2) = (Range::_new(1, 2), Range::_new(3, 4));
    assert!(!r1.overlap(&r2));

}
