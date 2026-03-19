use std::fmt::Write;
use std::time::Instant;

use crate::simplex::{LinearProgram, SimplexSolver};

fn generate_klee_minty_lp(n: usize) -> String {
    let mut lp_text = String::new();

    // number of variables and constraints
    writeln!(lp_text, "{n}").unwrap();
    writeln!(lp_text, "{n}").unwrap();

    // Objective: 1,2,4,...,2^(n-1)
    for i in 0..n {
        if i > 0 {
            lp_text.push(' ');
        }
        write!(lp_text, "{}", 1 << i).unwrap();
    }
    writeln!(lp_text).unwrap();

    for i in 0..n {
        for j in 0..n {
            if j < i {
                write!(lp_text, "1/{} ", 1 << (i - j)).unwrap();
            } else if j == i {
                write!(lp_text, "1 ").unwrap();
            } else {
                write!(lp_text, "0 ").unwrap();
            }
        }
        // RHS: 2^i
        writeln!(lp_text, "{}", 1 << i).unwrap();
    }

    lp_text
}

pub fn solve_and_bench(n: usize) {
    let text = generate_klee_minty_lp(n);

    let lp: LinearProgram = crate::simplex::parse_lp(&text).unwrap();
    let mut solver = SimplexSolver::new(lp);

    let start = Instant::now();
    let sol = solver.solve().unwrap();
    let duration = start.elapsed();

    println!("Solved Klee-Minty n={} in {:?}", n, duration);
    println!("{}", sol);
}
