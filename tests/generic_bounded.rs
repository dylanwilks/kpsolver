use kpsolver::bounded_solvers;

#[macro_use]
mod generic_data;

selective_tests! {
    fn dynamic_test(bounded_solvers::Dynamic, <f64 as PartialOrd>::gt, 0.5) {
        DEFAULT: {
            //default_simple!(u32, bounded_solvers::Dynamic),
            default_multi_constraint!(u32, bounded_solvers::Dynamic),
        }

        IGNORE: {
            //BoundedProblems::<u32, bounded_solvers::Dynamic>::Bounded1Tuple(
            //generic_data::default_multi_constraint::random_sample_2::<u32, bounded_solvers::Dynamic>),
        }
        CUSTOM: {}
    }
}
