use std::iter::FusedIterator;
use crate::solgrid::SolutionGrid;
use crate::{Puzzle, Grid};

pub struct SolutionIterator {
    stack: Vec<SolutionGrid>,
}

impl SolutionIterator {
    pub fn with_constraints(problem: &Puzzle) -> Self {
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
    type Item = Grid<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut grid) = self.stack.pop() {
            if grid.is_solved() {
                return Some(Self::Item::try_from(grid).unwrap())
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
