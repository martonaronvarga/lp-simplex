mod bench;
mod io;
mod matrix;
mod phase1;
mod phase2;
mod rational;
mod simplex;
mod solution;
mod tableau;

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

    for n in [4, 5, 6, 12, 15] {
        // Klee-Minty worst case timings
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
        fn test_simple_2d_optimal() {
            // Maximize: x + y
            // Subject to: x <= 4, y <= 3
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
        fn test_3d_optimal() {
            // Maximize: 3x + 2y + z
            // Subject to: x + y + z <= 5, 2x + y <= 8
            let text = "\
3
2
3 2 1
1 1 1 5
2 1 0 8
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution for 3D problem"),
            }
        }

        #[test]
        fn test_zero_objective() {
            // Maximize: 0x + 0y (constant objective)
            // Subject to: x + y <= 10
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
            // Maximize: 2x + 3y
            // Subject to: x + y <= 10
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
            // Maximize: 5x
            // Subject to: x <= 7
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
            // Maximize: -x - y (minimize x + y)
            // Subject to: x + y >= 5, x <= 10, y <= 10
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
            // Maximize: x + 2y
            // Subject to: 2x + 3y <= 10
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
        fn test_unbounded_2d() {
            // Maximize: x + y
            // Subject to: x - y <= 0 (no upper bound on both)
            let text = "\
2
1
1 1
1 -1 0
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Unbounded { .. } => {}
                _ => panic!("expected unbounded solution"),
            }
        }

        #[test]
        fn test_unbounded_single_var() {
            // Maximize: x
            // Subject to: (no constraints)
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
        fn test_unbounded_negative_objective() {
            // Maximize: -x (unbounded in negative direction)
            let text = "\
1
0
-1
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Unbounded { .. } => {}
                _ => panic!("expected unbounded solution"),
            }
        }

        #[test]
        fn test_unbounded_multiple_vars() {
            // Maximize: x + 2y
            // Subject to: x - y <= 5
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
        fn test_infeasible_contradictory() {
            // Maximize: x
            // Subject to: x >= 5 AND x <= 2 (contradictory)
            let text = "\
1
2
1
1 1
-1 -3
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Infeasible => {}
                _ => panic!("expected infeasible solution"),
            }
        }

        #[test]
        fn test_infeasible_2d() {
            // Maximize: x + y
            // Subject to: x + y >= 10, x + y <= 5 (contradictory)
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
            // Maximize: x + y
            // Subject to: x + y <= -1 (no point satisfies with x,y >= 0)
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
            // Maximize: x + y
            // Subject to: x + y <= 5, 2x + 2y <= 10 (redundant/degenerate)
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
            // This problem is designed to cause cycling without Bland's rule
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
        fn test_optimal_at_origin() {
            // Maximize: -x - y (optimal at origin)
            // Subject to: x + y >= 0 (always satisfied)
            let text = "\
2
1
-1 -1
-1 -1 0
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "0");
                }
                _ => panic!("expected optimal solution at origin"),
            }
        }

        #[test]
        fn test_large_coefficients() {
            // Maximize: 1000x + 2000y
            // Subject to: x + y <= 100
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
        fn test_many_constraints() {
            // Maximize: x + y + z
            // Subject to: 5 different constraints
            let text = "\
3
5
1 1 1
1 0 0 10
0 1 0 20
0 0 1 15
1 1 0 25
1 0 1 20
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution with many constraints"),
            }
        }

        #[test]
        fn test_redundant_constraints() {
            // Maximize: x + y
            // Subject to: x <= 5, 2x <= 12, x <= 10
            let text = "\
2
3
1 1
1 0 5
2 0 12
1 0 10
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution with redundant constraints"),
            }
        }

        #[test]
        fn test_identity_basis() {
            // Problem where constraint matrix is already identity
            // Maximize: 1x + 2y
            // Subject to: x <= 5, y <= 8
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
            // Maximize: 2x + 4y
            // Subject to: x + 2y <= 10
            // Optimal line is parallel to constraint
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
        fn test_minimization_via_negation() {
            // Minimize: 3x + 2y ≡ Maximize: -3x - 2y
            // Subject to: x + y >= 5, x <= 10, y <= 8
            let text = "\
2
3
-3 -2
-1 -1 -5
1 0 10
0 1 8
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution for minimization problem"),
            }
        }

        // ============================================================================
        // RATIO TESTS
        // ============================================================================

        #[test]
        fn test_small_fractional_coefficients() {
            // Tests with small numbers
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
        fn test_zero_constraint_rhs() {
            // Maximize: x
            // Subject to: -x <= 0 (equivalent to x >= 0)
            let text = "\
1
1
1
-1 0
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Unbounded { .. } => {}
                _ => panic!("expected unbounded solution"),
            }
        }

        // ============================================================================
        // STRESS TESTS
        // ============================================================================

        #[test]
        fn test_4d_problem() {
            // Maximize: w + x + y + z
            // Subject to: w + x + y + z <= 20, w + x <= 10, y + z <= 15
            let text = "\
4
3
1 1 1 1
1 1 1 1 20
1 1 0 0 10
0 0 1 1 15
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "20");
                }
                _ => panic!("expected optimal solution for 4D problem"),
            }
        }

        #[test]
        fn test_many_variables_few_constraints() {
            // Maximize: sum of 6 variables
            // Subject to: 2 constraints
            let text = "\
6
2
1 1 1 1 1 1
1 1 1 0 0 0 5
0 0 0 1 1 1 7
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { objective, .. } => {
                    assert_eq!(objective.to_string(), "12");
                }
                _ => panic!("expected optimal solution"),
            }
        }

        #[test]
        fn test_many_constraints_few_variables() {
            // Maximize: x + y
            // Subject to: 5 different linear constraints
            let text = "\
2
5
1 1
1 0 4
0 1 6
1 1 8
2 0 10
0 2 12
";
            let sol = parse_and_solve(text).unwrap();
            match sol {
                Solution::Optimal { .. } => {}
                _ => panic!("expected optimal solution"),
            }
        }
    }
}
