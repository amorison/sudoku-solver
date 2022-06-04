use std::{
    collections::BTreeSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use sudoku_solver::{Grid, PossibleValuesFinder, Puzzle, SolutionIterator};

use crate::{
    counter::{count_saturated, CounterUpTo},
    threaded::Threaded,
};

/// Manage threads running various solvers to find diagnostics about a given
/// puzzle.  Note that the threads are stopped when the [`DetachedSolver`] is
/// dropped.
pub struct DetachedSolver {
    keep_going: Arc<AtomicBool>,
    solution: Threaded<Option<Grid<u8>>>,
    possible_values: Threaded<Grid<BTreeSet<u8>>>,
    n_solutions: Threaded<CounterUpTo>,
}

impl DetachedSolver {
    /// Create a new solver for the given puzzle and a maximum number of
    /// solutions to count.
    pub fn new(puzzle: Puzzle, max_count: usize) -> Self {
        let keep_going = Arc::new(AtomicBool::new(true));

        let mut solit_1 = SolutionIterator::new(&puzzle).with_handle(keep_going.clone());
        let finder = PossibleValuesFinder::with_handle(keep_going.clone());
        let mut solit_2 = solit_1.clone();

        Self {
            keep_going,
            solution: Threaded::spawn(move || solit_1.next()),
            possible_values: Threaded::spawn(move || finder.search(&puzzle)),
            n_solutions: Threaded::spawn(move || count_saturated(&mut solit_2, max_count)),
        }
    }

    /// Check whether a solution has been found by the solver.
    pub fn poll_solution(&mut self) -> Option<&Option<Grid<u8>>> {
        self.solution.try_join()
    }

    /// Check whether the set of possible values has been found by the solver.
    pub fn poll_possible_values(&mut self) -> Option<&Grid<BTreeSet<u8>>> {
        self.possible_values.try_join()
    }

    /// Check whether the number of solutions has been found by the solver.
    pub fn poll_n_solutions(&mut self) -> Option<&CounterUpTo> {
        self.n_solutions.try_join()
    }
}

impl Drop for DetachedSolver {
    fn drop(&mut self) {
        self.keep_going.store(false, Ordering::Relaxed);
    }
}
