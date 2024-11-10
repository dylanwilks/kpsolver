use kpsolver::{
    items,
    Knapsack, ProblemKnapsacks, knapsacks,
    BinarySolver,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;
/*
#[allow(dead_code)]
pub fn random_sample_1<T, S>(solver: S) //OPTIMAL: 302.0
-> <S as BinarySolver<T, 1>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BinarySolver<T, 1>,
{
    items! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            55.0,       [95];
            10.0,       [4];
            47.0,       [60];
            5.0,        [32];
            4.0,        [23];
            50.0,       [72];
            8.0,        [80];
            61.0,       [62];
            85.0,       [65];
            87.0,       [46];
    }

    knapsacks! {
        knapsacks<u32, 1>:
            [295]
    }

    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

macro_rules! default_simple {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Binary1Tuple(
                $crate::generic_data::default_multi_constraint::random_sample_1::<$type, $solver>),
                302.0
            ),
        ]
    }
}
*/
