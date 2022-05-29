use std::collections::BTreeSet;

use crate::puzzle::Value;

const ALL_ON: u16 = (1 << 9) - 1;

#[derive(Copy, Clone)]
pub struct SoftConstraint(u16);

impl SoftConstraint {
    pub fn unique_solution(&self) -> Option<Value> {
        if self.num_solutions() == 1 {
            let mut i = 1;
            let mut inner: u16 = self.0;
            while inner != 1 {
                i += 1;
                inner /= 2;
            }
            Some(Value::new(i))
        } else {
            None
        }
    }

    pub fn smallest_solution(&self) -> Option<Value> {
        if self.0 != 0 {
            let mut i = 1;
            let mut inner: u16 = self.0;
            while inner % 2 != 1 {
                i += 1;
                inner /= 2;
            }
            Some(Value::new(i))
        } else {
            None
        }
    }

    pub fn has_solutions(&self) -> bool {
        self.0 != 0
    }

    pub fn has_solution(&self, constraint: Value) -> bool {
        self.0 & (1 << (constraint.value() - 1)) != 0
    }

    pub fn num_solutions(&self) -> u32 {
        self.0.count_ones()
    }

    /// Remove a given value of the possible values.
    pub fn forbid(&mut self, val: Value) {
        self.0 &= ALL_ON - Self::from(val).0;
    }

    /// Iterate through possible values.
    pub fn all_values(&self) -> BTreeSet<Value> {
        (1..=9).map(Value::new).filter(|&v| self.has_solution(v)).collect()
    }
}

impl Default for SoftConstraint {
    fn default() -> Self {
        SoftConstraint(ALL_ON)
    }
}

impl From<Value> for SoftConstraint {
    fn from(value: Value) -> Self {
        SoftConstraint(1 << (value.value() - 1))
    }
}

impl From<Option<Value>> for SoftConstraint {
    fn from(hconst: Option<Value>) -> Self {
        match hconst {
            Some(hc) => hc.into(),
            None => SoftConstraint::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn default_num_sols() {
        let sc = SoftConstraint::default();
        assert!(sc.unique_solution().is_none());
        assert_eq!(sc.smallest_solution().unwrap().value(), 1);
        assert!(sc.has_solutions());
        for i in 1..=9 {
            assert!(sc.has_solution(Value::new(i)));
        }
        assert_eq!(sc.num_solutions(), 9);
    }

    #[test]
    fn from_val() {
        let sc = SoftConstraint::from(Value::new(6));
        assert_eq!(sc.unique_solution().unwrap(), Value::new(6));
        assert_eq!(sc.smallest_solution().unwrap().value(), 6);
        assert!(sc.has_solutions());
        for i in 1..=9 {
            assert_eq!(sc.has_solution(Value::new(i)), i == 6);
        }
        assert_eq!(sc.num_solutions(), 1);
    }

    #[test]
    fn from_opt_val() {
        let sc = SoftConstraint::from(Some(Value::new(2)));
        assert_eq!(sc.unique_solution().unwrap(), Value::new(2));
        assert_eq!(sc.smallest_solution().unwrap().value(), 2);
        assert!(sc.has_solutions());
        for i in 1..=9 {
            assert_eq!(sc.has_solution(Value::new(i)), i == 2);
        }
        assert_eq!(sc.num_solutions(), 1);
    }

    #[test]
    fn from_none() {
        let sc = SoftConstraint::from(None);
        assert!(sc.unique_solution().is_none());
        assert_eq!(sc.smallest_solution().unwrap().value(), 1);
        assert!(sc.has_solutions());
        for i in 1..=9 {
            assert!(sc.has_solution(Value::new(i)));
        }
        assert_eq!(sc.num_solutions(), 9);
    }

    #[test]
    fn forbid() {
        let mut sc = SoftConstraint::from(None);
        sc.forbid(Value::new(1));
        assert!(sc.unique_solution().is_none());
        assert_eq!(sc.smallest_solution().unwrap().value(), 2);
        assert!(sc.has_solutions());
        for i in 1..=9 {
            assert_eq!(sc.has_solution(Value::new(i)), i != 1);
        }
        assert_eq!(sc.num_solutions(), 8);
    }
}
