#[allow(unused_imports)]
use kpsolver::{bounded_solvers, binary_solvers};
use generic_data::Problems;

#[macro_use]
mod generic_data;

selective_tests! {
    fn binary_dynamic_test(binary_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple_binary!(u32, binary_solvers::Dynamic),
            default_multi_constraint_binary!(u32, binary_solvers::Dynamic),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

selective_tests! {
    fn binary_generalized_greedy_test(binary_solvers::GeneralizedGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::GeneralizedGreedy),
            default_multi_constraint_binary!(f64, binary_solvers::GeneralizedGreedy),
            default_multi_knapsack_binary!(f64, binary_solvers::GeneralizedGreedy),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

selective_tests! {
    fn binary_theoretical_greedy_test(binary_solvers::TheoreticalGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple_binary!(f64, binary_solvers::TheoreticalGreedy),
            default_multi_constraint_binary!(f64, binary_solvers::TheoreticalGreedy),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

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
