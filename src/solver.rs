use crate::solgrid::SolutionGrid;
use crate::{Grid, Puzzle};
use std::collections::BTreeSet;
use std::iter::FusedIterator;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Iterate through the solution of a given [`Puzzle`].  Instances are usually
/// obtained via [`Puzzle::solutions`].
#[derive(Clone)]
pub struct SolutionIterator {
    stack: Vec<SolutionGrid>,
    keep_going: Arc<AtomicBool>,
}

impl SolutionIterator {
    /// Create an iterator over the solutions of the given [`Puzzle`].
    pub fn new(problem: &Puzzle) -> Self {
        let mut stack = Vec::with_capacity(81); // could do better
        if let Ok(grid) = problem.try_into() {
            stack.push(grid);
        }
        Self {
            stack,
            keep_going: Arc::new(true.into()),
        }
    }

    /// Set the `Arc<AtomicBool>` used as a "keep going" flag.  This is useful
    /// when running the iterator in its own thread as a way to stop that thread.
    ///
    /// # Example
    ///
    /// ```
    /// use std::thread;
    /// use std::time::Duration;
    /// use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
    /// use sudoku_solver::{Puzzle, SolutionIterator};
    ///
    /// let mut keep_going = Arc::new(AtomicBool::new(true));
    ///
    /// let iterator = SolutionIterator::new(&Puzzle::default())
    ///     .with_handle(keep_going.clone());;
    /// let join_handle = thread::spawn(move || {
    ///     // counting every sudoku grid would take a long time...
    ///     iterator.count()
    /// });
    ///
    /// // This signals the iterator that it should stop now.
    /// keep_going.store(false, Ordering::Release);
    ///
    /// // Giving the other thread some time to read the flag.
    /// thread::sleep(Duration::from_millis(1));
    /// assert!(join_handle.is_finished());
    /// ```
    pub fn with_handle(self, keep_going: Arc<AtomicBool>) -> Self {
        Self { keep_going, ..self }
    }

    fn maximize_and_push(&mut self, mut grid: SolutionGrid) {
        if grid.maximize_constraints().is_ok() {
            self.stack.push(grid);
        }
    }
}

impl Iterator for SolutionIterator {
    type Item = Grid<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut grid) = self.stack.pop() {
            if !self.keep_going.load(Ordering::Relaxed) {
                self.stack.truncate(0);
                continue;
            }
            if grid.is_solved() {
                return Some(Self::Item::try_from(grid).unwrap());
            } else if let Some((row, col, sc)) = grid.find_least_sols_fuzzy() {
                if let Some(val) = sc.smallest_solution() {
                    let mut new_grid = grid.clone();
                    if grid.forbid(row, col, val).is_ok() {
                        self.maximize_and_push(grid);
                    }
                    if new_grid.pin(row, col, val).is_ok() {
                        self.maximize_and_push(new_grid);
                    }
                }
            }
        }
        None
    }
}

impl FusedIterator for SolutionIterator {}

/// A helper to find all the values in each cell that lead to a
/// solvable [`Puzzle`].
pub struct PossibleValuesFinder {
    keep_going: Arc<AtomicBool>,
}

impl PossibleValuesFinder {
    pub fn new() -> Self {
        Self {
            keep_going: Arc::new(true.into()),
        }
    }

    /// Set the `Arc<AtomicBool>` to use as a "keep going" flag. Similarly to
    /// [`SolutionIterator::with_handle`], this is useful to have a way to stop
    /// a separate thread running [`Self::search`].
    pub fn with_handle(keep_going: Arc<AtomicBool>) -> Self {
        Self { keep_going }
    }

    /// Search all the values in the non-pinned cells of a given [`Puzzle`]
    /// that lead to a solvable [`Puzzle`].
    pub fn search(self, puzzle: &Puzzle) -> Grid<BTreeSet<u8>> {
        let mut pvs = Grid::default();
        let solgrid: SolutionGrid = match puzzle.try_into() {
            Ok(sg) => sg,
            Err(_) => return pvs,
        };
        for row in 0..9 {
            for col in 0..9 {
                for val in solgrid.possible_values(row, col) {
                    if !self.keep_going.load(Ordering::Relaxed) {
                        return Grid::default();
                    }
                    if pvs[row][col].contains(&val.value()) {
                        continue;
                    }
                    let mut pzl: Puzzle = puzzle.clone();
                    pzl.pin(row, col, val);
                    let mut sols = pzl.solutions().with_handle(self.keep_going.clone());
                    if let Some(sol) = sols.next() {
                        sol.iter().enumerate().for_each(|(ir, r)| {
                            r.iter().enumerate().for_each(|(ic, &v)| {
                                pvs[ir][ic].insert(v);
                            });
                        });
                    }
                }
            }
        }
        pvs
    }
}
