use generic_data::Problems;
#[allow(unused_imports)]
use kpsolver::{binary_solvers, bounded_solvers};

#[macro_use]
mod generic_data;

#[cfg(all(feature = "cplex", feature = "highs",))]
selective_tests! {
    fn binary_dynamic_test(binary_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple_binary!(u32, binary_solvers::Dynamic),
            default_multi_constraint_binary!(u32, binary_solvers::Dynamic),
        }
        IGNORE: {}
        CUSTOM: {
            for _ in 0..1000 {
                {
                let (solution, optimal_solution) =
                generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 1>
                (
                    binary_solvers::Dynamic,
                    binary_solvers::CPLEX,
                    100,
                    1,
                    0.0,
                    100.0,
                    [0; 1],
                    [100; 1],
                    [1000; 1],
                    [2000; 1],
                );

                assert_eq!(solution.value(), optimal_solution.value());
                }
            }

            {
            let (solution, optimal_solution) =
            generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 2>
            (
                binary_solvers::Dynamic,
                binary_solvers::HiGHS,
                100,
                1,
                0.0,
                100.0,
                [0; 2],
                [100; 2],
                [1000; 2],
                [2000; 2],
            );

            assert_eq!(solution.value(), optimal_solution.value());
            }

            {
            let (solution, optimal_solution) =
            generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 3>
            (
                binary_solvers::Dynamic,
                binary_solvers::CPLEX,
                10,
                1,
                0.0,
                100.0,
                [0; 3],
                [100; 3],
                [100; 3],
                [200; 3],
            );

            assert_eq!(solution.value(), optimal_solution.value());
            }
        }
    }
}

#[cfg(all(feature = "cplex",))]
selective_tests! {
    fn binary_generalized_greedy_test(binary_solvers::GeneralizedGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::GeneralizedGreedy),
            default_multi_constraint_binary!(f64, binary_solvers::GeneralizedGreedy),
            default_multi_knapsack_binary!(f64, binary_solvers::GeneralizedGreedy),
        }
        IGNORE: {}
        CUSTOM: {
            {
            for _ in 0..100 {
                let (solution, optimal_solution) =
                generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 2>
                (
                    binary_solvers::GeneralizedGreedy,
                    binary_solvers::CPLEX,
                    100,
                    2,
                    0.0,
                    100.0,
                    [0; 2],
                    [100; 2],
                    [1000; 2],
                    [2000; 2],
                );

                assert!(solution.value() >= optimal_solution.value() * 0.5);
            }
            }
        }
    }
}

#[cfg(all(feature = "highs",))]
selective_tests! {
    fn binary_theoretical_greedy_test(binary_solvers::TheoreticalGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::TheoreticalGreedy),
            default_multi_constraint_binary!(f64, binary_solvers::TheoreticalGreedy),
        }
        IGNORE: {}
        CUSTOM: {
            {
            for _ in 0..100 {
                let (solution, optimal_solution) =
                generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 1>
                (
                    binary_solvers::TheoreticalGreedy,
                    binary_solvers::HiGHS,
                    2000,
                    1,
                    0.0,
                    100.0,
                    [0; 1],
                    [100; 1],
                    [5000; 1],
                    [10000; 1],
                );

                assert!(solution.value() >= optimal_solution.value() * 0.5);
                }

                {
                let (solution, optimal_solution) =
                generic_data::random_test::binary_random_test_from_u32::<_, _, _, _, 2>
                (
                    binary_solvers::TheoreticalGreedy,
                    binary_solvers::HiGHS,
                    2000,
                    1,
                    0.0,
                    100.0,
                    [0; 2],
                    [100; 2],
                    [5000; 2],
                    [10000; 2],
                );

                assert!(solution.value() >= optimal_solution.value() * 0.5);
                }
            }
        }
    }
}

#[cfg(feature = "cbc")]
selective_tests! {
    fn binary_cbc_test(binary_solvers::CBC, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::CBC),
            default_multi_constraint_binary!(f64, binary_solvers::CBC),
            default_multi_knapsack_binary!(f64, binary_solvers::CBC),
        }
        IGNORE: {
            Problems::Binary1Tuple(
            generic_data::default_multi_knapsack::random_sample_1::<f64, binary_solvers::CBC>), //overflows
        }
        CUSTOM: {}
    }
}

#[cfg(feature = "highs")]
selective_tests! {
    fn binary_highs_test(binary_solvers::HiGHS, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::HiGHS),
            default_multi_constraint_binary!(f64, binary_solvers::HiGHS),
            default_multi_knapsack_binary!(f64, binary_solvers::HiGHS),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

#[cfg(feature = "cplex")]
selective_tests! {
    fn binary_cplex_test(binary_solvers::CPLEX, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::CPLEX),
            default_multi_constraint_binary!(f64, binary_solvers::CPLEX),
            default_multi_knapsack_binary!(f64, binary_solvers::CPLEX),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}
