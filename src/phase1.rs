use crate::{rational::Rational, solution::Solution, tableau::Tableau};
use num_traits::{One, Signed, Zero};

impl Tableau {
    pub fn phase1(&mut self) -> Result<(), Solution> {
        for i in 0..self.m {
            if self.rhs(i).is_negative() {
                for v in self.matrix.row_mut(i) {
                    *v = -v.clone();
                }
            }
        }
        let art_cols = self.add_artificials();
        let obj_row = self.m;
        for j in 0..self.matrix.cols {
            *self.matrix.index_mut(obj_row, j) = Rational::zero();
        }
        for &col in &art_cols {
            *self.matrix.index_mut(obj_row, col) = -Rational::one();
        }
        for i in 0..self.m {
            for j in 0..self.matrix.cols {
                let val = self.matrix.index(i, j).clone();
                *self.matrix.index_mut(obj_row, j) += val;
            }
        }

        (0..self.m).for_each(|i| {
            self.basis[i] = Some(art_cols[i]);
        });
        let total_vars = self.matrix.cols - 1;
        loop {
            let enter = (0..total_vars)
                .find(|&j| self.matrix.index(obj_row, j).is_positive())
                .iter()
                .max()
                .copied();

            match enter {
                None => {
                                break;
                },
                Some(col) => {
                    if let Some(leave) = self.leaving_row(col) {

                        self.pivot(leave, col);
                    } else {
                        return Err(Solution::Infeasible);
                    }
                }
            }
        }
        let aux_obj = self.matrix.index(obj_row, self.matrix.cols - 1);
        if *aux_obj != Rational::zero() {
            return Err(Solution::Infeasible);
        }

        let mut to_replace = Vec::new();
        for i in 0..self.basis.len() {
            if let Some(j) = self.basis[i]
                && self.variables[j].is_artificial() {
                to_replace.push(i);
            }
        }
        let mut removed_rows = Vec::new();
        for &i in &to_replace {
            let width = self.matrix.cols - 1;
            let mut found = false;
            for k in 0..width {
                if !self.variables[k].is_artificial()
                   && self.matrix.index(i, k).is_one()
                   && (0..self.basis.len()).all(|ii| ii == i || self.matrix.index(ii, k).is_zero())
                {
                    self.pivot(i, k);
                    found = true;
                    break;
                }
            }
            if !found {
                let is_artificial_col = |j: usize| self.variables[j].is_artificial();
                let row_all_zero = (0..width)
                    .all(|j| is_artificial_col(j) || self.matrix.index(i, j).is_zero());
                let rhs_zero = self.rhs(i).is_zero();
                if row_all_zero && rhs_zero {
                    removed_rows.push(i);
                } else {
                    return Err(Solution::Infeasible);
                }
            }
        }

        let mut columns_to_remove = Vec::new();
        for &i in &removed_rows {
            if let Some(Some(j)) = self.basis.get(i).copied()
                && self.variables[j].is_artificial() {
                    columns_to_remove.push(j);
                }
        }
        removed_rows.sort_by(|a, b| b.cmp(a));
        for &i in &removed_rows {
            self.matrix.remove_row(i);
            self.basis.remove(i);
        }
        columns_to_remove.sort_by(|a, b| b.cmp(a));
        for &j in &columns_to_remove {
            self.matrix.remove_columns(&[j]);
            self.variables.remove(j);
            // Always update all indices in self.basis > j!
            for idx in self.basis.iter_mut().flatten() {
                if *idx > j { *idx -= 1; }
            }
        }

        self.m = self.matrix.rows - 1;
        self.basis.truncate(self.m);



        for (row, &b) in self.basis.iter().enumerate() {
            if let Some(j) = b {
                if self.variables[j].is_artificial() {
                    eprintln!("ERROR: Still have basic artificial {} at row {} after phase 1!", self.variables[j].name, row);
                    return Err(Solution::Infeasible);
                }
            } else {
                eprintln!("ERROR: No basic variable found at row {} after phase 1!", row);
                return Err(Solution::Infeasible);
            }
        }
       
        self.strip_artificials();

        for &b in &self.basis {
            if let Some(j) = b {
                if self.variables[j].is_artificial() {
                    return Err(Solution::Infeasible);
                }
            } else {
                return Err(Solution::Infeasible);
            }
        }

        Ok(())
    }
}
