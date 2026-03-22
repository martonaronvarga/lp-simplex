mod bench;
mod io;
mod matrix;
mod phase1;
mod phase2;
mod rational;
mod simplex;
mod solution;
mod tableau;
mod variable;

use std::path::PathBuf;

use anyhow::Result;
use io::read_input;
use simplex::SimplexSolver;

fn main() -> Result<()> {
    let path = PathBuf::from("../examples/42.lp");

    let input = read_input(&path)?;
    println!("{}", input);

    let lp = simplex::parse_lp(&input)?;

    let mut solver = SimplexSolver::new(lp);

    let result = solver.solve()?;

    println!("Solved test, {}", result);

    for n in [4, 5] {
        crate::bench::solve_and_bench(n);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use std::path::PathBuf;

    fn parse_and_solve(text: &str) -> Result<Solution, anyhow::Error> {
        let lp = crate::simplex::parse_lp(text)?;
        let mut solver = crate::simplex::SimplexSolver::new(lp);
        solver.solve()
    }

    #[test]
    fn parse_basic_lp() {
        let text = "\
2
2
1 1
1 0 4
0 1 3
";
        let lp = crate::simplex::parse_lp(text).unwrap();
        assert_eq!(lp.n, 2);
        assert_eq!(lp.m, 2);
        assert_eq!(lp.c.len(), 2);
    }

    #[test]
    fn optimal_lp() {
        let text = "\
2
2
1 1
1 0 4
0 1 3
";
        let sol = parse_and_solve(text).unwrap();
        match sol {
            Solution::Optimal { objective, .. } => {
                assert_eq!(objective.to_string(), "7");
            }
            _ => panic!("expected optimal solution"),
        }
    }

    #[test]
    fn unbounded_lp() {
        let text = "\
2
1
1 1
1 -1 0
";
        let sol = parse_and_solve(text).unwrap();
        println!("{}", sol);
        match sol {
            Solution::Unbounded { .. } => {}
            _ => panic!("expected unbounded solution"),
        }
    }

    #[test]
    fn infeasible_lp() {
        let text = "\
1
2
1
1 1
-1 -3
";
        let lp = crate::simplex::parse_lp(text).unwrap();
        let mut solver = crate::simplex::SimplexSolver::new(lp);
        println!("{:?}", solver.solve());
        assert!(solver
            .solve()
            .is_ok_and(|a| matches!(a, Solution::Infeasible)));
    }

    fn solve_example_file(filename: &str) -> Solution {
        let path = PathBuf::from(format!(
            "{}/examples/{}",
            env!("CARGO_MANIFEST_DIR"),
            filename
        ));
        let input = crate::io::read_input(&path).unwrap();
        let lp = crate::simplex::parse_lp(&input).unwrap();
        let mut solver = crate::simplex::SimplexSolver::new(lp);
        solver.solve().unwrap()
    }

    #[test]
    fn test_bland_lp() {
        let sol = solve_example_file("bland.lp");
        match sol {
            Solution::Optimal { .. } => {}
            _ => panic!("expected optimal solution for bland.lp"),
        }
    }

    #[test]
    fn test_blowup_lp() {
        let sol = solve_example_file("blowup.lp");
        match sol {
            Solution::Optimal { .. } | Solution::Unbounded { .. } => {}
            _ => panic!("expected solution for blowup.lp"),
        }
    }

    #[test]
    fn test_klee_minty_lp() {
        let sol = solve_example_file("klee_minty.lp");
        match sol {
            Solution::Optimal { .. } => {}
            _ => panic!("expected optimal solution for klee_minty.lp"),
        }
    }

    #[cfg(test)]
    mod extended_tests {
        use super::*;
        use crate::solution::Solution;

        fn parse_and_solve(text: &str) -> Result<Solution, anyhow::Error> {
            let lp = crate::simplex::parse_lp(text)?;
            let mut solver = crate::simplex::SimplexSolver::new(lp);
            solver.solve()
        }

        #[test]
        fn test_zero_objective() {
            let text = "\
2
1
0 0
1 1 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "0");
                }
                _ => panic!("expected optimal solution with zero objective"),
            }
        }

        #[test]
        fn test_single_constraint() {
            let text = "\
2
1
2 3
1 1 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "30");
                }
                _ => panic!("expected optimal solution"),
            }
        }

        #[test]
        fn test_single_variable() {
            let text = "\
1
1
5
1 7
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "35");
                }
                _ => panic!("expected optimal solution for single variable"),
            }
        }

        #[test]
        fn test_negative_coefficients() {
            let text = "\
2
1
-1 -1
1 1 15
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution with negative coefficients"),
            }
        }

        #[test]
        fn test_fractional_optimal_value() {
            let text = "\
2
1
1 2
2 3 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution with potential fractional value"),
            }
        }

        #[test]
        fn test_unbounded_single_var() {
            let text = "\
1
0
1
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Unbounded { .. } => {}
                _ => panic!("expected unbounded solution with no constraints"),
            }
        }

        #[test]
        fn test_unbounded_multiple_vars() {
            let text = "\
2
1
1 2
1 -1 5
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Unbounded { .. } => {}
                _ => panic!("expected unbounded solution for multiple variables"),
            }
        }

        #[test]
        fn test_infeasible_2d() {
            let text = "\
2
2
1 1
1 1 5
-1 -1 -10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Infeasible => {}
                _ => panic!("expected infeasible solution for 2D problem"),
            }
        }

        #[test]
        fn test_infeasible_no_feasible_point() {
            let text = "\
2
1
1 1
1 1 -1
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Infeasible => {}
                _ => panic!("expected infeasible solution"),
            }
        }

        #[test]
        fn test_degenerate_problem() {
            let text = "\
2
2
1 1
1 1 5
2 2 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "5");
                }
                _ => panic!("expected optimal solution for degenerate problem"),
            }
        }

        #[test]
        fn test_bland_rule_cycling() {
            let text = "\
3
4
10 6 -10
1 1 0 5
0 0 1 0
1 0 1 2
0 1 1 1
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } | Solution::Infeasible => {}
                _ => panic!("expected solution (optimal or infeasible) for cycling test"),
            }
        }

        #[test]
        fn test_large_coefficients() {
            let text = "\
2
1
1000 2000
1 1 100
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "200000");
                }
                _ => panic!("expected optimal solution with large coefficients"),
            }
        }

        #[test]
        fn test_identity_basis() {
            let text = "\
2
2
1 2
1 0 5
0 1 8
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "21");
                }
                _ => panic!("expected optimal solution"),
            }
        }

        #[test]
        fn test_multiple_optimal_solutions() {
            let text = "\
2
1
2 4
1 2 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "20");
                }
                _ => panic!("expected optimal solution (multiple optima exist)"),
            }
        }

        #[test]
        fn test_small_fractional_coefficients() {
            let text = "\
2
1
1 1
1 1 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution with fractional coefficients"),
            }
        }

        #[test]
        fn test_4d_problem() {
            let text = "\
4
3
1 1 1 1
1 1 1 1 20
1 1 0 0 10
0 0 1 1 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "20");
                }
                _ => panic!("expected optimal solution for 4D problem"),
            }
        }
    }
}
