# kpsolver

Provides an interface and some algorithms for solving variations of the knapsack problems; namely Binary, Bounded, and Unbounded problems, MDKPs (Multi-dimensional knapsack problems), MKPs (Multiple knapsack problems), or a mixture of them.

```rust
use kpsolver::{items, knapsacks, BoundedSolver, bounded_solvers};

fn main() {
    items! {
        items<u32, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [2, 2],       70;
            5.0,        [5, 2],       70;
            10.0,       [10, 2],      70;
    }

    knapsacks! {
        knapsacks<u32, 2>:
            [100, 70];
    }

    let solution = items
        .insert_into(knapsacks)
        .using(bounded_solvers::Dynamic);

    println!("Optimal value: {}", solution.value());
    for item in solution {
        println!("value: {}, weights: [{}, {}], quantity: {}",
        item.value, item.weights[0], item.weights[1], item.quantity);
    }
}
```

## Solvers
So far this library implements 3 algorithms described in papers/articles and wraps 3 more algorithms through the `good_lp` package. More will be added later.

### Dynamic
A dynamic programming solver for the MDKP variant. The implementation extends the algorithm described [here](https://en.wikipedia.org/wiki/Knapsack_problem#0-1_knapsack_problem).
You can find the exact implementation of the binary variant in [`dynamic.rs`](https://github.com/dylanwilks/kpsolver/blob/main/src/binary_solvers/dynamic.rs).\\
Can only take in items and knapsacks of type `u32`. If more than 1 knapsack is provided it will only modify the first.

### Generalized Greedy
Takes a hybrid approach of the MDKP and MKP generalized greedy algorithms featured in pages 256-259 and 299 respectively in the book [Knapsack Problems](https://link.springer.com/book/10.1007/978-3-540-24777-7). 
The implementation of the binary variant is in [`generalized_greedy.rs`](https://github.com/dylanwilks/kpsolver/blob/main/src/binary_solvers/generalized_greedy.rs).\\
Can only take in items and knapsacks of type `f64`.

### Theoretical Greedy
Implements the algorithm described in this [article](https://www.sciencedirect.com/science/article/pii/0166218X9390051O) to solve MDKPs. It is more accurate than generalized greedy but slower.
Binary variant implementation is in [`theoretical_greedy.rs`](https://github.com/dylanwilks/kpsolver/blob/main/src/binary_solvers/theoretical_greedy.rs).\\
Can only take in items and knapsacks of type `f64`. If more than 1 knapsack is provided it will only modify the first.

### CBC, HiGHS and CPLEX
The library uses `good_lp` to interface the aforementioned algorithms to solve the following linear programming model (MDKP and MKP):

$$
\begin{aligned}
\text{maximize}\hspace{4mm}&\sum_{i=1}^m \sum_{j=1}^n p_j x_{ij}\\
\text{subject to}\hspace{3.5mm}&\sum_{j=1}^m w_{kj} x_{ij} \leq c_{ik}\hspace{5mm}i = 1,...,m\ \text{ and }\ k = 1,...,d\\
\hspace{25mm}&\sum_{i=1}^m x_{ij} \leq q_j\hspace{11mm}j = 1,...n  \\
\hspace{25mm}&x_{ij} \in \lbrace 0,...,q_j \rbrace\hspace{6mm}i = 1,...,m\ \text{ and }\ j = 1,...,n\\
\end{aligned}
$$

where:
$p_j$     	 &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;profit of item $j$\\
$x_{ij}$    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;quantity of item $j$ in knapsack $i$\\
$w_{kj}$ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;$k$-th weight of item $j$\\
$c_{ik}$ &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;$k$-th capacity of knapsack $i$\\
$q_j$     	 &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;quantity of item $j$ &nbsp;&nbsp;($q_j = 1$ in the binary case)\\

To use these solvers these features need to be enabled in your `Cargo.toml`:

```toml
kpsolver = { version = "*", features = ["cbc", "highs", "cplex"] }
```

See their [page](https://github.com/rust-or/good_lp) for more details.

## Implementing Your Own
Solvers can be implemented through any of the following traits:
- `BinarySolver`
- `BoundedSolver`
- `UnboundedSolver`

These traits have the relationship `BinarySolver` $\subset$ `BoundedSolver` $\subset$ `UnboundedSolver`; i.e if a solver can solve bounded problems, it can also solve binary problems. For unbounded solvers, it will be assumed that each item has infinite quantity. 

You will have to create your own object to implement these traits on. Here is an example of what this may look like:

```rust
use kpsolver::{BoundedSolver, BoundedProblemKnapsacks, BoundedProblem, ...}

pub struct Some_Solver;
impl<const S: usize> BoundedSolver<f64, S> for Some_Solver {
    type Output = BoundedProblemKnapsacks<f64, S>;

    fn solve(self, mut problem: BoundedProblem<f64, S>) -> Self::Output {
        ...
    }
}
```

Documentation of types, objects, and other features to come... eventually.
