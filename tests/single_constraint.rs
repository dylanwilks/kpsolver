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
            1.0,        [1],          2;
    }

    knapsacks! {
        knapsack<u32, 1>:
            [1]
    }

    let solution = items.clone().insert_into(knapsack).using(solver);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1_u32]);
    knapsacks! {
        knapsack_mut<u32, 1>:
            [1]
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(solver);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1);
    assert_eq!(solution_mut[0].weights(), &[1_u32]);
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
            2.0,        [1];
            3.0,        [2];
            1.0,        [1];
    }

    knapsacks! {
        knapsack<u32, 1>:
            [2]
    }

    let solution = items.insert_into(knapsack).using(solver);
    assert!(cmp(&solution.value(), &(3.0 * scale)));
}

#[allow(dead_code)]
fn random_sample_2_for_bounded_solvers<S, F>(solver: S, cmp: F, scale: f64)
where
    S: BoundedSolver<u32, 1>,
    F: Fn(&f64, &f64) -> bool,
{
    items! {
        items<u32, 1, usize>:
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
        knapsack<u32, 1>:
            [295]
    }

    let solution = items.insert_into(knapsack).using(solver);
    assert!(cmp(&solution.value(), &(302.0 * scale)));
}

macro_rules! single_constraint_bounded_test {
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

        #[test]
        fn random_sample_2() {
            $(
                random_sample_2_for_bounded_solvers($solver, $cmp, $scale);
            )*
        }
    };
}

single_constraint_bounded_test!(
    (binary_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0),
    (binary_solvers::GeneralizedGreedy, <f64 as PartialOrd>::gt, 0.5),
    (binary_solvers::TheoreticalGreedy, <f64 as PartialOrd>::gt, 0.5),
    (bounded_solvers::CBC, <f64 as PartialEq>::eq, 1.0),
    (bounded_solvers::HiGHS, <f64 as PartialEq>::eq, 1.0),
    (bounded_solvers::CPLEX, <f64 as PartialEq>::eq, 1.0),
);
