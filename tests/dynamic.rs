use kpsolver::{
    Knapsack, ProblemKnapsacks,
    bounded_solvers,
    items, knapsacks,
};

#[allow(dead_code)]
#[test]
fn random_sample_1() {
    items! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1],          2;
    }

    knapsacks! {
        knapsack<u32, 1>:
            [1]
    }

    let solution = items.clone().insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1_u32]);
    knapsacks! {
        knapsack_mut<u32, 1>:
            [1]
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(bounded_solvers::Dynamic);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1);
    assert_eq!(solution_mut[0].weights(), &[1_u32]);
}

#[allow(dead_code)]
#[test]
fn random_sample_2() {
    items! {
        items<u32, 1>:
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

    let solution = items.insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 302.0);
}

#[allow(dead_code)]
#[test]
fn random_sample_3() {
    items! {
        items<u32, 1>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [1];
            3.0,        [2];
            1.0,        [1];
    }

    knapsacks! {
        knapsack<u32, 1>:
            [2]
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 3.0);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_1() {
    items! {
        items<u32, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            1.0,        [1, 1],       2;
    }

    knapsacks! {
        knapsack<u32, 2>:
            [1, 1]
    }

    let solution = items.clone().insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 1.0);
    assert_eq!(solution[0].weights(), &[1_u32, 1_u32]);
    knapsacks! {
        knapsack_mut<u32, 2>:
            [1, 1]
    }

    let solution_mut = items.insert_mut_into(knapsack_mut).using(bounded_solvers::Dynamic);
    assert_eq!(solution_mut.value(), 1.0);
    assert_eq!(items[0].quantity, 1);
    assert_eq!(solution_mut[0].weights(), &[1_u32, 1_u32]);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_2() {
    items! {
        items<u32, 2>:
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

    let solution = items.insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 280.0);
}

#[allow(dead_code)]
#[test]
fn random_sample_multi_constraint_3() {
    items! {
        items<u32, 2>:
            /* Value */ /* Weights */ /* Quantity (?) */
            2.0,        [2, 2],       70;
            5.0,        [5, 2],       70;
            10.0,       [10, 2],      70;
    }

    knapsacks! {
        knapsack<u32, 2>:
            [100, 70]
    }

    let solution = items.insert_into(knapsack).using(bounded_solvers::Dynamic);
    assert_eq!(solution.value(), 100.0);
}
