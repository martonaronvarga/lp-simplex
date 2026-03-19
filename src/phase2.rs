use crate::tableau::Tableau;
use crate::{rational::Rational, solution::Solution};
use anyhow::Result;
use num_traits::{one, zero};

// pub fn phase2(tableau: &mut Tableau) -> Result<Solution> {
//     while let Some(j) = tableau.entering_variable() {
//         let entering = j;

//         let leaving = tableau.leaving_row(entering);

//         let row = match leaving {
//             Some(r) => r,

//             None => {
//                 return Ok(unbounded_direction(tableau, entering));
//             }
//         };

//         tableau.pivot(row, entering);
//     }

//     Ok(optimal_solution(tableau))
// }

pub fn phase2(tableau: &mut Tableau) -> Result<Solution> {
    while let Some(entering) = tableau.entering_variable() {
        match tableau.leaving_row(entering) {
            Some(row) => tableau.pivot(row, entering),
            None => {
                let sol = unbounded_direction(tableau, entering);
                strip_artificials(tableau);
                return Ok(sol);
            }
        }
    }

    let sol = optimal_solution(tableau);
    strip_artificials(tableau);
    Ok(sol)
}

fn optimal_solution(tableau: &Tableau) -> Solution {
    let art = tableau.n; // == n

    println!(
        "DEBUG optimal_solution: n={} m={} art={} matrix.cols={}",
        tableau.n, tableau.m, art, tableau.matrix.cols
    );
    println!("DEBUG obj row at art cols:");
    for k in 0..tableau.m {
        let rc = tableau.reduced_cost(art + k);
        println!(
            "  obj[{}] = {:?}  => dual[{}] = {:?}",
            art + k,
            rc,
            k,
            -rc.clone()
        );
    }

    let mut primal: Vec<Rational> = vec![zero(); tableau.n];
    for i in 0..tableau.m {
        let var = tableau.basis[i];
        if var < tableau.n {
            primal[var] = tableau.rhs(i).clone();
        }
    }

    let dual: Vec<Rational> = (0..tableau.m)
        .map(|k| tableau.reduced_cost(art + k))
        .collect();

    let objective = -tableau.objective_value();

    Solution::Optimal {
        objective,
        primal,
        dual,
    }
}

fn unbounded_direction(tableau: &Tableau, entering: usize) -> Solution {
    let mut direction = vec![zero(); tableau.n];

    direction[entering] = one();

    for i in 0..tableau.m {
        let var = tableau.basis[i];

        if var < tableau.n {
            let val = tableau.matrix.index(i, entering).clone();
            direction[var] = -val;
        }
    }

    Solution::Unbounded { direction }
}

fn strip_artificials(tableau: &mut Tableau) {
    let art = tableau.n;
    let m = tableau.m;
    for k in (0..m).rev() {
        tableau.matrix.remove_column(art + k);
    }
}
