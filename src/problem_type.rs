use crate::unbound_struct::unbound;
use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::item::ProblemItems;
use crate::knapsack::ProblemKnapsacks;

pub struct BoundedProblem<T, const S: usize>
where
    T: CompatibleProblemType,
{
    pub items: ProblemItems<T, S, usize>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub struct BoundedProblemMut<'a, T, const S: usize>
where
    T: CompatibleProblemType,
{
    pub items: &'a mut ProblemItems<T, S, usize>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub trait BoundedSolver<T, const S: usize>: Clone + Copy
where
    T: CompatibleProblemType,
    Self: Sized,
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
                let index = problem.items.items_hash
                    .get_mut(&item.to_key()).unwrap();
                problem.items.items[*index].quantity -= item.quantity;
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
