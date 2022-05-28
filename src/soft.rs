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

    /// Combine two constraints, only keeping what is not allowed by
    /// the other.
    pub fn forbid(&mut self, val: Value) {
        self.0 &= ALL_ON - Self::from(val).0;
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
