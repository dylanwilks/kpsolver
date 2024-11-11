use kpsolver::{
    Knapsack, ProblemKnapsacks,
    bounded_solvers,
    items, knapsacks,
};

#[allow(dead_code)]
#[test]
fn random_sample_1() {
    items! {
        items<f64, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1.0],          2.0;
    }

    knapsacks! {
        knapsack<f64, 1>:
            [1.0];
    }

    let solution = items.clone().insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1.0]);
    knapsacks! {
        knapsack_mut<f64, 1>:
            [1.0];
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(bounded_solvers::TheoreticalGreedy);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1.0);
    assert_eq!(solution_mut[0].weights(), &[1.0]);
}

#[allow(dead_code)]
#[test]
fn random_sample_2() {
    items! {
        items<f64, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            55.0,       [95.0];
            10.0,       [4.0];
            47.0,       [60.0];
            5.0,        [32.0];
            4.0,        [23.0];
            50.0,       [72.0];
            8.0,        [80.0];
            61.0,       [62.0];
            85.0,       [65.0];
            87.0,       [46.0];
    }

    knapsacks! {
        knapsack<f64, 1>:
            [295.0];
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert!(solution.value() >= 151.0);
}

#[allow(dead_code)]
#[test]
fn random_sample_3() {
    items! {
        items<f64, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [1.0];
            3.0,        [2.0];
            1.0,        [1.0];
    }

    knapsacks! {
        knapsack<f64, 1>:
            [2.0];
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert!(solution.value() >= 1.5);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_1() {
    items! {
        items<f64, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1.0, 1.0],   2.0;
    }

    knapsacks! {
        knapsack<f64, 2>:
            [1.0, 1.0];
    }

    let solution = items.clone().insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1.0, 1.0]);
    knapsacks! {
        knapsack_mut<f64, 2>:
            [1.0, 1.0];
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(bounded_solvers::TheoreticalGreedy);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1.0);
    assert_eq!(solution_mut[0].weights(), &[1.0, 1.0]);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_2() {
    items! {
        items<f64, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            55.0,       [95.0, 34.0];
            10.0,       [4.0, 64.0];
            47.0,       [60.0, 13.0];
            5.0,        [32.0, 35.0];
            4.0,        [23.0, 9.0];
            50.0,       [72.0, 87.0];
            8.0,        [80.0, 35.0];
            61.0,       [62.0, 12.0];
            85.0,       [65.0, 54.0];
            87.0,       [46.0, 92.0];
    }

    knapsacks! {
        knapsack<f64, 2>:
            [269.0, 175.0];
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert!(solution.value() >= 140.0);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_3() {
    items! {
        items<f64, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [2.0, 2.0],       70.0;
            5.0,        [5.0, 2.0],       70.0;
            10.0,       [10.0, 2.0],      70.0;
    }

    knapsacks! {
        knapsack<f64, 2>:
            [100.0, 70.0];
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::TheoreticalGreedy);
    assert!(solution.value() >=  50.0);
}
