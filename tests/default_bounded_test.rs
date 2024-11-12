use kpsolver::{bounded_solvers, binary_solvers};
use generic_data::Problems;

#[macro_use]
mod generic_data;

selective_tests! {
    fn bounded_dynamic_test(bounded_solvers::Dynamic, <f64 as PartialEq>::eq, 1.0) {
        DEFAULT: {
            default_simple!(u32, bounded_solvers::Dynamic),
            default_multi_constraint!(u32, bounded_solvers::Dynamic),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

selective_tests! {
    fn bounded_generalized_greedy_test(bounded_solvers::GeneralizedGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::GeneralizedGreedy),
            default_multi_constraint!(f64, bounded_solvers::GeneralizedGreedy),
            default_multi_knapsack!(f64, bounded_solvers::GeneralizedGreedy),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

selective_tests! {
    fn bounded_theoretical_greedy_test(bounded_solvers::TheoreticalGreedy, <f64 as PartialOrd>::ge, 0.5) {
        DEFAULT: {
            default_simple!(f64, bounded_solvers::TheoreticalGreedy),
            default_multi_constraint!(f64, bounded_solvers::TheoreticalGreedy),
        }
        IGNORE: {}
        CUSTOM: {}
    }
}

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
