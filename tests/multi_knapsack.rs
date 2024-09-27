use kpsolver::{
    Knapsack, ProblemKnapsacks,
    BoundedSolver,
    binary_solvers, bounded_solvers,
    items, knapsacks
};

use std::cmp::{PartialEq, PartialOrd};

#[allow(dead_code)]
fn simple_sample_for_bounded_solvers<S>(solver: S)
where
    S: BoundedSolver<u32, 1>,
{
    items! {
        items<u32, 1, usize>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1],          3;
    }

    knapsacks! {
        knapsacks<u32, 1>:
            [1]
            [1]
    }

    let solution = items.clone().insert_into(knapsacks).using(solver);
    assert_eq!(solution.value(), 2.0);
    assert_eq!(solution[0].weights(), &[1_u32]);
    assert_eq!(solution[1].weights(), &[1_u32]);
    knapsacks! {
        knapsacks_mut<u32, 1>:
            [1]
            [1]
    }

    let solution_mut = items.insert_mut_into(knapsacks_mut).using(solver);
    assert_eq!(solution.value(), 2.0);
    assert_eq!(items[0].quantity, 1);
    assert_eq!(solution_mut[0].weights(), &[1_u32]);
    assert_eq!(solution_mut[1].weights(), &[1_u32]);
}

#[allow(dead_code)]
fn random_sample_1_for_bounded_solvers<S, F>(solver: S, cmp: F, scale: f64)
where
    S: BoundedSolver<u32, 1>,
    F: Fn(&f64, &f64) -> bool,
{
    items! {
        items<u32, 1, usize>:
            /* Value */ /* Weights */ /* Quantity (?) */
            10.0,       [48];
            30.0,       [30];
            25.0,       [42];
            50.0,       [36];
            35.0,       [36];
            30.0,       [48];
            15.0,       [42];
            40.0,       [42];
            30.0,       [36];
            35.0,       [24];
            45.0,       [30];
            10.0,       [30];
            20.0,       [42];
            30.0,       [36];
            25.0,       [36];
    }

    knapsacks! {
        knapsacks<u32, 1>:
            [100]
            [100]
            [100]
            [100]
            [100]
    }

    let solution = items.insert_into(knapsacks).using(solver);
    assert!(cmp(&solution.value(), &(395.0 * scale)));
}

macro_rules! multi_knapsack_bounded_test {
    ($(($solver:expr, $cmp:expr, $scale:literal),)*$(,)?) => {
        #[test]
        fn simple_sample() {
            $(
                simple_sample_for_bounded_solvers($solver);
            )*
        }

        #[test]
        fn random_sample_1() {
            $(
                random_sample_1_for_bounded_solvers($solver, $cmp, $scale);
            )*
        }   
    };
}

multi_knapsack_bounded_test!(
    (binary_solvers::GeneralizedGreedy, <f64 as PartialOrd>::gt, 0.5),
    //(bounded_solvers::CBC, <f64 as PartialEq>::eq, 1.0), #OVERFLOWS
    (bounded_solvers::HiGHS, <f64 as PartialEq>::eq, 1.0),
    (bounded_solvers::CPLEX, <f64 as PartialEq>::eq, 1.0),
);
