use crate::matrix::Matrix;
use crate::rational::Rational;
use crate::solution::Solution;
use crate::tableau::Tableau;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct LinearProgram {
    pub m: usize,
    pub n: usize,
    pub a: Matrix,
    pub b: Vec<Rational>,
    pub c: Vec<Rational>,
}

pub fn parse_lp(text: &str) -> Result<LinearProgram> {
    use anyhow::{bail, Context};

    let mut lines = text.lines();

    let n: usize = lines
        .next()
        .context("missing n")?
        .trim()
        .parse()
        .context("invalid n")?;

    let m: usize = lines
        .next()
        .context("missing m")?
        .trim()
        .parse()
        .context("invalid m")?;

    let c: Vec<Rational> = lines
        .next()
        .context("missing objective row")?
        .split_whitespace()
        .map(|x| x.parse::<Rational>())
        .collect::<Result<_, _>>()
        .context("invalid objective coefficient")?;

    if c.len() != n {
        bail!("objective row must contain {} coefficients", n);
    }

    let mut a = Matrix::new(m, n);
    let mut b = Vec::with_capacity(m);

    for i in 0..m {
        let line = lines
            .next()
            .context(format!("missing constraint row {}", i))?;

        let row: Vec<Rational> = line
            .split_whitespace()
            .map(|x| x.parse::<Rational>())
            .collect::<Result<_, _>>()
            .context("invalid matrix entry")?;

        if row.len() != n + 1 {
            bail!("row {} must contain {} coefficients + RHS", i, n);
        }

        (0..n).for_each(|j| {
            *a.index_mut(i, j) = row[j].clone();
        });

        b.push(row[n].clone());
    }

    Ok(LinearProgram { m, n, a, b, c })
}

pub struct SimplexSolver {
    lp: LinearProgram,
}

impl SimplexSolver {
    pub fn new(lp: LinearProgram) -> Self {
        Self { lp }
    }

    pub fn solve(&mut self) -> Result<Solution> {
        let mut tableau = Tableau::from_lp(&self.lp);
        println!("==== PROBLEM ====");
        println!("Variables: n = {}", self.lp.n);
        println!("Constraints: m = {}", self.lp.m);
        println!("Objective coefficients: {:?}", self.lp.c);
        println!("A (constraints matrix):");
        for i in 0..self.lp.m {
            print!("  ");
            for j in 0..self.lp.n {
                print!("{:>8} ", self.lp.a.index(i, j));
            }
            println!("| rhs = {}", self.lp.b[i]);
        }
        println!("===================");

        println!("==== Initial Tableau ====");
        tableau.pretty_print();

        let phase1_res = tableau.phase1();

        match phase1_res {
            Err(sol) => Ok(sol),
            Ok(()) => Ok(tableau.phase2(&self.lp.c)),
        }
    }
}
