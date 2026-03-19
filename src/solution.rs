use std::fmt::Display;

use crate::rational::Rational;

#[derive(Debug, Clone)]
pub enum Solution {
    Optimal {
        objective: Rational,
        primal: Vec<Rational>,
        dual: Vec<Rational>,
    },
    Unbounded {
        direction: Vec<Rational>,
    },
    Infeasible,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Optimal {
                objective,
                primal,
                dual,
            } => {
                writeln!(f, "Optimal solution")?;
                writeln!(f, "Objective: {}", objective)?;
                writeln!(f, "Primal: {:?}", primal)?;
                writeln!(f, "Dual: {:?}", dual)
            }

            Solution::Unbounded { direction } => {
                writeln!(f, "Unbounded")?;
                writeln!(f, "Direction: {:?}", direction)
            }

            Solution::Infeasible => writeln!(f, "Infeasible"),
        }
    }
}
