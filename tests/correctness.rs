use lp_simplex::rational::Rational;
use lp_simplex::simplex::{self, LinearProgram, SimplexSolver};
use lp_simplex::solution::Solution;
use num_traits::{Signed, Zero};
use rand::RngExt;

fn verify_solution(lp: &LinearProgram, sol: &Solution) {
    match sol {
        Solution::Optimal {
            objective,
            primal,
            dual,
        } => {
            for i in 0..lp.m {
                let sum: Rational = (0..lp.n)
                    .map(|j| lp.a.index(i, j) * &primal[j])
                    .fold(Rational::zero(), |acc, x| acc + x);
                assert_eq!(sum, lp.b[i], "Ax != b at row {}", i);
            }

            for (j, xj) in primal.iter().enumerate() {
                assert!(!xj.is_negative(), "x[{}] < 0", j);
            }

            for j in 0..lp.n {
                let lhs: Rational = (0..lp.m)
                    .map(|i| lp.a.index(i, j) * &dual[i])
                    .fold(Rational::zero(), |acc, x| acc + x);
                assert!(lhs >= lp.c[j], "Dual infeasible at column {}", j);
            }

            (0..lp.n).for_each(|j| {
                let lhs: Rational = (0..lp.m)
                    .map(|i| lp.a.index(i, j) * &dual[i])
                    .fold(Rational::zero(), |acc, x| acc + x);
                let reduced = &lp.c[j] - lhs;
                if primal[j].is_zero() {
                    assert!(
                        reduced <= Rational::zero(),
                        "Reduced cost violated at {}",
                        j
                    );
                } else {
                    assert_eq!(
                        reduced,
                        Rational::zero(),
                        "Reduced cost not zero for basic x[{}]",
                        j
                    );
                }
            });

            let primal_obj: Rational = (0..lp.n)
                .map(|j| &lp.c[j] * &primal[j])
                .fold(Rational::zero(), |acc, x| acc + x);
            let dual_obj: Rational = (0..lp.m)
                .map(|i| &dual[i] * &lp.b[i])
                .fold(Rational::zero(), |acc, x| acc + x);
            assert_eq!(primal_obj, dual_obj, "Strong duality failed");
            assert_eq!(*objective, primal_obj, "Objective mismatch");
        }

        Solution::Unbounded { direction } => {
            for i in 0..lp.m {
                let sum: Rational = (0..lp.n)
                    .map(|j| lp.a.index(i, j) * &direction[j])
                    .fold(Rational::zero(), |acc, x| acc + x);
                assert!(
                    sum <= Rational::zero(),
                    "Unbounded direction violates Ax <= b at row {}",
                    i
                );
            }

            let obj_inc: Rational = (0..lp.n)
                .map(|j| &lp.c[j] * &direction[j])
                .fold(Rational::zero(), |acc, x| acc + x);
            assert!(
                obj_inc > Rational::zero(),
                "Unbounded direction does not increase objective"
            );
        }

        Solution::Infeasible => {}
    }
}

fn small_lps() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Optimal",
            "\
2
2
1 1
1 0 4
0 1 3
",
        ),
        (
            "Unbounded",
            "\
1
1
1
0 1
",
        ),
        (
            "Infeasible",
            "\
1
2
1
1 1
-1 -3
",
        ),
    ]
}

fn klee_minty(n: usize) -> String {
    let mut s = format!("{}\n{}\n", n, n);
    s += &vec!["0".to_string(); n].join(" ");
    s += "\n";
    for i in 0..n {
        let mut row: Vec<String> = Vec::new();
        for j in 0..n {
            if j < i {
                row.push("0".to_string());
            } else if j == i {
                row.push("1".to_string());
            } else {
                row.push(format!("{}", 2usize.pow((j - i) as u32)));
            }
        }
        row.push(format!("{}", 2usize.pow(i as u32))); // RHS
        s += &row.join(" ");
        s += "\n";
    }
    s
}

#[test]
fn correctness_harness() {
    for (name, txt) in small_lps() {
        let lp = simplex::parse_lp(txt).unwrap_or_else(|_| panic!("Failed to parse LP {}", name));
        let mut solver = SimplexSolver::new(lp.clone());
        let sol = solver.solve().expect("Solver returned error");
        verify_solution(&lp, &sol);
    }

    for n in [5, 7, 10].iter() {
        let txt = klee_minty(*n);
        let lp = simplex::parse_lp(&txt).unwrap();
        let mut solver = SimplexSolver::new(lp.clone());
        let sol = solver.solve().unwrap();
        verify_solution(&lp, &sol);
    }
}
