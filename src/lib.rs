pub use item::{
    Item, BinaryItem, UnboundedItem, 
    ProblemItems, BinaryProblemItems, UnboundedProblemItems
};
pub use knapsack::{
    Knapsack, BinaryKnapsack,
    ProblemKnapsacks, BinaryProblemKnapsacks
};

pub use problem_type::{
    BoundedProblem, BoundedSolver, 
    BinaryProblem, BinarySolver,
    UnboundedProblem, UnboundedSolver
};
pub use unbounded_struct::unbounded;
 
pub mod compatible_problem_type_trait;
#[macro_use]
pub mod item;
pub mod knapsack;
pub mod problem_type;
pub mod unbounded_struct;
mod items_macro;
mod knapsacks_macro;

pub mod binary_solvers;
pub mod bounded_solvers;
//pub mod unboundeded_solvers;
