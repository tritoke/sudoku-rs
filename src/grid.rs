use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::ops;
use std::slice;
use std::str::FromStr;

/// Structure representing a sudoku grid
/// empty tiles are represented by 0
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SudokuGrid {
    tiles: Vec<u32>,
    cell_width: usize,
    row_width: usize,
}

#[allow(dead_code)]
impl SudokuGrid {
    /// get the cell width member from the struct
    #[inline]
    pub fn cell_width(&self) -> &usize {
        &self.cell_width
    }

    /// get the cell width member from the struct
    #[inline]
    pub fn row_width(&self) -> &usize {
        &self.row_width
    }

    /// get a reference to the tiles
    #[inline]
    pub fn tiles(&self) -> &[u32] {
        &self.tiles
    }

    /// Returns an iterator over all the tiles on the board
    #[inline]
    pub fn iter(&self) -> slice::Iter<u32> {
        self.tiles.iter()
    }

    /// Returns an interator over row `row`
    pub fn iter_row(&self, row: usize) -> slice::Iter<u32> {
        if row < self.row_width {
            let start = row * self.row_width;
            self.tiles[start..(start + self.row_width)].iter()
        } else {
            panic!(
                "Out of bounds. Row must be less than {:?}, but is {:?}.",
                self.row_width, row
            )
        }
    }

    /// Returns an interator over column `col`
    pub fn iter_col(&self, col: usize) -> iter::StepBy<slice::Iter<u32>> {
        if col < self.row_width {
            self.tiles[col..].iter().step_by(self.row_width)
        } else {
            panic!(
                "Out of bounds. Col must be less than {:?}, but is {:?}.",
                self.row_width, col
            )
        }
    }

    /// Returns an interator over the cell which `col`, `row` is in
    pub fn iter_cell(&self, row: usize, col: usize) -> CellIter {
        if col >= self.row_width {
            panic!(
                "Out of bounds. Col must be less than {:?}, but is {:?}.",
                self.row_width, col
            )
        } else if row >= self.row_width {
            panic!(
                "Out of bounds. Row must be less than {:?}, but is {:?}.",
                self.row_width, col
            )
        }

        // calculate the number of chunsk to skip
        let chunks_to_skip = (row / self.cell_width) * self.row_width + col / self.cell_width;

        CellIter {
            inner: self
                .tiles
                .chunks(self.cell_width)
                .skip(chunks_to_skip)
                .step_by(self.cell_width)
                .take(self.cell_width)
                .flatten(),
        }
    }
}

pub struct CellIter<'a> {
    inner: iter::Flatten<iter::Take<iter::StepBy<iter::Skip<slice::Chunks<'a, u32>>>>>,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = &'a u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bottom_cell_border_centre = vec!["═"; self.cell_width].join("═╧═");
        let top_cell_border_centre = vec!["═"; self.cell_width].join("═╤═");

        let bottom_border_centre = iter::repeat(bottom_cell_border_centre)
            .take(self.cell_width)
            .collect::<Vec<_>>()
            .join("═╩═");

        let top_border_centre = iter::repeat(top_cell_border_centre)
            .take(self.cell_width)
            .collect::<Vec<_>>()
            .join("═╦═");

        let top_border = vec!["╔═", &top_border_centre, "═╗"].join("");
        let bottom_border = vec!["╚═", &bottom_border_centre, "═╝"].join("");

        let row_sep = vec![
            "╟─",
            &iter::repeat(vec!["─"; self.cell_width].join("─┼─"))
                .take(self.cell_width)
                .collect::<Vec<_>>()
                .join("─╫─"),
            "─╢",
        ]
        .join("");

        let cell_row_sep = vec![
            "╠═",
            &iter::repeat(vec!["═"; self.cell_width].join("═╪═"))
                .take(self.cell_width)
                .collect::<Vec<_>>()
                .join("═╬═"),
            "═╣",
        ]
        .join("");

        writeln!(f, "{}", top_border)?;
        for y in 0..self.row_width {
            write!(f, "║")?;
            for x in 0..self.row_width {
                let cell = self[(y, x)];
                if cell == 0 {
                    write!(f, "   ")?;
                } else {
                    write!(f, "{: ^3}", cell)?;
                }

                if x % self.cell_width == self.cell_width - 1 {
                    write!(f, "║")?;
                } else {
                    write!(f, "│")?;
                }
            }
            writeln!(f, "")?;

            // write row seperator
            if y != self.row_width - 1 {
                if y % self.cell_width == self.cell_width - 1 {
                    writeln!(f, "{}", cell_row_sep)?;
                } else {
                    writeln!(f, "{}", row_sep)?;
                }
            }
        }
        write!(f, "{}", bottom_border)
    }
}

