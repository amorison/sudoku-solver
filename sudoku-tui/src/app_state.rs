use std::collections::BTreeSet;

use sudoku_solver::{Puzzle, Grid, Value};
use crate::counter::{CounterUpTo, count_saturated};

/// State of application, contains the sudoku puzzle.
pub struct App {
    puzzle: Puzzle,
    cur_row: usize,
    cur_col: usize,
    solution: Option<Grid<u8>>,
    n_sols: CounterUpTo,
    all_sols: Grid<BTreeSet<u8>>,
}

/// All the possibilities for a cell in a sudoku puzzle.
pub enum CellValue {
    /// The value is set by the puzzle.
    Pinned(u8),
    /// The value of the first solution found for the current puzzle.
    Solution(u8),
    /// The current puzzle has no solution.
    NoSolution,
}

/// A direction to move on the grid.
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftBlock,
    RightBlock,
    UpBlock,
    DownBlock,
}

impl App {
    /// This keeps the solution and diagnostics up-to-date with the puzzle.
    /// This has to be called everytime the puzzle is changed.
    fn update_solution(&mut self) {
        self.solution = self.puzzle.solutions().next();
        self.n_sols = count_saturated(&mut self.puzzle.solutions(), 1000);
        self.all_sols = self.puzzle.possible_values();
    }

    /// Set the value of the puzzle at the cursor position.
    pub fn pin_current(&mut self, value: u8) {
        let (row, col) = self.current_pos();
        let val = Value::new(value);
        if self.puzzle.get(row, col) != Some(val) {
            self.puzzle.pin(row, col, Value::new(value));
            self.update_solution();
        }
    }

    /// Unset the value of the puzzle at the cursor position.
    pub fn unpin_current(&mut self) {
        let (row, col) = self.current_pos();
        if self.puzzle.get(row, col).is_some() {
            self.puzzle.unpin(row, col);
            self.update_solution();
        }
    }

    /// Return the current cursor position.
    pub fn current_pos(&self) -> (usize, usize) {
        (self.cur_row, self.cur_col)
    }

    /// The cell value at a given position.
    pub fn value_at(&self, row: usize, col: usize) -> CellValue {
        if let Some(v) = self.puzzle.get(row, col) {
            CellValue::Pinned(v.value())
        } else if let Some(sol) = self.solution {
            CellValue::Solution(sol[row][col])
        } else {
            CellValue::NoSolution
        }
    }

    /// All possible values that give a solvable puzzle.
    pub fn all_vals_at(&self, row: usize, col: usize) -> &BTreeSet<u8> {
        &self.all_sols[row][col]
    }

    /// Move the cursor in a given direction.
    pub fn move_pos(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.cur_col = (self.cur_col + 8) % 9,
            Direction::Right => self.cur_col  = (self.cur_col + 1) % 9,
            Direction::Up => self.cur_row  = (self.cur_row + 8) % 9,
            Direction::Down => self.cur_row  = (self.cur_row + 1) % 9,
            Direction::LeftBlock => self.cur_col  = (self.cur_col + 6) % 9,
            Direction::RightBlock => self.cur_col  = (self.cur_col + 3) % 9,
            Direction::UpBlock => self.cur_row  = (self.cur_row + 6) % 9,
            Direction::DownBlock => self.cur_row  = (self.cur_row + 3) % 9,
        }
    }

    /// Number of solutions of the current puzzle.
    pub fn n_solutions(&self) -> CounterUpTo {
        self.n_sols
    }
}

impl Default for App {

    fn default() -> Self {
        let mut app = Self {
            puzzle: Default::default(),
            cur_row: 0,
            cur_col: 0,
            // these values don't matter, they are updated immediately after.
            n_sols: CounterUpTo::Exactly(0),
            solution: Default::default(),
            all_sols: Default::default(),
        };
        app.update_solution();
        app
    }
}
