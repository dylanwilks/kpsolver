use kpsolver::{
    items,
    Knapsack, ProblemKnapsacks, knapsacks,
    BoundedSolver,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;

#[allow(dead_code)]
pub fn random_sample_1<T, S>(solver: S) //OPTIMAL: 100.0
-> <S as BoundedSolver<T, 2>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BoundedSolver<T, 2>,
{
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

    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

macro_rules! default_multi_constraint {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded2Tuple(
                $crate::generic_data::default_multi_constraint::random_sample_1::<$type, $solver>),
                100.0
            ),
        ]
    }
}
