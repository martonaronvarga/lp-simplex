use num_traits::{One, Signed, Zero};

use crate::matrix::Matrix;
use crate::rational::Rational;
use crate::simplex::LinearProgram;
use crate::variable::{VarKind, Variable};

#[derive(Debug)]
pub struct Tableau {
    pub matrix: Matrix,
    pub m: usize,
    pub n: usize,
    pub basis: Vec<Option<usize>>,
    pub variables: Vec<Variable>,
}

fn make_var(kind: VarKind) -> Variable {
    let name = match &kind {
        VarKind::Original(j) => format!("x{}", j + 1),
        VarKind::Artificial(i) => format!("a{}", i + 1),
    };
    Variable { kind, name }
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
            *matrix.index_mut(i, n) = lp.b[i].clone();
        }

        for j in 0..n {
            *matrix.index_mut(m, j) = lp.c[j].clone();
        }

        let variables = (0..n).map(|j| make_var(VarKind::Original(j))).collect();

        debug_assert_eq!(matrix.cols, n + 1);
        debug_assert_eq!(matrix.rows, m + 1);

        Self {
            matrix,
            basis: vec![None; m],
            m,
            n,
            variables,
        }
    }

    pub fn add_artificials(&mut self) -> Vec<usize> {
        let mut added = vec![];
        let mut saved_rhs = Vec::with_capacity(self.m);
        for i in 0..self.m {
            saved_rhs.push(self.matrix.index(i, self.matrix.cols - 1).clone());
        }
        for i in 0..self.m {
            self.matrix.add_column_zero();
            let col = self.variables.len();
            for r in 0..=self.m {
                *self.matrix.index_mut(r, col) = Rational::zero();
            }
            *self.matrix.index_mut(i, col) = Rational::one();
            self.variables.push(make_var(VarKind::Artificial(i)));
            added.push(col);
        }
        let rhs_idx = self.matrix.cols - 1;
        (0..self.m).for_each(|i| {
            *self.matrix.index_mut(i, rhs_idx) = saved_rhs[i].clone();
        });
        added
    }

    pub fn strip_artificials(&mut self) {
        // Find indices of all artificial variables
        let mut indices: Vec<_> = self
            .variables
            .iter()
            .enumerate()
            .filter(|(_, v): &(usize, &Variable)| v.is_artificial())
            .map(|(i, _)| i)
            .collect();

        indices.sort();

        let mut new_vars = Vec::with_capacity(self.variables.len() - indices.len());
        let mut old_to_new = vec![None; self.variables.len()];
        let mut new_idx = 0;
        for (i, v) in self.variables.iter().enumerate() {
            if !v.is_artificial() {
                new_vars.push(v.clone());
                old_to_new[i] = Some(new_idx);
                new_idx += 1;
            }
        }

        self.matrix.remove_columns(&indices);

        for b in &mut self.basis {
            if let Some(j) = b {
                if let Some(new_j) = old_to_new[*j] {
                    *j = new_j;
                } else {
                    // Should never happen, if phase1 guarantees no artificials left
                    *b = None;
                }
            }
        }
        self.variables = new_vars;
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
    pub fn pivot(&mut self, row: usize, col: usize) {
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

        self.basis[row] = Some(col);
    }

    pub fn entering_variable(&self) -> Option<usize> {
        let width = self.matrix.cols - 1;
        (0..width)
            .filter(|&j| !self.basis.contains(&Some(j)) && self.reduced_cost(j).is_negative())
            .min()
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

    pub fn pretty_print(&self) {
        use std::fmt::Write;
        let mut line = String::new();
        let width = self.matrix.cols;
        write!(line, "\n   |").unwrap();
        for (i, var) in self.variables.iter().enumerate() {
            write!(line, "{:>6}", var.name).unwrap();
            if i == width - 2 {
                break;
            }
        }
        write!(line, "{:>6}", "RHS").unwrap();
        println!("{line}");
        println!(
            "---+{}",
            (0..width).map(|_| "------").collect::<Vec<_>>().join("+")
        );
        for i in 0..self.matrix.rows {
            if i < self.m {
                let var = self.basis[i]
                    .and_then(|j| self.variables.get(j))
                    .map(|v| v.name.as_str())
                    .unwrap_or("-");
                print!("{:>3}|", var);
            } else {
                print!("Obj|");
            }
            for j in 0..width {
                print!("{:>6} |", self.matrix.index(i, j));
            }
            println!();
        }
        println!("=======================\n");
    }
}
