use crate::problem::{Puzzle, Value};
use crate::soft::SoftConstraint;

#[test]
fn no_sol_direct_constraint_conflict() {
    let mut p = Puzzle::new();
    p.pin(0, 0, Value::new(1));
    p.pin(0, 1, Value::new(1));
    assert!(p.solutions().next().is_none());
}

#[test]
fn no_sol_cell_with_no_possibility() {
    let mut p = Puzzle::new();
    for i in 0..6 {
        p.pin(0, i, Value::new((i + 1) as u8));
    }
    for i in 6..9 {
        p.pin(1, i, Value::new((i + 1) as u8));
    }
    assert!(p.solutions().next().is_none());
}

#[test]
fn soft_constraint() {
    let mut sc = SoftConstraint::default();
    assert_eq!(sc.num_solutions(), 9);
    assert!(sc.unique_solution().is_none());
    assert_eq!(sc.smallest_solution().unwrap().value(), 1);
    assert_eq!(sc.smallest_solution().unwrap().value(), 1);
    assert!(sc.has_solutions());
}

#[test]
fn empty_problem_sol() {
    let p = Puzzle::new();
    let expected = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [2, 3, 1, 6, 7, 4, 8, 9, 5],
        [8, 7, 5, 9, 1, 2, 3, 6, 4],
        [6, 9, 4, 5, 3, 8, 2, 1, 7],
        [3, 1, 7, 2, 6, 5, 9, 4, 8],
        [5, 4, 2, 8, 9, 7, 6, 3, 1],
        [9, 6, 8, 3, 4, 1, 5, 7, 2]
    ];
    assert_eq!(p.solutions().next().unwrap(), expected);
}

#[test]
fn empty_problem_1000_sols() {
    let p = Puzzle::new();
    assert_eq!(p.solutions().take(1000).count(), 1000);
}

#[test]
fn test_problem_single_sol() {
    let p = Puzzle::from_arr([
        [0, 5, 0, 0, 0, 0, 0, 0, 6],
        [0, 0, 6, 7, 3, 0, 0, 2, 0],
        [0, 0, 0, 0, 8, 0, 0, 0, 0],
        [0, 0, 5, 2, 6, 0, 0, 3, 0],
        [4, 0, 0, 0, 0, 0, 9, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 9, 0, 1, 7, 0, 3, 0, 0],
        [0, 0, 7, 0, 0, 8, 0, 0, 0],
        [0, 0, 0, 0, 0, 5, 0, 1, 0]
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
        [2, 3, 4, 6, 9, 5, 7, 1, 8]
    ];
    let mut sols = p.solutions();
    assert!(sols.next().unwrap() == expected);
    assert!(sols.next().is_none());
}

#[test]
fn test_problem_double_sol() {
    let p = Puzzle::from_arr([
        [0, 0, 3, 4, 5, 6, 7, 8, 9],  // row can start with 1,2 or 2,1
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [0, 0, 4, 8, 3, 5, 9, 6, 7],  // this one will start with 2,1 or 1,2
        [8, 3, 5, 6, 9, 7, 2, 1, 4],
        [6, 9, 7, 2, 1, 4, 5, 3, 8],
        [3, 4, 2, 5, 7, 8, 6, 9, 1],
        [5, 6, 8, 9, 4, 1, 3, 7, 2],
        [9, 7, 1, 3, 6, 2, 8, 4, 5]
    ]);
    let expected1 = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [2, 1, 4, 8, 3, 5, 9, 6, 7],
        [8, 3, 5, 6, 9, 7, 2, 1, 4],
        [6, 9, 7, 2, 1, 4, 5, 3, 8],
        [3, 4, 2, 5, 7, 8, 6, 9, 1],
        [5, 6, 8, 9, 4, 1, 3, 7, 2],
        [9, 7, 1, 3, 6, 2, 8, 4, 5]
    ];
    let expected2 = [
        [2, 1, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [1, 2, 4, 8, 3, 5, 9, 6, 7],
        [8, 3, 5, 6, 9, 7, 2, 1, 4],
        [6, 9, 7, 2, 1, 4, 5, 3, 8],
        [3, 4, 2, 5, 7, 8, 6, 9, 1],
        [5, 6, 8, 9, 4, 1, 3, 7, 2],
        [9, 7, 1, 3, 6, 2, 8, 4, 5]
    ];
    let mut sols = p.solutions();
    assert!(sols.next().unwrap() == expected1);
    assert!(sols.next().unwrap() == expected2);
    assert!(sols.next().is_none());
}

#[test]
fn test_problem_triple_sol() {
    let p = Puzzle::from_arr([
        [3, 0, 9, 6, 0, 0, 4, 0, 0],
        [0, 0, 0, 7, 0, 9, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 0, 0],
        [7, 5, 0, 0, 6, 0, 2, 3, 0],
        [6, 0, 0, 9, 0, 4, 0, 0, 8],
        [0, 2, 8, 0, 5, 0, 0, 4, 1],
        [0, 0, 0, 0, 0, 0, 5, 9, 0],
        [0, 0, 0, 1, 9, 6, 0, 0, 7],
        [0, 0, 6, 0, 0, 0, 1, 0, 4]
    ]);
    assert_eq!(p.solutions().count(), 3);
}
