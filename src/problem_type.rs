use crate::compatible_problem_type_trait::{CompatibleProblemType, UnboundedCompatibility};
use crate::item::{BinaryProblemItems, ProblemItems, UnboundedItem, UnboundedProblemItems};
use crate::knapsack::{BinaryProblemKnapsacks, Knapsack, ProblemKnapsacks};
use crate::unbounded_struct::unbounded;

pub struct BinaryProblem<T, const S: usize>
where
    T: CompatibleProblemType,
{
    pub items: BinaryProblemItems<T, S>,
    pub knapsacks: BinaryProblemKnapsacks<T, S>,
}

pub struct BinaryProblemMut<'a, T, const S: usize>
where
    T: CompatibleProblemType,
{
    pub items: &'a mut BinaryProblemItems<T, S>,
    pub knapsacks: BinaryProblemKnapsacks<T, S>,
}

pub trait BinarySolver<T, const S: usize>: Clone + Copy
where
    T: CompatibleProblemType,
{
    type Output;

    //Required methods
    fn solve(self, problem: BinaryProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> BinaryProblem<T, S>
where
    T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as BinarySolver<T, S>>::Output
    where
        N: BinarySolver<T, S>,
    {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BinaryProblemMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    pub fn using(
        self,
        solver: impl BinarySolver<T, S, Output = BinaryProblemKnapsacks<T, S>>,
    ) -> BinaryProblemKnapsacks<T, S> {
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
pub struct BoundedProblem<T, const S: usize, N = T>
where
    T: CompatibleProblemType,
    N: UnboundedCompatibility,
{
    pub items: ProblemItems<T, S, N>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub struct BoundedProblemMut<'a, T, const S: usize, N = T>
where
    T: CompatibleProblemType,
    N: UnboundedCompatibility,
{
    pub items: &'a mut ProblemItems<T, S, N>,
    pub knapsacks: ProblemKnapsacks<T, S>,
}

pub trait BoundedSolver<T, const S: usize>: Clone + Copy
where
    T: CompatibleProblemType,
{
    type Output;

    //Required methods
    fn solve(self, problem: BoundedProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> BoundedProblem<T, S>
where
    T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as BoundedSolver<T, S>>::Output
    where
        N: BoundedSolver<T, S>,
    {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> BoundedProblemMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    pub fn using(
        self,
        solver: impl BoundedSolver<T, S, Output = ProblemKnapsacks<T, S>>,
    ) -> ProblemKnapsacks<T, S> {
        let solution = solver.solve(BoundedProblem::<T, S> {
            items: self.items.clone(),
            knapsacks: self.knapsacks,
        });

        for knapsack in &solution {
            for item in knapsack {
                self.items
                    .get_item_mut((item.value, item.weights))
                    .unwrap()
                    .quantity -= item.quantity;
            }
        }

        solution
    }
}

impl<T, const S: usize, N> BinarySolver<T, S> for N
where
    T: CompatibleProblemType,
    N: BoundedSolver<T, S>,
{
    type Output = <N as BoundedSolver<T, S>>::Output;

    fn solve(self, problem: BinaryProblem<T, S>) -> Self::Output {
        let bounded_problem = BoundedProblem::<T, S> {
            items: {
                let mut problem_items = ProblemItems::<T, S>::new();
                for item in problem.items {
                    problem_items.add(item);
                }

                problem_items
            },
            knapsacks: {
                let mut problem_knapsacks = ProblemKnapsacks::<T, S>::new();
                for knapsack in problem.knapsacks {
                    problem_knapsacks.add(Knapsack::<T, S>::new(knapsack.capacity));
                    for (i, item) in knapsack.into_iter().enumerate() {
                        problem_knapsacks[i].add(item);
                    }
                }

                problem_knapsacks
            },
        };

        <N as BoundedSolver<T, S>>::solve(self, bounded_problem)
    }
}

pub type UnboundedProblem<T, const S: usize> = BoundedProblem<T, S, unbounded>;
pub type UnboundedProblemMut<'a, T, const S: usize> = BoundedProblemMut<'a, T, S, unbounded>;

pub trait UnboundedSolver<T, const S: usize>: Clone + Copy
where
    T: CompatibleProblemType,
{
    type Output;

    //Required methods
    fn solve(self, problem: UnboundedProblem<T, S>) -> Self::Output;
}

impl<T, const S: usize> UnboundedProblem<T, S>
where
    T: CompatibleProblemType,
{
    pub fn using<N>(self, solver: N) -> <N as UnboundedSolver<T, S>>::Output
    where
        N: UnboundedSolver<T, S>,
    {
        solver.solve(self)
    }
}

impl<'a, T, const S: usize> UnboundedProblemMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    pub fn using(
        self,
        solver: impl UnboundedSolver<T, S, Output = ProblemKnapsacks<T, S>>,
    ) -> ProblemKnapsacks<T, S> {
        let solution = solver.solve(UnboundedProblem::<T, S> {
            items: self.items.clone(),
            knapsacks: self.knapsacks,
        });

        solution
    }
}

impl<T, const S: usize, N> BoundedSolver<T, S> for N
where
    T: CompatibleProblemType,
    N: UnboundedSolver<T, S>,
{
    type Output = <N as UnboundedSolver<T, S>>::Output;

    fn solve(self, problem: BoundedProblem<T, S>) -> Self::Output {
        let unbounded_problem = UnboundedProblem::<T, S> {
            items: {
                let mut problem_items = UnboundedProblemItems::<T, S>::new();
                for item in problem.items {
                    problem_items.add(UnboundedItem::<T, S> {
                        value: item.value,
                        weights: item.weights,
                        quantity: unbounded,
                    });
                }

                problem_items
            },
            knapsacks: {
                let mut problem_knapsacks = ProblemKnapsacks::<T, S>::new();
                for knapsack in problem.knapsacks {
                    problem_knapsacks.add(Knapsack::<T, S>::new(knapsack.capacity));
                    for (i, item) in knapsack.into_iter().enumerate() {
                        problem_knapsacks[i].add(item);
                    }
                }

                problem_knapsacks
            },
        };

        <N as UnboundedSolver<T, S>>::solve(self, unbounded_problem)
    }
}
