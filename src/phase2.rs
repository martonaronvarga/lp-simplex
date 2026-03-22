use crate::matrix::Matrix;
use crate::tableau::Tableau;
use crate::variable::VarKind;
use crate::{rational::Rational, solution::Solution};
use num_traits::{One, Zero};

impl Tableau {
    pub fn phase2(&mut self, c: &[Rational]) -> Solution {
        let obj_row = self.m;
        let width = self.matrix.cols;
        for j in 0..width {
    *self.matrix.index_mut(obj_row, j) = Rational::zero();
}
for (j, var) in self.variables.iter().enumerate() {
    if let VarKind::Original(k) = var.kind {
        *self.matrix.index_mut(obj_row, j) = -c[k].clone();
    }
}
for i in 0..self.m {
    if let Some(j) = self.basis[i]
        && let VarKind::Original(k) = self.variables[j].kind {
            let coeff = c[k].clone();
            if !coeff.is_zero() {
                for l in 0..width {
                    let val = self.matrix.index(i, l).clone();
                    *self.matrix.index_mut(obj_row, l) += coeff.clone() * val;
                }
            }
        }
}
        while let Some(entering) = self.entering_variable() {
            match self.leaving_row(entering) {
                Some(row) => self.pivot(row, entering),
                None => return self.unbounded_direction(self.n),
            }
        }
        
        self.optimal_solution(c,self.n)
    }

fn optimal_solution(&self, c: &[Rational], orig_vars: usize) -> Solution {
    use crate::variable::VarKind;

    let mut primal = vec![Rational::zero(); orig_vars];
    for i in 0..self.m {
        if let Some(j) = self.basis[i]
            && let VarKind::Original(k) = self.variables[j].kind {
                primal[k] = self.rhs(i).clone();
            }
    }

    let mut c_b = Vec::with_capacity(self.m);
    let mut ab = Matrix::new(self.m, self.m);
    for i in 0..self.m {
        let j = self.basis[i].expect("Basis slot missing");
        c_b.push(match self.variables[j].kind {
            VarKind::Original(idx) => c[idx].clone(),
            _ => Rational::zero(),
        });
        for row in 0..self.m {
            *ab.index_mut(row, i) = self.matrix.index(row, j).clone();
        }
    }
    let ab_inv = ab.inverse();

    let mut dual = vec![Rational::zero(); self.m];
    for i in 0..self.m {
        for j in 0..self.m {
            dual[i] += c_b[j].clone() * ab_inv.index(j, i).clone();
        }
    }

    let obj = -self.objective_value();
    Solution::Optimal {
        objective: obj,
        primal,
        dual,
    }
}

    fn unbounded_direction(&self, orig_vars: usize) -> Solution {
        let n = orig_vars;
        let mut dir = vec![Rational::zero(); n];
        let enter = self.entering_variable().unwrap();
        if let VarKind::Original(k) = self.variables[enter].kind
            && k < n {
                dir[k] = One::one();
            }
        for i in 0..self.m {
            if let Some(j) = self.basis[i]
                && let VarKind::Original(k) = self.variables[j].kind
                    && k < n {
                        dir[k] = -self.matrix.index(i, enter).clone();
                    }
        }
        Solution::Unbounded { direction: dir }
    }
}
