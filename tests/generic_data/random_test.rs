use kpsolver::compatible_problem_type_trait::CompatibleProblemType;
#[allow(unused_imports)]
use kpsolver::{
    items, items_binary, knapsacks, knapsacks_binary, BinaryItem, BinaryKnapsack, BinarySolver,
    BoundedSolver, Item, Knapsack,
};
use rand::Rng;

#[allow(dead_code)]
pub fn binary_random_test_from_u32<T1, T2, S1, S2, const N: usize>(
    solver: S1,
    optimal_solver: S2, //for comparison
    items: usize,
    knapsacks: usize,
    value_min: f64,
    value_max: f64,
    weights_min: [u32; N],
    weights_max: [u32; N],
    capacities_min: [u32; N],
    capacities_max: [u32; N],
) -> (
    <S1 as BinarySolver<T1, N>>::Output,
    <S2 as BinarySolver<T2, N>>::Output,
)
where
    T1: CompatibleProblemType + From<u32>,
    T2: CompatibleProblemType + From<u32>,
    S1: BinarySolver<T1, N>,
    S2: BinarySolver<T2, N>,
{
    let mut rng = rand::thread_rng();
    let mut problem_items = items_binary!(u32, N);
    for _ in 0..items {
        problem_items.add(BinaryItem::<u32, N>::binary(
            rng.gen_range((value_min as u32)..(value_max as u32)) as f64, //avoid rounding issues
            {
                let mut rand_arr: [u32; N] = [0; N];
                for r in 0..N {
                    rand_arr[r] = rng.gen_range(weights_min[r]..weights_max[r]);
                }

                rand_arr
            },
        ));
    }

    let mut problem_knapsacks = knapsacks_binary!(u32, N);
    for _ in 0..knapsacks {
        problem_knapsacks.add(BinaryKnapsack::<u32, N>::new({
            let mut rand_arr: [u32; N] = [0; N];
            for r in 0..N {
                rand_arr[r] = rng.gen_range(capacities_min[r]..capacities_max[r]);
            }

            rand_arr
        }));
    }

    let solution_1 = problem_items
        .clone()
        .to_generic::<T1>()
        .insert_into(problem_knapsacks.clone().to_generic::<T1>())
        .using(solver);

    let solution_2 = problem_items
        .to_generic::<T2>()
        .insert_into(problem_knapsacks.to_generic::<T2>())
        .using(optimal_solver);

    (solution_1, solution_2)
}

#[allow(dead_code)]
pub fn bounded_random_test_from_u32<T1, T2, S1, S2, const N: usize>(
    solver: S1,
    optimal_solver: S2, //for comparison
    items: usize,
    knapsacks: usize,
    value_min: f64,
    value_max: f64,
    weights_min: [u32; N],
    weights_max: [u32; N],
    quantity_min: u32,
    quantity_max: u32,
    capacities_min: [u32; N],
    capacities_max: [u32; N],
) -> (
    <S1 as BoundedSolver<T1, N>>::Output,
    <S2 as BoundedSolver<T2, N>>::Output,
)
where
    T1: CompatibleProblemType + From<u32>,
    T2: CompatibleProblemType + From<u32>,
    S1: BoundedSolver<T1, N>,
    S2: BoundedSolver<T2, N>,
{
    let mut rng = rand::thread_rng();
    let mut problem_items = items!(u32, N);
    for _ in 0..items {
        problem_items.add(Item::<u32, N>::new(
            rng.gen_range((value_min as u32)..(value_max as u32)) as f64,
            {
                let mut rand_arr: [u32; N] = [0; N];
                for r in 0..N {
                    rand_arr[r] = rng.gen_range(weights_min[r]..weights_max[r]);
                }

                rand_arr
            },
            rng.gen_range(quantity_min..quantity_max),
        ));
    }

    let mut problem_knapsacks = knapsacks!(u32, N);
    for _ in 0..knapsacks {
        problem_knapsacks.add(Knapsack::<u32, N>::new({
            let mut rand_arr: [u32; N] = [0; N];
            for r in 0..N {
                rand_arr[r] = rng.gen_range(capacities_min[r]..capacities_max[r]);
            }

            rand_arr
        }));
    }

    let solution_1 = problem_items
        .clone()
        .to_generic::<T1>()
        .insert_into(problem_knapsacks.clone().to_generic::<T1>())
        .using(solver);

    let solution_2 = problem_items
        .to_generic::<T2>()
        .insert_into(problem_knapsacks.to_generic::<T2>())
        .using(optimal_solver);

    (solution_1, solution_2)
}
