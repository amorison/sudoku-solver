use std::iter::FusedIterator;
use crate::soft::SoftConstraint;
use crate::problem::{Value, Problem};

/// Some operations return this type wrapped in an error to signal that the
/// grid has no solution.
struct NoSolError{}
type SolResult<T> = Result<T, NoSolError>;

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
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Fuzzy(SoftConstraint::default())
    }
}

/// A sudoku grid solution.
#[derive(Clone, Default)]
struct SolutionGrid([[CellState; 9]; 9]);

impl SolutionGrid {
    fn is_solved(&self) -> bool {
        self.0.iter()
            .flatten()
            .all(|cs| cs.is_fixed())
    }

    fn constrain(&mut self, row: usize, col: usize, val: Value) -> SolResult<()> {
        self.0[row][col].pin(val)?;
        for i in 0..9 {
            if i != row {
                self.0[i][col].forbid(val)?;
            }
            if i != col {
                self.0[row][i].forbid(val)?;
            }
        }
        let block_row = row / 3;
        let block_col = col / 3;
        for ir in (3 * block_row)..(3 * (block_row + 1)) {
            for ic in (3 * block_col)..(3 * (block_col + 1)) {
                if ir != row && ic != col {
                    self.0[ir][ic].forbid(val)?;
                }
            }
        }
        Ok(())
    }

    fn forbid(&mut self, row: usize, col: usize, val: Value) -> SolResult<()> {
        self.0[row][col].forbid(val)
    }

    /// Find all fuzzy cells with only one possibility left.
    fn find_fuzzy_uniques(&self) -> Vec<(usize, usize, Value)> {
        let mut out = Vec::new();
        for row in 1..9 {
            for col in 1..9 {
                if let Some(sc) = self.0[row][col].fuzzy_constraint() {
                    if let Some(val) = sc.unique_solution() {
                        out.push((row, col, val))
                    }
                }
            }
        }
        out
    }

    fn maximize_constraints(&mut self) -> SolResult<()> {
        loop {
            let uniques = self.find_fuzzy_uniques();
            if uniques.is_empty() {
                break Ok(());
            }
            for (row, col, val) in uniques {
                self.constrain(row, col, val)?;
            }
        }
    }

    /// Find the fuzzy cell with the least possibilities.
    fn find_least_sols_fuzzy(&self) -> Option<(usize, usize, SoftConstraint)> {
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
}

impl TryFrom<&Problem> for SolutionGrid {
    type Error = NoSolError;

    fn try_from(value: &Problem) -> SolResult<Self> {
        let mut sgrid = Self::default();
        for row in 0..9 {
            for col in 0..9 {
                if let Some(c) = value.get(row, col) {
                    sgrid.constrain(row, col, c)?;
                }
            }
        }
        sgrid.maximize_constraints()?;
        Ok(sgrid)
    }
}

pub struct SolutionIterator {
    stack: Vec<SolutionGrid>,
}

impl SolutionIterator {
    pub fn with_constraints(problem: &Problem) -> Self {
        let mut stack = Vec::with_capacity(81);  // could do better
        if let Ok(grid) = problem.try_into() {
            stack.push(grid);
        }
        Self { stack }
    }

    fn maximize_and_push(&mut self, mut grid: SolutionGrid) {
        if grid.maximize_constraints().is_ok() {
            self.stack.push(grid);
        }
    }
}

impl Iterator for SolutionIterator {
    type Item = SolvedGrid;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut grid) = self.stack.pop() {
            if grid.is_solved() {
                return Some(SolvedGrid::try_from(grid).unwrap())
            } else if let Some((row, col, sc)) = grid.find_least_sols_fuzzy() {
                if let Some(val) = sc.smallest_solution() {
                    let mut new_grid = grid.clone();
                    if grid.forbid(row, col, val).is_ok() {
                        self.maximize_and_push(grid);
                    }
                    if new_grid.constrain(row, col, val).is_ok() {
                        self.maximize_and_push(new_grid);
                    }
                }
            }
        }
        None
    }
}

type SolvedGrid = [[u8; 9]; 9];

impl TryFrom<SolutionGrid> for SolvedGrid {
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

impl FusedIterator for SolutionIterator {}
