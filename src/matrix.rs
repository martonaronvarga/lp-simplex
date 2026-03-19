use crate::rational::Rational;
use num_traits::{zero, Zero};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Rational>,
}

impl Matrix {
    #[inline]
    pub fn index(&self, r: usize, c: usize) -> &Rational {
        &self.data[r * self.cols + c]
    }

    #[inline]
    pub fn index_mut(&mut self, r: usize, c: usize) -> &mut Rational {
        &mut self.data[r * self.cols + c]
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![zero(); rows * cols],
        }
    }

    pub fn row(&self, r: usize) -> &[Rational] {
        let start = r * self.cols;
        &self.data[start..start + self.cols]
    }

    pub fn row_mut(&mut self, r: usize) -> &mut [Rational] {
        let start = r * self.cols;
        &mut self.data[start..start + self.cols]
    }
    pub fn scale_row(&mut self, r: usize, scalar: Rational) {
        assert!(!scalar.is_zero(), "Attempted to scale row {} by zero", r);
        for v in self.row_mut(r) {
            *v *= scalar.clone();
        }
    }

    pub fn add_scaled_row(&mut self, target: usize, source: usize, factor: &Rational) {
        if factor.is_zero() {
            return;
        }
        let cols = self.cols;
        let tgt_base = target * cols;

        for j in 0..cols {
            let scaled_val = self.index(source, j).clone() * factor;
            self.data[tgt_base + j] -= scaled_val;
        }
    }

    pub fn add_column_zero(&mut self) {
        let old_cols = self.cols;
        let old_rows = self.rows;
        self.cols += 1;

        // Grow the flat data vector with zeros for each row
        self.data.resize(self.rows * self.cols, Rational::zero());

        // Shift elements row by row to make room for the new column
        // We iterate rows in reverse to avoid overwriting data
        for r in (0..old_rows).rev() {
            let old_row_start = r * old_cols;
            let new_row_start = r * self.cols;
            for c in (0..old_cols).rev() {
                self.data[new_row_start + c] = self.data[old_row_start + c].clone();
            }
            self.data[new_row_start + old_cols] = Rational::zero(); // new column
        }
    }

    /// Removes the column at the given index
    pub fn remove_column(&mut self, col: usize) {
        assert!(col < self.cols, "Column index out of bounds");
        let old_cols = self.cols;
        let old_rows = self.rows;
        self.cols -= 1;

        for r in (0..old_rows).rev() {
            let old_row_start = r * (old_cols);
            let new_row_start = r * self.cols;

            for c in 0..col {
                self.data[new_row_start + c] = self.data[old_row_start + c].clone();
            }

            for c in col + 1..old_cols {
                self.data[new_row_start + c - 1] = self.data[old_row_start + c].clone();
            }
        }

        // Truncate the excess elements at the end
        self.data.truncate(self.rows * self.cols);
    }
}
