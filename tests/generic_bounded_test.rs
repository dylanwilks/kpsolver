#[allow(unused_imports)]
use kpsolver::{bounded_solvers, binary_solvers};
use generic_data::Problems;

#[macro_use]
mod generic_data;

#[cfg(all(
    feature = "cplex",
    feature = "highs",
))]
selective_tests! {
    fn bounded_dynamic_test(bounded_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple!(u32, bounded_solvers::Dynamic),
            default_multi_constraint!(u32, bounded_solvers::Dynamic),
        }
        IGNORE: {}
        CUSTOM: {
            //large number of small tests
            for _ in 0..1000 {
                {
                let (solution, optimal_solution) =
                generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 1>
                (
                    bounded_solvers::Dynamic,
                    bounded_solvers::CPLEX,
                    100,
                    1,
                    0.0,
                    100.0,
                    [0; 1],
                    [100; 1],
                    1,
                    10,
                    [1000; 1],
                    [2000; 1],
                );

                assert_eq!(solution.value(), optimal_solution.value());
                }
            }

            //1 large test
            {
            let (solution, optimal_solution) =
            generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 1>
            (
                bounded_solvers::Dynamic,
                bounded_solvers::HiGHS,
                2000,
                1,
                0.0,
                100.0,
                [0; 1],
                [100; 1],
                1,
                10,
                [5000; 1],
                [10000; 1],
            );

            assert_eq!(solution.value(), optimal_solution.value());
            }

            //test 2 dimensions
            {
            let (solution, optimal_solution) =
            generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 2>
            (
                bounded_solvers::Dynamic,
                bounded_solvers::CPLEX,
                100,
                1,
                0.0,
                100.0,
                [0; 2],
                [100; 2],
                1,
                10,
                [1000; 2],
                [2000; 2],
            );

            assert_eq!(solution.value(), optimal_solution.value());
            }

            //test 3 dimensions, small
            {
            let (solution, optimal_solution) =
            generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 3>
            (
                bounded_solvers::Dynamic,
                bounded_solvers::CPLEX,
                10,
                1,
                0.0,
                100.0,
                [0; 3],
                [100; 3],
                1,
                10,
                [100; 3],
                [200; 3],
            );

            assert_eq!(solution.value(), optimal_solution.value());
            }
        }
    }
}

#[cfg(all(
    feature = "cplex",
))]
selective_tests! {
    fn bounded_generalized_greedy_test(bounded_solvers::GeneralizedGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::GeneralizedGreedy),
            default_multi_constraint!(f64, bounded_solvers::GeneralizedGreedy),
            default_multi_knapsack!(f64, bounded_solvers::GeneralizedGreedy),
        }
        IGNORE: {}
        CUSTOM: {
            //lots of tests for 2 knapsacks, 2 dimensions
            {
            for _ in 0..100 {
                let (solution, optimal_solution) =
                generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 2>
                (
                    bounded_solvers::GeneralizedGreedy,
                    bounded_solvers::CPLEX,
                    100,
                    2,
                    0.0,
                    100.0,
                    [0; 2],
                    [100; 2],
                    1,
                    5,
                    [2500; 2],
                    [5000; 2],
                );

                assert!(solution.value() >= optimal_solution.value() * 0.5);
            }
            }
        }
    }
}

#[cfg(all(
    feature = "highs",
))]
selective_tests! {
    fn bounded_theoretical_greedy_test(bounded_solvers::TheoreticalGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::TheoreticalGreedy),
            default_multi_constraint!(f64, bounded_solvers::TheoreticalGreedy),
        }
        IGNORE: {}
        CUSTOM: {
            //lots of large tests
            {
            for _ in 0..100 {
                let (solution, optimal_solution) =
                generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 1>
                (
                    bounded_solvers::TheoreticalGreedy,
                    bounded_solvers::HiGHS,
                    2000,
                    1,
                    0.0,
                    100.0,
                    [0; 1],
                    [100; 1],
                    1,
                    10,
                    [10000; 1],
                    [20000; 1],
                );

                assert!(solution.value() >= optimal_solution.value() * 0.5);
            }
            }

            //large test, 2 dimensions
            {
            let (solution, optimal_solution) =
            generic_data::random_test::bounded_random_test_from_u32::<_, _, _, _, 2>
            (
                bounded_solvers::TheoreticalGreedy,
                bounded_solvers::HiGHS,
                2000,
                1,
                0.0,
                100.0,
                [0; 2],
                [100; 2],
                1,
                10,
                [10000; 2],
                [20000; 2],
            );

            assert!(solution.value() >= optimal_solution.value() * 0.5);
            }
        }
    }
}

#[cfg(feature = "cbc")]
selective_tests! {
    fn bounded_cbc_test(bounded_solvers::CBC, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::CBC),
            default_multi_constraint!(f64, bounded_solvers::CBC),
            default_multi_knapsack!(f64, bounded_solvers::CBC),
        }
        IGNORE: {
            Problems::Bounded1Tuple(
            generic_data::default_multi_knapsack::random_sample_1::<f64, bounded_solvers::CBC>), //overflows
        }
        CUSTOM: {}
    }
}

#[cfg(feature = "highs")]
selective_tests! {
    fn bounded_highs_test(bounded_solvers::HiGHS, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::HiGHS),
            default_multi_constraint!(f64, bounded_solvers::HiGHS),
            default_multi_knapsack!(f64, bounded_solvers::HiGHS),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

#[cfg(feature = "cplex")]
selective_tests! {
    fn bounded_cplex_test(bounded_solvers::CPLEX, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::CPLEX),
            default_multi_constraint!(f64, bounded_solvers::CPLEX),
            default_multi_knapsack!(f64, bounded_solvers::CPLEX),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}
