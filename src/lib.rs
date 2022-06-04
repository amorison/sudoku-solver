//! Library to solve sudoku puzzles.
//!
//! This library allows you to create arbitrary sudoku puzzles and solve them.
//! The [`Puzzle`] does not have to be well-posed (in the sense of having one
//! and only one solution).  You can then use [`Puzzle::solutions`] to iterate
//! through all the solutions of your puzzle, and [`Puzzle::possible_values`]
//! to obtain the set of possible values in each cell that would lead to a
//! [`Puzzle`] with at least one solution if you were to add it as an additional
//! clue.
//!
//! # Known caveats
//!
//! The solving strategy is fairly naive. This is a backtracking algorithm with
//! two simple ingredients:
//!
//! - when making a guess in a cell (the puzzle is considered as a set of
//!   guesses that are valid), the guessed value is removed as a viable
//!   candidate in the relevant row, column, and block of the puzzle;
//! - when a guess has to be made, the cell with the fewest candidate values is
//!   picked.
//!
//! This is efficient to solve the vast majority of puzzles (either well-posed
//! or not), but will trip on some pathological cases where the space to
//! explore under this strategy is large but the set of actual solutions to the
//! puzzle is small.  Such a puzzle is the following:
//!
//! ```text
//! . . .   . . .   . 1 .
//! . . .   . . 2   . . 3
//! . . .   4 . .   . . .
//!
//! . . .   . . .   5 . 8
//! . . 1   6 . .   . . .
//! . . 7   1 . .   . . .
//!
//! . . .   . . .   . . .
//! . . .   . . .   . . .
//! . . .   . . .   . . .
//! ```
//!
//! If you build an interface around this library, you might want to avoid
//! blocking calls to the solver in case you run into such a case.  See
//! [`SolutionIterator::with_handle`] for how to call the solver in a separate
//! thread and signal it to stop.
mod puzzle;
mod soft;
mod solgrid;
mod solver;

pub use puzzle::{Grid, Puzzle, Value};
pub use solver::{PossibleValuesFinder, SolutionIterator};
