use std::collections::BTreeSet;

use crate::puzzle::{Value, Puzzle, Grid};
use crate::soft::SoftConstraint;

/// Some operations return this type wrapped in an error to signal that the
/// grid has no solution.
pub struct NoSolError{}

/// Result of an operation susceptible to detect the puzzle has no solution.
pub type SolResult<T> = Result<T, NoSolError>;

/// Represent the possible values for a cell.
#[derive(Clone)]
enum CellState {
    /// A cell with a known value.
    Pinned(Value),
    /// A cell that has potentially several possible values.
    Fuzzy(SoftConstraint),
}

impl CellState {
    /// Whether the cell has a Fixed value.
    fn is_fixed(&self) -> bool {
        matches!(self, CellState::Pinned(_))
    }

    /// Extract the value in a Fixed cell.
    fn fixed_val(&self) -> Option<Value> {
        match self {
            CellState::Pinned(v) => Some(*v),
            _ => None,
        }
    }

    /// Extract a SoftConstraint in a Fuzzy cell.
    fn fuzzy_constraint(&self) -> Option<SoftConstraint> {
        match self {
            CellState::Fuzzy(sc) => Some(*sc),
            _ => None,
        }
    }

    fn pin(&mut self, val: Value) -> SolResult<()> {
        match self {
            CellState::Pinned(v) if val == *v => Ok(()),
            CellState::Fuzzy(sc) if sc.has_solution(val) => {
                *self = CellState::Pinned(val);
                Ok(())
            },
            _ => Err(NoSolError{}),
        }
    }

    fn forbid(&mut self, val: Value) -> SolResult<()> {
        match self {
            CellState::Pinned(v) if val != *v => Ok(()),
            CellState::Fuzzy(sc) => {
                sc.forbid(val);
                (sc.has_solutions()).then(|| ()).ok_or(NoSolError{})
            },
            _ => Err(NoSolError{}),
        }
    }

    fn all_values(&self) -> BTreeSet<Value> {
        match self {
            CellState::Pinned(v) => {
                let mut bset = BTreeSet::new();
                bset.insert(*v);
                bset
            },
            CellState::Fuzzy(sc) => {
                sc.all_values()
            }
        }
    }
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Fuzzy(SoftConstraint::default())
    }
}

/// A sudoku grid solution.
#[derive(Clone, Default)]
pub struct SolutionGrid([[CellState; 9]; 9]);

impl SolutionGrid {
    pub fn is_solved(&self) -> bool {
        self.0.iter()
            .flatten()
            .all(|cs| cs.is_fixed())
    }

    pub fn pin(&mut self, row: usize, col: usize, val: Value) -> SolResult<()> {
        self.0[row][col].pin(val)?;
        for i in 0..9 {
            if i != row {
                self.forbid(i, col, val)?;
            }
            if i != col {
                self.forbid(row, i, val)?;
            }
        }
        let block_row = row / 3;
        let block_col = col / 3;
        for ir in (3 * block_row)..(3 * (block_row + 1)) {
            for ic in (3 * block_col)..(3 * (block_col + 1)) {
                if ir != row && ic != col {
                    self.forbid(ir, ic, val)?;
                }
            }
        }
        Ok(())
    }

    pub fn forbid(&mut self, row: usize, col: usize, val: Value) -> SolResult<()> {
        self.0[row][col].forbid(val)
    }

    /// Find all fuzzy cells with only one possibility left.
    fn find_fuzzy_uniques(&self) -> Vec<(usize, usize, Value)> {
        let mut out = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                if let Some(sc) = self.0[row][col].fuzzy_constraint() {
                    if let Some(val) = sc.unique_solution() {
                        out.push((row, col, val))
                    }
                }
            }
        }
        out
    }

    pub fn maximize_constraints(&mut self) -> SolResult<()> {
        loop {
            let uniques = self.find_fuzzy_uniques();
            if uniques.is_empty() {
                break Ok(());
            }
            for (row, col, val) in uniques {
                self.pin(row, col, val)?;
            }
        }
    }

    /// Find the fuzzy cell with the least possibilities.
    pub fn find_least_sols_fuzzy(&self) -> Option<(usize, usize, SoftConstraint)> {
        let mut min_nsols = 10;
        let mut out = None;
        for row in 0..9 {
            for col in 0..9 {
                if let Some(sc) = self.0[row][col].fuzzy_constraint() {
                    let nsols = sc.num_solutions();
                    if nsols < min_nsols {
                        out = Some((row, col, sc));
                        min_nsols = nsols;
                    }
                }
            }
        }
        out
    }

    /// All possible values that have not been ruled out yet.
    pub fn possible_values(&self, row: usize, col: usize) -> BTreeSet<Value> {
        self.0[row][col].all_values()
    }
}

impl TryFrom<&Puzzle> for SolutionGrid {
    type Error = NoSolError;

    fn try_from(value: &Puzzle) -> SolResult<Self> {
        let mut sgrid = Self::default();
        for row in 0..9 {
            for col in 0..9 {
                if let Some(c) = value.get(row, col) {
                    sgrid.pin(row, col, c)?;
                }
            }
        }
        sgrid.maximize_constraints()?;
        Ok(sgrid)
    }
}

impl TryFrom<SolutionGrid> for Grid<u8> {
    type Error = ();

    fn try_from(value: SolutionGrid) -> Result<Self, Self::Error> {
        if value.is_solved() {
            Ok(value.0.map(|r| r.map(|cs| {
                    cs.fixed_val().unwrap().value()
            })))
        } else {
            Err(())
        }
    }
}

