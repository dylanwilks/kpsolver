use kpsolver::compatible_problem_type_trait::CompatibleProblemType;
use kpsolver::{BinaryProblemKnapsacks, ProblemKnapsacks};

#[macro_use]
pub mod default_simple;
#[macro_use]
pub mod default_multi_constraint;
#[macro_use]
pub mod default_multi_knapsack;
pub mod random_test;

#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum Problems<T, S>
where
    T: CompatibleProblemType,
{
    Bounded1Tuple(fn(S) -> ProblemKnapsacks<T, 1>),
    Bounded2Tuple(fn(S) -> ProblemKnapsacks<T, 2>),
    Binary1Tuple(fn(S) -> BinaryProblemKnapsacks<T, 1>),
    Binary2Tuple(fn(S) -> BinaryProblemKnapsacks<T, 2>),
}

macro_rules! selective_tests {
    (
    fn $test_name:ident($solver:path, $cmp:expr, $scale:literal) {
        DEFAULT: { $($default_arr:expr),* $(,)? }
        IGNORE: { $($ignore_fn:expr),* $(,)? }
        CUSTOM: { $($custom_call:tt)* }
    }
    ) => {
        #[test]
        fn $test_name() {
            let ignore_arr = [$($ignore_fn),*];

            $(
            let default_arr = $default_arr;
            'default_iter: for default_tuple in default_arr {
                for ignore_fn in &ignore_arr {
                    if default_tuple.0 == *ignore_fn {
                        continue 'default_iter;
                    }
                }

                match (default_tuple.0, default_tuple.1) {
                    ($crate::generic_data::Problems::Bounded1Tuple(f), v) => {
                        let solution = f($solver);
                        assert!($cmp(&solution.value(), &(v * $scale)),
                        "ERROR::COMPARISON_FAILED;
                        SOLVER_NAME: {:?}
                        LHS_VALUE: {}
                        RHS_VALUE: {}",
                        $solver, solution.value(), v * $scale);
                    }
                    ($crate::generic_data::Problems::Bounded2Tuple(f), v) => {
                        let solution = f($solver);
                        assert!($cmp(&solution.value(), &(v * $scale)),
                        "ERROR::COMPARISON_FAILED;
                        SOLVER_NAME: {:?}
                        LHS_VALUE: {}
                        RHS_VALUE: {}",
                        $solver, solution.value(), v * $scale);
                    }
                    ($crate::generic_data::Problems::Binary1Tuple(f), v) => {
                        let solution = f($solver);
                        assert!($cmp(&solution.value(), &(v * $scale)),
                        "ERROR::COMPARISON_FAILED;
                        SOLVER_NAME: {:?}
                        LHS_VALUE: {}
                        RHS_VALUE: {}",
                        $solver, solution.value(), v * $scale);
                    }
                    ($crate::generic_data::Problems::Binary2Tuple(f), v) => {
                        let solution = f($solver);
                        assert!($cmp(&solution.value(), &(v * $scale)),
                        "ERROR::COMPARISON_FAILED;
                        SOLVER_NAME: {:?}
                        LHS_VALUE: {}
                        RHS_VALUE: {}",
                        $solver, solution.value(), v * $scale);
                    }
                }
            }
            )*

            $($custom_call)*
        }
    };
}
