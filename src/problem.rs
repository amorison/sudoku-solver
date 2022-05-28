use std::num::NonZeroU8;

use crate::solver::SolutionIterator;

/// Represent a cell that is forced to a given value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Value(NonZeroU8);

impl Value {
    /// Create a new constrained value.
    ///
    /// # Panics
    ///
    /// Panics if `val` is not between 1 and 9 inclusive.
    pub fn new(val: u8) -> Self {
        assert!(val > 0 && val < 10, "Cell value is from 1 to 9, got {val}.");
        Self(val.try_into().unwrap())
    }

    /// The cell value required by the constraint.
    pub fn value(&self) -> u8 {
        self.0.get()
    }
}

/// A sudoku grid problem, which is a set of constraint.
#[derive(Default)]
pub struct Puzzle([[Option<Value>; 9]; 9]);

impl Puzzle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a Problem from an array of values.  Zeros are ignored.
    ///
    /// # Panics
    ///
    /// Panics if an element is not between 0 and 9 inclusive.
    pub fn from_arr(arr: [[u8; 9]; 9]) -> Self {
        let inner = arr.map(|row| {
            row.map(|v| {
                (v != 0).then(|| Value::new(v))
            })
        });
        Puzzle(inner)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Value> {
        self.0[row][col]
    }

    pub fn pin(&mut self, row: usize, col: usize, val: Value) {
        self.0[row][col] = Some(val);
    }

    pub fn unpin(&mut self, row: usize, col: usize) {
        self.0[row][col] = None;
    }

    pub fn solutions(&self) -> SolutionIterator {
        SolutionIterator::with_constraints(self)
    }
}
