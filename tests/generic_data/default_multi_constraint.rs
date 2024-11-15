use kpsolver::compatible_problem_type_trait::CompatibleProblemType;
use kpsolver::{items, items_binary, knapsacks, knapsacks_binary, BinarySolver, BoundedSolver};

#[allow(dead_code)]
pub fn random_sample_1<T, S>(solver: S) -> <S as BoundedSolver<T, 2>>::Output
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

    items
        .to_generic::<T>()
        .insert_into(knapsacks.to_generic::<T>())
        .using(solver)
}

#[allow(dead_code)]
pub fn random_sample_2<T, S>(solver: S) -> <S as BinarySolver<T, 2>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BinarySolver<T, 2>,
{
    items_binary! {
        items<u32, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            55.0,       [95, 34];
            10.0,       [4, 64];
            47.0,       [60, 13];
            5.0,        [32, 35];
            4.0,        [23, 9];
            50.0,       [72, 87];
            8.0,        [80, 35];
            61.0,       [62, 12];
            85.0,       [65, 54];
            87.0,       [46, 92];
    }

    knapsacks_binary! {
        knapsacks<u32, 2>:
            [269, 175];
    }

    items
        .to_generic::<T>()
        .insert_into(knapsacks.to_generic::<T>())
        .using(solver)
}

#[allow(unused_macros)]
macro_rules! default_multi_constraint {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded2Tuple(
                $crate::generic_data::default_multi_constraint::random_sample_1::<$type, $solver>),
                100.0
            ),
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded2Tuple(
                $crate::generic_data::default_multi_constraint::random_sample_2::<$type, $solver>),
                280.0
            ),
        ]
    }
}

#[allow(unused_macros)]
macro_rules! default_multi_constraint_binary {
    ($type:ty, $solver:ty) => {
        [(
            $crate::generic_data::Problems::<$type, $solver>::Binary2Tuple(
                $crate::generic_data::default_multi_constraint::random_sample_2::<$type, $solver>,
            ),
            280.0,
        )]
    };
}
