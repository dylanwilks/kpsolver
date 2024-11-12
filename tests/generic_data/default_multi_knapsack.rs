use kpsolver::{
    items_binary,
    knapsacks_binary,
    BinarySolver,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;

#[allow(dead_code)]
pub fn random_sample_1<T, S>(solver: S) //OPTIMAL: 100.0
-> <S as BinarySolver<T, 1>>::Output
where
    T: CompatibleProblemType + From<u32>,
    S: BinarySolver<T, 1>,
{
    items_binary! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            10.0,       48; 
            30.0,       30;
            25.0,       42;
            50.0,       36;
            35.0,       36;
            30.0,       48;
            15.0,       42;
            40.0,       42;
            30.0,       36;
            35.0,       24;
            45.0,       30;
            10.0,       30;
            20.0,       42;
            30.0,       36;
            25.0,       36;
    }

    knapsacks_binary! {
        knapsacks<u32, 1>:
            100;
            100;
            100;
            100;
            100;
    }
    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

macro_rules! default_multi_knapsack {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Bounded1Tuple(
                $crate::generic_data::default_multi_knapsack::random_sample_1::<$type, $solver>),
                395.0
            ),
        ]
    }
}

macro_rules! default_multi_knapsack_binary {
    ($type:ty, $solver:ty) => {
        [
            (
                $crate::generic_data::Problems::<$type, $solver>::Binary1Tuple(
                $crate::generic_data::default_multi_knapsack::random_sample_1::<$type, $solver>),
                395.0
            ),
        ]
    }
}
