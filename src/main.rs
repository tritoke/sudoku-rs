#![feature(test)]
extern crate test;

#[macro_use]
extern crate failure;

mod grid;
use grid::SudokuGrid;

mod bitmanip;
use bitmanip::BitManip;

#[allow(unused_imports)]
use std::convert::TryInto;
use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut sudoku: SudokuGrid = if let Some(path) = env::args().nth(1) {
        fs::read_to_string(path)?.parse()
    } else {
        include_str!("inputs/extreme.csv").parse()
    }
    .unwrap();

    println!("{}", sudoku);

    let result = solve(&mut sudoku);
    println!("{:?}", result);
    if result.is_solved() {
        println!("{}", sudoku);
    }

    Ok(())
}

/// wrapper to set up the recursive function
fn solve(sudoku: &mut SudokuGrid) -> SolveState {
    // find first empty tile
    if let Some(&first_zero) = sudoku.iter().find(|tile| **tile == 0) {
        solve_rec(sudoku, first_zero as usize)
    } else {
        SolveState::Solved
    }
}

/// does the recursive backtracking to solve the sudoku
fn solve_rec(sudoku: &mut SudokuGrid, tileno: usize) -> SolveState {
    let tries = possible(sudoku, tileno);

    let next_tile = sudoku
        .iter()
        .skip(tileno + 1)
        .position(|tile| *tile == 0)
        .map(|pos| pos + tileno + 1);

    for num in (1..=*sudoku.row_width() as u32).filter(|digit| tries.test_bit(*digit)) {
        sudoku[tileno] = num;
        if let Some(nextno) = next_tile {
            let state = solve_rec(sudoku, nextno as usize);
            if state.is_solved() {
                return SolveState::Solved;
            }
        } else {
            return SolveState::Solved;
        }
    }

    sudoku[tileno] = 0;

    SolveState::UnSolved
}

// returns a which digits are possible at this tile number
// each are bitpacked into a u32
// bit 1 being set means 1 is possible
fn possible(sudoku: &SudokuGrid, tileno: usize) -> u32 {
    let row = tileno / sudoku.row_width();
    let col = tileno % sudoku.row_width();

    let mut bad: u32 = 0;

    for n in sudoku.iter_row(row) {
        bad.set_bit(*n);
    }

    for n in sudoku.iter_col(col) {
        bad.set_bit(*n);
    }

    for n in sudoku.iter_cell(row, col) {
        bad.set_bit(*n);
    }

    return !bad;
}

#[derive(Debug)]
enum SolveState {
    Solved,
    UnSolved,
}

impl SolveState {
    fn is_solved(&self) -> bool {
        matches!(self, SolveState::Solved)
    }
}

#[bench]
fn bench_solve(b: &mut test::Bencher) {
    let grid: Vec<u32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 9,
    ];

    let sudoku: SudokuGrid = grid.try_into().unwrap();

    b.iter(|| {
        let mut cloned = sudoku.clone();
        solve(&mut cloned)
    })
}
