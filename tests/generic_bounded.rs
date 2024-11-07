use kpsolver::{
    Item, Knapsack, ProblemKnapsacks,
    BoundedSolver, bounded_solvers,
    items, knapsacks,
};
use kpsolver::compatible_problem_type_trait::CompatibleProblemType;

#[allow(dead_code)]
fn random_sample_1<T, S>(solver: S, cmp: fn(&f64, &f64) -> bool, scale: f64) 
where
    T: CompatibleProblemType + From<u32>,
    S: BoundedSolver<T, 2, Output = ProblemKnapsacks<T, 2>>,
{
    items! {
        t_items<u32, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [2, 2],       70;
            5.0,        [5, 2],       70;
            10.0,       [10, 2],      70;
    }

    items! {
        items<T, 2>:
    }

    for item in t_items {
        items.add(
            Item::<T, 2> {
                value: item.value,
                weights: item.weights.map(|x| T::from(x)),
                quantity: T::from(item.quantity),
            }
        );
    }

    knapsacks! {
        t_knapsacks<u32, 2>:
            [100, 70]
    }

    knapsacks! {
        knapsacks<T, 2>:
    }

    for knapsack in t_knapsacks {
        knapsacks.add(
            Knapsack::<T, 2>::new(knapsack.capacity.map(|x| T::from(x)))
        );
    }

    let solution = items.insert_into(knapsacks).using(solver);
    assert!(cmp(&solution.value(), &(scale * 100.0)));
}

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
