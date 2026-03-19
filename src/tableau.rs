use num_traits::{One, Signed, Zero};

use crate::matrix::Matrix;
use crate::rational::Rational;
use crate::simplex::LinearProgram;

#[derive(Debug)]
pub struct Tableau {
    pub matrix: Matrix,
    pub basis: Vec<usize>,
    pub m: usize,
    pub n: usize,
}

impl Tableau {
    pub fn from_lp(lp: &LinearProgram) -> Self {
        let m = lp.m;
        let n = lp.n;

        let mut matrix = Matrix::new(m + 1, n + 1);

        for i in 0..m {
            for j in 0..n {
                *matrix.index_mut(i, j) = lp.a.index(i, j).clone();
            }
            *matrix.index_mut(i, n) = lp.b[i].clone(); // RHS
        }

        for j in 0..n {
            *matrix.index_mut(m, j) = lp.c[j].clone(); // Coefficients of the objective function
        }

        debug_assert_eq!(matrix.cols, n + 1);
        debug_assert_eq!(matrix.rows, m + 1);

        Self {
            matrix,
            basis: vec![0; m],
            m,
            n,
        }
    }

    pub fn restore_objective(&mut self, lp: &LinearProgram) {
        let obj_row = self.m;
        for j in 0..self.matrix.cols {
            *self.matrix.index_mut(obj_row, j) = Rational::zero();
        }
        for j in 0..lp.n {
            *self.matrix.index_mut(obj_row, j) = -lp.c[j].clone();
        }
        for i in 0..self.m {
            let basic_var = self.basis[i];
            if basic_var >= lp.n {
                continue; // redundant-row sentinel — skip
            }
            let coeff = lp.c[basic_var].clone();
            if !coeff.is_zero() {
                for j in 0..self.matrix.cols {
                    let val = self.matrix.index(i, j).clone();
                    *self.matrix.index_mut(obj_row, j) += coeff.clone() * val;
                }
            }
        }
    }

    pub fn pivot(&mut self, row: usize, col: usize) {
        println!("DEBUG: Pivoting on row {} and column {}", row, col);
        println!("DEBUG: Basis before pivot: {:?}", self.basis);

        let pivot = self.matrix.index(row, col).clone();
        if pivot.is_zero() {
            panic!("DEBUG: Pivot element is zero! This should not happen.");
        }
        self.matrix.scale_row(row, pivot.recip()); // Scale the pivot row

        for r in 0..=self.m {
            if r == row {
                continue;
            }

            let factor = self.matrix.index(r, col).clone();
            if !factor.is_zero() {
                self.matrix.add_scaled_row(r, row, &factor);
            }
        }

        self.basis[row] = col;
        println!("DEBUG: Basis after pivot: {:?}", self.basis);
        println!("DEBUG: Tableau after pivot:");
        self.pretty_print();
    }

    pub fn entering_variable(&self) -> Option<usize> {
        println!("DEBUG: Identifying entering variable using reduced costs...");
        let entering = (0..self.n)
            .filter(|&j| !self.basis.contains(&j) && self.reduced_cost(j).is_negative())
            .min();

        if let Some(col) = entering {
            println!(
                "DEBUG: Chose entering variable (column): {}. Reduced cost: {:?}",
                col,
                self.reduced_cost(col)
            );
        } else {
            println!("DEBUG: No entering variable found. Optimal solution reached.");
        }

        entering
    }

    pub fn leaving_row(&self, col: usize) -> Option<usize> {
        let mut min_ratio: Option<Rational> = None;
        let mut leaving: Option<usize> = None;

        for i in 0..self.m {
            let a = self.matrix.index(i, col);
            if a.is_positive() {
                let ratio = self.rhs(i).clone() / a;

                match min_ratio {
                    None => {
                        min_ratio = Some(ratio.clone());
                        leaving = Some(i);
                    }
                    Some(ref current_min) => {
                        if ratio < *current_min {
                            min_ratio = Some(ratio.clone());
                            leaving = Some(i);
                        } else if ratio == *current_min
                            && self.basis[i] < self.basis[leaving.unwrap()]
                        {
                            leaving = Some(i);
                        }
                    }
                }
            }
        }

        leaving
    }

    pub fn rhs(&self, row: usize) -> &Rational {
        self.matrix.index(row, self.matrix.cols - 1)
    }

    pub fn objective_value(&self) -> Rational {
        -self.matrix.index(self.m, self.matrix.cols - 1).clone()
    }

    pub fn reduced_cost(&self, j: usize) -> Rational {
        self.matrix.index(self.m, j).clone()
    }

    pub fn pretty_print(&self) {
        println!("\n======= Tableau =======");
        println!(
            "   |{:>6}",
            (0..self.matrix.cols)
                .map(|i| format!("{:>6}", i))
                .collect::<Vec<_>>()
                .join(" |")
        );
        println!(
            "---+{}",
            (0..self.matrix.cols)
                .map(|_| "------")
                .collect::<Vec<_>>()
                .join("+")
        );

        for i in 0..self.matrix.rows {
            if i < self.m {
                print!("{:<3}|", self.basis[i]); // Print the row's basic variable
            } else {
                print!("Obj|"); // Objective row
            }

            for j in 0..self.matrix.cols {
                print!("{:>6} |", self.matrix.index(i, j));
            }
            println!();
        }
        println!("=======================\n");
    }
}
