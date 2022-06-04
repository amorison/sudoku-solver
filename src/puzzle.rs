use std::collections::BTreeSet;
use std::num::NonZeroU8;

use crate::solver::{PossibleValuesFinder, SolutionIterator};

/// Sudoku-shaped array holding a given type.
pub type Grid<T> = [[T; 9]; 9];

/// Represent a valid cell value, i.e. an integer between 1 and 9 inclusive.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct Value(NonZeroU8);

impl Value {
    /// Create a new value.
    ///
    /// # Panics
    ///
    /// Panics if `val` is not between 1 and 9 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_solver::Value;
    /// let val = Value::new(3);  // represent a cell with the value 3
    /// # assert_eq!(val.value(), 3);
    /// ```
    ///
    /// ```should_panic
    /// use sudoku_solver::Value;
    /// let val = Value::new(0);
    /// ```
    pub fn new(val: u8) -> Self {
        assert!(val > 0 && val < 10, "Cell value is from 1 to 9, got {val}.");
        Self(val.try_into().unwrap())
    }

    /// The cell value, always between 1 and 9 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_solver::Value;
    /// let val = Value::new(7);
    /// assert_eq!(val.value(), 7);
    /// ```
    pub fn value(&self) -> u8 {
        self.0.get()
    }
}

/// A sudoku grid puzzle.  This type is the main API of the library as it
/// allows you to define your sudoku puzzle and solve it.
///
/// # Note
///
/// The [`Puzzle`] doesn't have to be well-posed: it can have 0 or several
/// solutions.  The [`Puzzle::solutions`] method offers an iterator through
/// all the solutions of the puzzle.
///
/// # Example
///
/// ```
/// use sudoku_solver::{Puzzle, Value};
///
/// // Create a puzzle with no constraint, essentially an empty grid.
/// let mut pzl = Puzzle::default();
/// assert_eq!(pzl.get(0, 0), None);
///
/// // Set the top-left cell to 3.
/// pzl.pin(0, 0, Value::new(3));
/// assert_eq!(pzl.get(0, 0), Some(Value::new(3)));
///
/// // Delete that constraint.
/// pzl.unpin(0, 0);
/// assert_eq!(pzl.get(0, 0), None);
///
/// // Get the first solution found.
/// let sol = pzl.solutions().next().unwrap();
/// // Note that the solving algorithm is deterministic, so for an empty
/// // problem you will always get this solution first.
/// let expected = [
///     [1, 2, 3, 4, 5, 6, 7, 8, 9],
///     [4, 5, 6, 7, 8, 9, 1, 2, 3],
///     [7, 8, 9, 1, 2, 3, 4, 5, 6],
///     [2, 3, 1, 6, 7, 4, 8, 9, 5],
///     [8, 7, 5, 9, 1, 2, 3, 6, 4],
///     [6, 9, 4, 5, 3, 8, 2, 1, 7],
///     [3, 1, 7, 2, 6, 5, 9, 4, 8],
///     [5, 4, 2, 8, 9, 7, 6, 3, 1],
///     [9, 6, 8, 3, 4, 1, 5, 7, 2]
/// ];
/// assert_eq!(sol, expected);
/// ```
#[derive(Default, Clone)]
pub struct Puzzle(Grid<Option<Value>>);

impl Puzzle {
    /// Build a [`Puzzle`] from an array of values.  Zeros are seen as non-constrained cells.
    ///
    /// # Panics
    ///
    /// Panics if an element is not between 0 and 9 inclusive.
    ///
    /// # Example
    ///
    /// ```
    /// use sudoku_solver::{Puzzle, Value};
    /// let mut arr = [[0; 9]; 9];
    /// // Set the center cell of the second row to 1.
    /// arr[2][4] = 1;
    /// let pzl = Puzzle::from_arr(arr);
    /// assert_eq!(pzl.get(2, 4).unwrap(), Value::new(1));
    /// assert_eq!(pzl.get(0, 0), None);
    /// ```
    pub fn from_arr(arr: Grid<u8>) -> Self {
        let inner = arr.map(|row| row.map(|v| (v != 0).then(|| Value::new(v))));
        Puzzle(inner)
    }

    /// Get the [`Value`] at a given position.  This is 0-indexed.
    pub fn get(&self, row: usize, col: usize) -> Option<Value> {
        self.0[row][col]
    }

    /// Set the [`Value`] at a given position.  This is 0-indexed.
    pub fn pin(&mut self, row: usize, col: usize, val: Value) {
        self.0[row][col] = Some(val);
    }

    /// Unset the [`Value`] at a given position.  This is 0-indexed.
    pub fn unpin(&mut self, row: usize, col: usize) {
        self.0[row][col] = None;
    }

