# lp-simplex

This is a toy LP solver implementation with the following constraints:
- rational arithmetic
- two-phase primal simplex algorithm
- integer constraints and matrix elements
- optimal solution returns the optimal value and primal & dual locations of solution
- unbounded problem returns an increasing direction vector
- reads input from file
- output returns as rationals

## input format

```input.lp
n                         # number of variables
m                         # number of rows
c_1 c_2 ... c_n           # objective function
a_1_1 a_1_2 ... a_1_n b_1 # first row
a_2_1 a_2_2 ... a_2_n b_2 # second row
.                         .
.                         .
.                         .
a_m_1 a_m_2 ... a_m_n b_n # m-th row
```
