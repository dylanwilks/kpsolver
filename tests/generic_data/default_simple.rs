use kpsolver::{
    items_binary,
    knapsacks_binary,
    BinarySolver,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;

#[allow(dead_code)]
pub fn random_sample_1<T, S>(solver: S) //OPTIMAL: 302.0
-> <S as BinarySolver<T, 1>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BinarySolver<T, 1>,
{
    items_binary! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            55.0,       95;
            10.0,       4;
            47.0,       60;
            5.0,        32;
            4.0,        23;
            50.0,       72;
            8.0,        80;
            61.0,       62;
            85.0,       65;
            87.0,       46;
    }

    knapsacks_binary! {
        knapsacks<u32, 1>:
            295;
    }

    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

#[allow(dead_code)]
pub fn random_sample_2<T, S>(solver: S) //OPTIMAL: 302.0
-> <S as BinarySolver<T, 1>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BinarySolver<T, 1>,
{
    items_binary! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        1;
            3.0,        2;
            1.0,        1;
    }

    knapsacks_binary! {
        knapsacks<u32, 1>:
            2;
    }

    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

macro_rules! default_simple {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded1Tuple(
                $crate::generic_data::default_simple::random_sample_1::<$type, $solver>),
                302.0
            ),
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded1Tuple(
                $crate::generic_data::default_simple::random_sample_2::<$type, $solver>),
                3.0
            ),
        ]
    }
}

macro_rules! default_simple_binary {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Binary1Tuple(
                $crate::generic_data::default_simple::random_sample_1::<$type, $solver>),
                302.0
            ),
            (
                $crate::generic_data::Problems::<$type, $solver>::Binary1Tuple(
                $crate::generic_data::default_simple::random_sample_2::<$type, $solver>),
                3.0
            ),
        ]
    }
}
