pub use item::{
    Item, ItemBinary, ItemUnbound, 
    ProblemItems, ProblemItemsBinary, ProblemItemsUnbound
};
pub use knapsack::{
    Knapsack, KnapsackBinary,
    ProblemKnapsacks, ProblemKnapsacksBinary
};

pub use problem_type::{
    BoundedProblem, BoundedSolver, 
    BinaryProblem, BinarySolver,
    UnboundedProblem, UnboundedSolver
};
pub use unbound_struct::unbound;
 
pub mod compatible_problem_type_trait;
#[macro_use]
pub mod item;
pub mod knapsack;
pub mod problem_type;
pub mod unbound_struct;
mod items_macro;
mod knapsacks_macro;

pub mod binary_solvers;
pub mod bounded_solvers;
//pub mod unbounded_solvers;
