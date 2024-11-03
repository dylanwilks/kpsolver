use crate::unbound_struct::unbound;
use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::item::{
    ProblemItems, ProblemItemsBinary
};
use crate::knapsack::{
    ProblemKnapsacks, ProblemKnapsacksBinary
};

pub struct BoundedProblem<T, const S: usize>
where T: CompatibleProblemType,
{
    pub items: ProblemItems<T, S>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub struct BoundedProblemMut<'a, T, const S: usize>
where T: CompatibleProblemType,
{
    pub items: &'a mut ProblemItems<T, S>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub trait BoundedSolver<T, const S: usize>: Clone + Copy
where T: CompatibleProblemType,
{
    //Required methods
    fn solve(self, problem: BoundedProblem<T, S>) -> ProblemKnapsacks<T, S>;

    //Provided methods
    fn solve_mut(self, problem: BoundedProblemMut<'_, T, S>) 
    -> ProblemKnapsacks<T, S> {
        let solution = self.solve(BoundedProblem::<T, S> {
            items: problem.items.clone(),
            knapsacks: problem.knapsacks,
        });

        for knapsack in &solution {
            for item in knapsack {
                problem.items.get_item_mut((item.value, item.weights))
                             .unwrap()
                             .quantity -= item.quantity;
            }
        }

        solution
    }
}

impl<T, const S: usize> BoundedProblem<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BoundedSolver<T, S>) 
    -> ProblemKnapsacks<T, S> {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BoundedProblemMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BoundedSolver<T, S>)
    -> ProblemKnapsacks<T, S> {
        solver.solve_mut(self)
    }
}

pub struct UnboundedProblem<T, const S: usize>
where
    T: CompatibleProblemType,
{
    pub items: ProblemItems<T, S, unbound>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub trait UnboundedSolver<T, const S: usize>: Clone + Copy
where
    T: CompatibleProblemType,
{
    //Required methods
    fn solve(self, problem: UnboundedProblem<T, S>) -> ProblemKnapsacks<T, S>;
}

impl<T, const S: usize> UnboundedProblem<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn using(self, solver: impl UnboundedSolver<T, S>) 
    -> ProblemKnapsacks<T, S> {
        solver.solve(self)
    }
}

pub struct BinaryProblem<T, const S: usize>
where T: CompatibleProblemType,
{
    pub items: ProblemItemsBinary<T, S>,
    pub knapsacks: ProblemKnapsacksBinary<T, S>,
}

pub struct BinaryProblemMut<'a, T, const S: usize>
where T: CompatibleProblemType,
{
    pub items: &'a mut ProblemItemsBinary<T, S>,
    pub knapsacks: ProblemKnapsacksBinary<T, S>,
}

pub trait BinarySolver<T, const S: usize>: Clone + Copy
where T: CompatibleProblemType {
    //Required methods
    fn solve(self, problem: BinaryProblem<T, S>) -> ProblemKnapsacksBinary<T, S>;

    //Provided methods
    fn solve_mut(self, problem: BinaryProblemMut<'_, T, S>) 
    -> ProblemKnapsacksBinary<T, S> {
        let solution = self.solve(BinaryProblem::<T, S> {
            items: problem.items.clone(),
            knapsacks: problem.knapsacks,
        });

        for knapsack in &solution {
            for item in knapsack {
                for mut_item in &mut *problem.items {
                    if item == mut_item {
                        mut_item.quantity -= item.quantity;
                        continue;
                    }
                }
            }
        }

        solution
    }
}

impl<T, const S: usize> BinaryProblem<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BinarySolver<T, S>) -> ProblemKnapsacksBinary<T, S> {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BinaryProblemMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BinarySolver<T, S>) -> ProblemKnapsacksBinary<T, S> {
        solver.solve_mut(self)
    }
}
