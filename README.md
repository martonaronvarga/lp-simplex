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

## Usage

To use the solver, you need to have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system. Clone the repository and build the project with `cargo build --release`. You can run the solver directly with `cargo run --release`, or execute the full test suite with `cargo test`.

## Using with Nix

If you are using [Nix](https://nixos.org/) or [Nix flakes](https://nixos.wiki/wiki/Flakes), you can build and run the project in a reproducible environment without needing to install Rust globally. In the project directory, run `nix develop` to enter a shell with Rust and Cargo available. Then you can use `cargo build`, `cargo run`, and `cargo test` as usual. This ensures all dependencies are pinned and no system-wide installation is required.
