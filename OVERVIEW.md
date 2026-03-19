# Overview

### 1. Architecture

LP input file -> (Idris DSL parser - optional) -> LP AST / JSON ->
Rust solver core -> (optinal MLIR kernel generation) -> solution output

### 2. Solver architecture

1. Use arbitrary precision naturals
2. Represent "Rational"
3. Represent "Matrix"
4. Represent "Tableau"

### 3. Core algorithm modules

1. Phase I
- solve: minimize sum(a_i)
- goal: obtain feasible basis
- outputs: feasible basis OR infeasible

2. Phase II
- solve: original objective c^Tx
- main loop:
```
  while reduced_const < 0:
    choose entering variable
    perform ratio test
    pivot
```
- pivot formula: `T'[i,j] = T[i,j] - T[i,e] * T[l,j] / T[l,e]`
- steps: normalize pivot row, eliminate pivot column in other rows

### 4. Required features
1. Optimal solution
- optimal objective value, x*, y*
2. Unbounded problem
- unbounded direction
3. Infeasible problem
4. Input format in readme.md
