use crate::{matrix::Matrix, rational::Rational, tableau::Tableau};
use num_traits::{One, Signed, Zero};

pub fn phase1(tableau: &mut Tableau) -> bool {
    let m = tableau.m;
    let orig_cols = tableau.matrix.cols;
    let rhs_col = orig_cols - 1;

    println!("DEBUG: Entering Phase 1.");
    tableau.pretty_print();

    for i in 0..m {
        if tableau.rhs(i).is_negative() {
            for val in tableau.matrix.row_mut(i) {
                *val = -val.clone();
            }
        }
    }

    let artificial_start = rhs_col;
    let new_cols = orig_cols + m;
    let mut new_matrix = Matrix::new(m + 1, new_cols);

    for i in 0..=m {
        for j in 0..rhs_col {
            *new_matrix.index_mut(i, j) = tableau.matrix.index(i, j).clone();
        }
        *new_matrix.index_mut(i, new_cols - 1) = tableau.matrix.index(i, rhs_col).clone();
    }
    for i in 0..m {
        *new_matrix.index_mut(i, artificial_start + i) = Rational::one();
    }
    tableau.matrix = new_matrix;
    tableau.basis = (artificial_start..artificial_start + m).collect();

    println!("DEBUG: Tableau after adding artificial variables:");
    tableau.pretty_print();

    let obj_row = m;
    for j in 0..tableau.matrix.cols {
        *tableau.matrix.index_mut(obj_row, j) = Rational::zero();
    }
    for k in 0..m {
        *tableau.matrix.index_mut(obj_row, artificial_start + k) = -Rational::one();
    }
    for i in 0..m {
        for j in 0..tableau.matrix.cols {
            let val = tableau.matrix.index(i, j).clone();
            *tableau.matrix.index_mut(obj_row, j) += val;
        }
    }

    println!("DEBUG: Constructed auxiliary objective row:");
    tableau.pretty_print();

    let total_vars = tableau.matrix.cols - 1; // exclude RHS
    loop {
        let entering = (0..total_vars)
            .find(|&j| tableau.matrix.index(obj_row, j).is_positive())
            .iter()
            .max()
            .copied();
        match entering {
            None => break, // Phase 1 stop
            Some(col) => {
                if let Some(leaving_row) = tableau.leaving_row(col) {
                    tableau.pivot(leaving_row, col);
                } else {
                    println!("DEBUG: No leaving row found. Problem is infeasible in Phase 1.");
                    return false;
                }
            }
        }
    }

    let aux_obj = tableau.matrix.index(obj_row, tableau.matrix.cols - 1);
    if !aux_obj.is_zero() {
        println!(
            "DEBUG: Infeasible problem detected. Auxiliary objective: {:?}",
            aux_obj
        );
        return false;
    }

    for k in 0..m {
        *tableau.matrix.index_mut(obj_row, artificial_start + k) = Rational::zero();
    }

    for i in 0..m {
        if tableau.basis[i] < artificial_start {
            continue; // already an original variable
        }
        let mut pivoted = false;
        for j in 0..tableau.n {
            if !tableau.matrix.index(i, j).is_zero() {
                tableau.pivot(i, j);
                pivoted = true;
                break;
            }
        }
        if !pivoted {
            tableau.basis[i] = tableau.n;
        }
    }

    println!("DEBUG: Phase 1 complete, tableau cleaned:");
    tableau.pretty_print();

    true
}
