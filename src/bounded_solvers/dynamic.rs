use crate::item::Item;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{BoundedProblem, BoundedSolver};
use ndarray::{ArrayD, IxDyn};

//algorithm implemented for types that can be cast into usize (indexing)
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Dynamic;
impl<const S: usize> BoundedSolver<u32, S> for Dynamic {
    type Output = ProblemKnapsacks<u32, S>;

    fn solve(self, mut problem: BoundedProblem<u32, S>) -> ProblemKnapsacks<u32, S> {
        //find and create the dimensions of the memo matrix
        let items = problem.items;
        let knapsack = &mut problem.knapsacks[0];

        let mut dim: Vec<usize> = (0..=S).collect();
        for item in items.iter() {
            dim[0] += usize::try_from(item.quantity).unwrap();
        }

        dim[0] += 1;
        let mut capacity = vec![0_usize; S];
        capacity.clone_from_slice(
            knapsack
                .capacity
                .map(|x| usize::try_from(x).unwrap())
                .as_slice(),
        );
        let weight = *knapsack.weights();
        for i in 0..S {
            if usize::try_from(weight[i]).unwrap() > capacity[i] {
                capacity[i] = 0;
            } else {
                capacity[i] -= weight[i] as usize;
            }
        }

        for (i, cap) in capacity.iter().enumerate() {
            dim[i + 1] = *cap + 1;
        }

        let mut memo = ArrayD::<f64>::zeros(IxDyn(&dim));

        //iterate over each item before iterating over the capacity (cascadingly).
        //allocate vectors before loop
        let mut index = vec![0_usize; dim.len()];
        let mut prev_index = vec![0_usize; dim.len()];
        let mut ref_index = vec![0_usize; dim.len()];
        loop {
            index[0] += 1;
            for item in items.iter() {
                for _ in 0..item.quantity {
                    let mut excess_weight: bool = false;
                    //find ref_index by decreasing corresponding elements of index with item weights
                    for i in 1..=S {
                        if usize::try_from(item.weights[i - 1]).unwrap() > index[i] {
                            excess_weight = true;
                            break;
                        } else {
                            ref_index[i] = index[i] - item.weights[i - 1] as usize;
                        }
                    }

                    //excess_weight similar to w_1 > c_1 V w_2 > c_2 V ... lazily evaluated
                    if excess_weight {
                        memo[IxDyn(&index)] = memo[IxDyn(&prev_index)];
                    } else {
                        let prev_value = memo[IxDyn(&prev_index)];
                        let ref_value = memo[IxDyn(&ref_index)] + item.value;
                        if ref_value > prev_value {
                            memo[IxDyn(&index)] = ref_value;
                        } else {
                            memo[IxDyn(&index)] = prev_value;
                        }
                    }

                    index[0] += 1;
                    prev_index[0] += 1;
                    ref_index[0] += 1;
                }
            }

            //if index has reached final memo spot (index[1..] == weights)
            if index[1..] == capacity {
                index[0] -= 1; //correct index array to match final spot in matrix
                break;
            }

            //if index[i] == dim[i], 'increment' index
            for i in 0..S {
                if index[i] == dim[i] {
                    index[i] = 0;
                    index[i + 1] += 1;
                    prev_index[i] = 0;
                    prev_index[i + 1] += 1;
                } else {
                    break;
                }
            }

            ref_index[0] = 0;
        }

        //now to backtrack the matrix. First get an array of the # of corresponding items then
        //generate item and insert into knapsack
        let mut item_quantity = vec![0_usize; items.len()];
        let mut current_val = memo[IxDyn(&index)];
        for (i, item) in items.iter().enumerate().rev() {
            for _ in 0..item.quantity {
                index[0] -= 1;
                if current_val != memo[IxDyn(&index)] {
                    item_quantity[i] += 1;
                    for j in 1..=S {
                        index[j] -= item.weights[j - 1] as usize;
                    }
                }

                current_val = memo[IxDyn(&index)];
            }
        }

        for (i, quantity) in item_quantity.iter().enumerate() {
            knapsack.add(Item::<u32, S> {
                value: items[i].value,
                weights: items[i].weights,
                quantity: u32::try_from(*quantity).unwrap(),
            });
        }

        problem.knapsacks
    }
}
