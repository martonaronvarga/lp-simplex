use crate::rational::Rational;
use num_traits::{zero, One, Zero};

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

    pub fn inverse(&self) -> Self {
        assert_eq!(self.rows, self.cols, "Matrix must be square to invert");
        let n = self.rows;
        let mut m = Matrix::new(n, n * 2);

        // build [A | I]
        for i in 0..n {
            for j in 0..n {
                *m.index_mut(i, j) = self.index(i, j).clone();
            }
            for j in n..(2 * n) {
                *m.index_mut(i, j) = if j - n == i {
                    Rational::one()
                } else {
                    Rational::zero()
                };
            }
        }

        // Gauss-Jordan eliminate left half to identity
        for i in 0..n {
            let mut pivot_row = i;
            while pivot_row < n && m.index(pivot_row, i).is_zero() {
                pivot_row += 1;
            }
            assert!(pivot_row < n, "Matrix is singular!");

            if pivot_row != i {
                for j in 0..2 * n {
                    let tmp = m.index(i, j).clone();
                    *m.index_mut(i, j) = m.index(pivot_row, j).clone();
                    *m.index_mut(pivot_row, j) = tmp;
                }
            }

            let diag = m.index(i, i).clone();
            assert!(!diag.is_zero(), "Singular matrix at row {}", i);
            for j in 0..2 * n {
                *m.index_mut(i, j) /= diag.clone();
            }

            for k in 0..n {
                if k == i {
                    continue;
                }
                let factor = m.index(k, i).clone();
                if !factor.is_zero() {
                    for j in 0..2 * n {
                        let val = m.index(i, j).clone();
                        *m.index_mut(k, j) -= factor.clone() * val;
                    }
                }
            }
        }

        let mut inv = Matrix::new(n, n);
        for i in 0..n {
            for j in 0..n {
                *inv.index_mut(i, j) = m.index(i, j + n).clone();
            }
        }
        inv
    }
    // pub fn row(&self, r: usize) -> &[Rational] {
    //     let start = r * self.cols;
    //     &self.data[start..start + self.cols]
    // }

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

        self.data.resize(self.rows * self.cols, Rational::zero());

        for r in (0..old_rows).rev() {
            let old_row_start = r * old_cols;
            let new_row_start = r * self.cols;
            for c in (0..old_cols).rev() {
                self.data[new_row_start + c] = self.data[old_row_start + c].clone();
            }
            self.data[new_row_start + old_cols] = Rational::zero(); // new column
        }
    }

    pub fn remove_columns(&mut self, remove: &[usize]) {
        // remove: must be sorted and unique
        let keep: Vec<usize> = (0..self.cols).filter(|i| !remove.contains(i)).collect();
        let mut new_data = Vec::with_capacity(self.rows * keep.len());
        for r in 0..self.rows {
            for &c in &keep {
                new_data.push(self.data[r * self.cols + c].clone());
            }
        }
        self.cols = keep.len();
        self.data = new_data;
    }

    pub fn remove_row(&mut self, r: usize) {
        assert!(r < self.rows, "Row index out of bounds");
        let start = r * self.cols;
        self.data.drain(start..start + self.cols);
        self.rows -= 1;
    }
}
