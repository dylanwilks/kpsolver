use crate::unbound_struct::unbound;
use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::item::{
    ProblemItems, ProblemItemsBinary
};
use crate::knapsack::{
    ProblemKnapsacks, ProblemKnapsacksBinary
};

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
    type Output;

    //Required methods
    fn solve(self, problem: BinaryProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> BinaryProblem<T, S> 
where T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as BinarySolver<T, S>>::Output 
    where N: BinarySolver<T, S>,
    {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BinaryProblemMut<'a, T, S>
where T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BinarySolver<T, S, Output = ProblemKnapsacksBinary<T, S>>) 
    -> ProblemKnapsacksBinary<T, S> {
        let solution = solver.solve(BinaryProblem::<T, S> {
            items: self.items.clone(),
            knapsacks: self.knapsacks,
        });

        for knapsack in &solution {
            for item in knapsack {
                for mut_item in &mut *self.items {
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
    type Output;

    //Required methods
    fn solve(self, problem: BoundedProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> BoundedProblem<T, S> 
where T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as BoundedSolver<T, S>>::Output
    where N: BoundedSolver<T, S>,
    {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BoundedProblemMut<'a, T, S>
where T: CompatibleProblemType,
{
    pub fn using(self, solver: impl BoundedSolver<T, S, Output = ProblemKnapsacks<T, S>>)
    -> ProblemKnapsacks<T, S> {
        let solution = solver.solve(BoundedProblem::<T, S> {
            items: self.items.clone(),
            knapsacks: self.knapsacks,
        });

        for knapsack in &solution {
            for item in knapsack {
                self.items.get_item_mut((item.value, item.weights))
                          .unwrap()
                          .quantity -= item.quantity;
            }
        }

        solution
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
    type Output;

    //Required methods
    fn solve(self, problem: UnboundedProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> UnboundedProblem<T, S> 
where T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as UnboundedSolver<T, S>>::Output
    where N: UnboundedSolver<T, S>,
    {
        solver.solve(self)
    }
}