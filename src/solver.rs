use crate::solgrid::SolutionGrid;
use crate::{Grid, Puzzle};
use std::collections::BTreeSet;
use std::iter::FusedIterator;

pub struct SolutionIterator {
    stack: Vec<SolutionGrid>,
}

impl SolutionIterator {
    pub fn new(problem: &Puzzle) -> Self {
        let mut stack = Vec::with_capacity(81); // could do better
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

pub fn possible_values(puzzle: &Puzzle) -> Grid<BTreeSet<u8>> {
    let mut pvs = Grid::default();
    let solgrid: SolutionGrid = match puzzle.try_into() {
        Ok(sg) => sg,
        Err(_) => return pvs,
    };
    for row in 0..9 {
        for col in 0..9 {
            for val in solgrid.possible_values(row, col) {
                if pvs[row][col].contains(&val.value()) {
                    continue;
                }
                let mut pzl: Puzzle = puzzle.clone();
                pzl.pin(row, col, val);
                if let Some(sol) = pzl.solutions().next() {
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