/// Custom error type to represent the ways that parsing a sudoku from a CSV can fail
#[derive(Debug, Fail)]
pub enum SudokuParseError {
    #[fail(display = "board not square")]
    NonSquare,
    #[fail(display = "digit not in range for board")]
    DigitOutOfRange,
    #[fail(display = "invalid digit in board")]
    InvalidDigit(#[fail(cause)] std::num::ParseIntError),
}

impl FromStr for SudokuGrid {
    type Err = SudokuParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse the tiles
        // propagating any internal errors into the outer result type
        let tiles_fromstr: Result<Vec<u32>, SudokuParseError> = s
            .lines()
            .flat_map(|line| {
                line.split(',').map(|n| {
                    if n.is_empty() {
                        Ok(0)
                    } else {
                        n.parse().map_err(SudokuParseError::InvalidDigit)
                    }
                })
            })
            .collect();

        // check if any of the parses failed with ParseIntError
        let tiles = tiles_fromstr?;

        // calculate the cell width
        let cell_width = (tiles.len() as f64).powf(0.25) as usize;
        let row_width = cell_width.pow(2);

        // validate the parsed data
        if cell_width.pow(4) != tiles.len() {
            Err(SudokuParseError::NonSquare)
        } else if !tiles.iter().all(|&n| n <= row_width as u32) {
            Err(SudokuParseError::DigitOutOfRange)
        } else {
            Ok(Self {
                tiles,
                cell_width,
                row_width,
            })
        }
    }
}

impl ops::Index<(usize, usize)> for SudokuGrid {
    type Output = u32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        // assert index in bounds
        if col >= self.row_width || row >= self.row_width {
            panic!(
                "index {:?} out of range for board with width {}",
                (row, col),
                self.row_width
            );
        }

        unsafe { self.tiles.get_unchecked(row * self.row_width + col) }
    }
}

impl ops::IndexMut<(usize, usize)> for SudokuGrid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        // assert index in bounds
        if col >= self.row_width || row >= self.row_width {
            panic!(
                "index {:?} out of range for board with width {}",
                (col, row),
                self.row_width
            );
        }

        unsafe { self.tiles.get_unchecked_mut(row * self.row_width + col) }
    }
}

impl ops::Index<usize> for SudokuGrid {
    type Output = u32;

    #[inline]
    fn index(&self, tileno: usize) -> &Self::Output {
        &self.tiles[tileno]
    }
}

impl ops::IndexMut<usize> for SudokuGrid {
    #[inline]
    fn index_mut(&mut self, tileno: usize) -> &mut Self::Output {
        &mut self.tiles[tileno]
    }
}

impl TryFrom<Vec<u32>> for SudokuGrid {
    type Error = SudokuParseError;

    fn try_from(tiles: Vec<u32>) -> Result<Self, Self::Error> {
        // calculate the cell width
        let cell_width = (tiles.len() as f64).powf(0.25) as usize;
        let row_width = cell_width.pow(2);

        if cell_width.pow(4) != tiles.len() {
            Err(SudokuParseError::NonSquare)
        } else if !tiles.iter().all(|&n| n <= row_width as u32) {
            Err(SudokuParseError::DigitOutOfRange)
        } else {
            Ok(Self {
                tiles,
                cell_width,
                row_width,
            })
        }
    }
}
