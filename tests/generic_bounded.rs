use kpsolver::{
    Item, ProblemItems, items,
    Knapsack, ProblemKnapsacks, knapsacks,
    BoundedSolver,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;

#[allow(dead_code)]
fn random_sample_1<T, S>(solver: S)
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
            [100, 70]
    }

    items.to_generic::<T>().insert_into(knapsacks.to_generic::<T>()).using(solver)
}

/*
macro_rules! default_multi_constraint {
    ($type:ty, $solver:ty) => {
        random_sample_1::<$type, $solver>
    }
}

macro_rules! selective_test {
    (
    $test_name:ident {
        DEFAULT($solver:path, $cmp:expr, $scale:literal): [$($default_fn:expr),*]
        IGNORE: [$($ignore_fn:expr),*]
        CUSTOM: [$($custom_call:expr),*]
    }
    ) => {
        #[test]
        fn $test_name() {
            let default: Vec<fn($solver, fn(&f64, &f64) -> bool, f64)>
                = vec![$($default_fn),*];
            let ignore: Vec<fn($solver, fn(&f64, &f64) -> bool, f64)>
                = vec![$($ignore_fn),*];

            for default_fn in default {
                if !ignore.contains(&(
                        default_fn as fn
                        (
                            $solver,
                            fn(&f64, &f64) -> bool,
                            f64
                        )
                )) {
                    default_fn($solver, $cmp, $scale);
                }
            }

            $($custom_call;)*
        }
    };
}

selective_test! {
    dynamic_tests {
        DEFAULT(bounded_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0):
            [
                default_multi_constraint!(u32, bounded_solvers::Dynamic)
            ]
        IGNORE: 
            [
                //random_sample_1::<u32, bounded_solvers::Dynamic>
            ]
        CUSTOM: 
            [
                random_sample_1::<u32, bounded_solvers::Dynamic>
                    (
                        bounded_solvers::Dynamic,
                        <f64 as PartialEq>::eq,
                        1.0
                    )
            ]
    }
}
*/