    /// Create an iterator through all the solutions of the [`Puzzle`].
    ///
    /// # Example
    ///
    /// ```
    /// use sudoku_solver::Puzzle;
    ///
    /// // Build a puzzle with two solutions.
    /// let p = Puzzle::from_arr([
    ///     [0, 0, 3, 4, 5, 6, 7, 8, 9],  // this row can start with 1,2 or 2,1
    ///     [4, 5, 6, 7, 8, 9, 1, 2, 3],
    ///     [7, 8, 9, 1, 2, 3, 4, 5, 6],
    ///     [0, 0, 4, 8, 3, 5, 9, 6, 7],  // this one can start with 2,1 or 1,2
    ///     [8, 3, 5, 6, 9, 7, 2, 1, 4],
    ///     [6, 9, 7, 2, 1, 4, 5, 3, 8],
    ///     [3, 4, 2, 5, 7, 8, 6, 9, 1],
    ///     [5, 6, 8, 9, 4, 1, 3, 7, 2],
    ///     [9, 7, 1, 3, 6, 2, 8, 4, 5]
    /// ]);
    ///
    /// // Both expected solutions.
    /// let expected1 = [
    ///     [1, 2, 3, 4, 5, 6, 7, 8, 9],
    ///     [4, 5, 6, 7, 8, 9, 1, 2, 3],
    ///     [7, 8, 9, 1, 2, 3, 4, 5, 6],
    ///     [2, 1, 4, 8, 3, 5, 9, 6, 7],
    ///     [8, 3, 5, 6, 9, 7, 2, 1, 4],
    ///     [6, 9, 7, 2, 1, 4, 5, 3, 8],
    ///     [3, 4, 2, 5, 7, 8, 6, 9, 1],
    ///     [5, 6, 8, 9, 4, 1, 3, 7, 2],
    ///     [9, 7, 1, 3, 6, 2, 8, 4, 5]
    /// ];
    /// let expected2 = [
    ///     [2, 1, 3, 4, 5, 6, 7, 8, 9],
    ///     [4, 5, 6, 7, 8, 9, 1, 2, 3],
    ///     [7, 8, 9, 1, 2, 3, 4, 5, 6],
    ///     [1, 2, 4, 8, 3, 5, 9, 6, 7],
    ///     [8, 3, 5, 6, 9, 7, 2, 1, 4],
    ///     [6, 9, 7, 2, 1, 4, 5, 3, 8],
    ///     [3, 4, 2, 5, 7, 8, 6, 9, 1],
    ///     [5, 6, 8, 9, 4, 1, 3, 7, 2],
    ///     [9, 7, 1, 3, 6, 2, 8, 4, 5]
    /// ];
    ///
    /// let mut sols = p.solutions();
    /// assert_eq!(sols.next().unwrap(), expected1);
    /// assert_eq!(sols.next().unwrap(), expected2);
    /// assert!(sols.next().is_none());
    /// ```
    pub fn solutions(&self) -> SolutionIterator {
        SolutionIterator::new(self)
    }

    /// Compute the set of values in each cell that lead to a solvable grid.
    pub fn possible_values(&self) -> Grid<BTreeSet<u8>> {
        PossibleValuesFinder::new().search(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_conflict_no_sol() {
        let mut p = Puzzle::default();
        p.pin(0, 0, Value::new(1));
        p.pin(0, 1, Value::new(1));
        assert!(p.solutions().next().is_none());
    }

    #[test]
    fn cell_with_no_possibility_no_sol() {
        let mut p = Puzzle::default();
        for i in 0..6 {
            p.pin(0, i, Value::new((i + 1) as u8));
        }
        for i in 6..9 {
            p.pin(1, i, Value::new((i + 1) as u8));
        }
        assert!(p.solutions().next().is_none());
    }

    #[test]
    fn empty_puzzle_1000_sols() {
        let p = Puzzle::default();
        assert_eq!(p.solutions().take(1000).count(), 1000);
    }

    #[test]
    fn puzzle_single_sol() {
        let p = Puzzle::from_arr([
            [0, 5, 0, 0, 0, 0, 0, 0, 6],
            [0, 0, 6, 7, 3, 0, 0, 2, 0],
            [0, 0, 0, 0, 8, 0, 0, 0, 0],
            [0, 0, 5, 2, 6, 0, 0, 3, 0],
            [4, 0, 0, 0, 0, 0, 9, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 9, 0, 1, 7, 0, 3, 0, 0],
            [0, 0, 7, 0, 0, 8, 0, 0, 0],
            [0, 0, 0, 0, 0, 5, 0, 1, 0],
        ]);
        let expected = [
            [3, 5, 9, 4, 1, 2, 8, 7, 6],
            [8, 4, 6, 7, 3, 9, 5, 2, 1],
            [1, 7, 2, 5, 8, 6, 4, 9, 3],
            [9, 8, 5, 2, 6, 7, 1, 3, 4],
            [4, 2, 1, 8, 5, 3, 9, 6, 7],
            [7, 6, 3, 9, 4, 1, 2, 8, 5],
            [6, 9, 8, 1, 7, 4, 3, 5, 2],
            [5, 1, 7, 3, 2, 8, 6, 4, 9],
            [2, 3, 4, 6, 9, 5, 7, 1, 8],
        ];
        let mut sols = p.solutions();
        assert!(sols.next().unwrap() == expected);
        assert!(sols.next().is_none());
    }

    #[test]
    fn puzzle_triple_sol() {
        let p = Puzzle::from_arr([
            [3, 0, 9, 6, 0, 0, 4, 0, 0],
            [0, 0, 0, 7, 0, 9, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 0, 0],
            [7, 5, 0, 0, 6, 0, 2, 3, 0],
            [6, 0, 0, 9, 0, 4, 0, 0, 8],
            [0, 2, 8, 0, 5, 0, 0, 4, 1],
            [0, 0, 0, 0, 0, 0, 5, 9, 0],
            [0, 0, 0, 1, 9, 6, 0, 0, 7],
            [0, 0, 6, 0, 0, 0, 1, 0, 4],
        ]);
        assert_eq!(p.solutions().count(), 3);
    }
}
