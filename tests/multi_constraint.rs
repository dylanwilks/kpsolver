use kpsolver::{
    Knapsack, ProblemKnapsacks,
    BoundedSolver,
    binary_solvers, bounded_solvers,
    items, knapsacks
};

#[allow(dead_code)]
fn simple_sample_for_bounded_solvers<S>(solver: S) 
where
    S: BoundedSolver<u32, 2>,
{
    items! {
        items<u32, 2, usize>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1, 1],       2;
    }

    knapsacks! {
        knapsack<u32, 2>:
            [1, 1]
    }

    let solution = items.clone().insert_into(knapsack).using(solver);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1_u32, 1_u32]);
    knapsacks! {
        knapsack_mut<u32, 2>:
            [1, 1]
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(solver);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1);
    assert_eq!(solution_mut[0].weights(), &[1_u32, 1_u32]);
}

#[allow(dead_code)]
fn random_sample_1_for_bounded_solvers<S, F>(solver: S, cmp: F, scale: f64) 
where
    S: BoundedSolver<u32, 2>,
    F: Fn(&f64, &f64) -> bool,
{
    items! {
        items<u32, 2, usize>:
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

    knapsacks! {
        knapsack<u32, 2>:
            [269, 175]
    }

    let solution = items.insert_into(knapsack).using(solver);
    assert!(cmp(&solution.value(), &(280.0 * scale)));
}

#[allow(dead_code)]
fn random_sample_2_for_bounded_solvers<S, F>(solver: S, cmp: F, scale: f64)
where
    S: BoundedSolver<u32, 2>,
    F: Fn(&f64, &f64) -> bool,
{
    items! {
        items<u32, 2, usize>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [2, 2],       70;
            5.0,        [5, 2],       70;
            10.0,       [10, 2],      70;
    }

    knapsacks! {
        knapsack<u32, 2>:
            [100, 70]
    }

    let solution = items.insert_into(knapsack).using(solver);
    assert!(cmp(&solution.value(), &(100.0 * scale)));
}

macro_rules! multi_constraint_bounded_test {
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

multi_constraint_bounded_test!(
    (binary_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0),
    (binary_solvers::GeneralizedGreedy, <f64 as PartialOrd>::gt, 0.5),
    (binary_solvers::TheoreticalGreedy, <f64 as PartialOrd>::gt, 0.5),
    (bounded_solvers::CBC, <f64 as PartialEq>::eq, 1.0),
    (bounded_solvers::HiGHS, <f64 as PartialEq>::eq, 1.0),
    (bounded_solvers::CPLEX, <f64 as PartialEq>::eq, 1.0),
);
