use crate::item::Item;
use crate::knapsack::BinaryProblemKnapsacks;
use crate::problem_type::{BinaryProblem, BinarySolver};

struct ItemInfo {
    pub j: usize, //index
    pub e: f64, //efficiency
    pub x: bool, //decision variable
}

struct KnapsackInfo {
    pub j: usize, //index
    pub s: f64, //score
}

//algorithm implemented for types that can be cast into f64 (efficiency calculation)
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GeneralizedGreedy;
impl<const S: usize> BinarySolver<f64, S> for GeneralizedGreedy {
    type Output = BinaryProblemKnapsacks<f64, S>;

    fn solve(self, problem: BinaryProblem<f64, S>) 
    -> BinaryProblemKnapsacks<f64, S> {
        let items = problem.items;
        let mut knapsacks = problem.knapsacks;
        let mut knapsack_order: Vec<KnapsackInfo> 
            = Vec::with_capacity(knapsacks.len());
        let mut capacity_d_sum: [f64; S] = [0.0; S];
        let mut weight_d_sum: [f64; S] = [0.0; S];
        let mut d_rating: [f64; S] = [1.0; S];

        //calculate sum of weights and capacities of corresponding dim.
        let mut largest_neg = 0.0;
        let mut d_diff: [f64; S] = [0.0; S];
        for r in 0..S {
            for item in items.iter() {
                weight_d_sum[r] += item.weights[r];
            }
            
            for knapsack in knapsacks.iter() {
                capacity_d_sum[r] += knapsack.capacity[r] 
                                   - knapsack.weights()[r];
            }

            d_diff[r] = weight_d_sum[r] - capacity_d_sum[r];
            d_rating[r] += weight_d_sum[r] / capacity_d_sum[r];
            if d_diff[r] < 0.0 && d_diff[r] < largest_neg {
                largest_neg = d_diff[r];
            }
        }

        //calculate senju-toyoda efficiency value first.
        //items with same value and weights will have the same rating.
        let mut items_toyoda: Vec<ItemInfo> = Vec::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            items_toyoda.push(
                ItemInfo {
                    j: i,
                    e: { 
                        //denominator from (9.35)
                        let mut sum = 0.0;
                        for r in 0..S {
                            sum += item.weights[r] * (d_diff[r] - largest_neg);
                        }

                        item.value/sum
                    },

                    x: false,
                }
            );
        }

        //create array of knapsack data
        for (i, knapsack) in knapsacks.iter().enumerate() {
            knapsack_order.push(
                KnapsackInfo {
                    j: i,
                    s: {
                        let mut score = 0.0;
                        for r in 0..S {
                            score += (knapsack.capacity[r] - 
                                      knapsack.weights()[r]) * d_rating[r];
                        }

                        score
                    },
                }
            );
        }

        //sort item_order and knapsack_order by increasing efficiency and score respectively
        items_toyoda.sort_by(|x, y| x.e.partial_cmp(&y.e).unwrap());
        knapsack_order.sort_by(|x, y| x.s.partial_cmp(&y.s).unwrap());

        //pop items_toyoda and iterate over each item in the set separately.
        //if its weight exceeds current capacity alone all remaining items in the set
        //do not as well, so insert remaining items into excess_stack.
        //each time there is a split push excess_stack onto items_toyoda.
        let mut items_loulou: Vec<ItemInfo> = Vec::with_capacity(items.len());
        let mut excess_stack: Vec<ItemInfo> = Vec::new();
        let mut k_i = 0; //knapsack_order index
        let mut k_w = [0.0; S]; //weight of current knapsack
        let mut value = 0.0; //value of items that fit
        let mut split_value = 0.0; //value of split items
        'item_set: while let Some(item_info) = items_toyoda.pop() {
            for r in 0..S {
                let new_d_weight = k_w[r] + items[item_info.j].weights[r];
                if new_d_weight > 
                knapsacks[knapsack_order[k_i].j].capacity[r] - 
                knapsacks[knapsack_order[k_i].j].weights()[r] { //item does not fit
                    for rf in r..S {
                        if items[item_info.j].weights[rf] > 
                        knapsacks[knapsack_order[k_i].j]
                            .capacity[rf] - 
                        knapsacks[knapsack_order[k_i].j]
                            .weights()[rf] { //item alone exceeds capacity
                            excess_stack.push(ItemInfo {
                                j: item_info.j,
                                e: 0.0,
                                x: false,
                            });

                            continue 'item_set;
                        }
                    }

                    //item is split item
                    split_value += items[item_info.j].value;
                    for rs in 0..S {
                        k_w[rs] = 0.0;
                    }

                    items_toyoda.push(ItemInfo {
                        j: item_info.j,
                        e: 0.0,
                        x: false,
                    });

                    while let Some(excess_item) = excess_stack.pop() {
                        items_toyoda.push(excess_item);
                    }

                    items_loulou.push(ItemInfo {
                        j: item_info.j,
                        e: 0.0,
                        x: false,
                    });

                    k_i += 1;
                    if k_i == knapsacks.len() {
                        break 'item_set;
                    }

                    continue 'item_set;
                }
            }

            //item fits normally
            items_loulou.push(ItemInfo {
                j: item_info.j,
                e: 0.0,
                x: true,
            });

            value += items[item_info.j].value;
            for r in 0..S {
                k_w[r] += items[item_info.j].weights[r];
            }
        }

        //if split items are worth more flip items_loulou x variable
        if split_value > value {
            for item_info in items_loulou.iter_mut() {
                if item_info.x {
                    item_info.x = false;
                } else {
                    item_info.x = true;
                }
            }
        }

        //now calculate Loulou-Michaelides efficiency values on items_loulou.
        let mut cumulative_weights = [0.0; S];
        d_rating = [1.0; S];
        'item: for item_info in items_loulou.iter_mut() {
            let mut v = 0.0; //penalty value
            for r in 0..S {
                if r == S - 1 {
                    item_info.e = f64::NEG_INFINITY;
                    break;
                }

                let q1 = cumulative_weights[r] + 
                         items[item_info.j].weights[r];
                let q2 = capacity_d_sum[r] - (cumulative_weights[r] +
                    items[item_info.j].weights[r]);
                if q2 <= 0.0 {
                    item_info.e = 0.0;
                    continue 'item;
                }

                weight_d_sum[r] -= items[item_info.j].weights[r];
                let q3 = weight_d_sum[r];
                let v_t = q1 * q3.sqrt() / (capacity_d_sum[r] * q2.sqrt());
                d_rating[r] += v_t;
                if item_info.x {
                    cumulative_weights[r] += items[item_info.j].weights[r];
                }

                if v_t > v {
                    v = v_t;
                }
            }
            
            item_info.e = items[item_info.j].value / v;
        }

        //rescore knapsack items
        for knapsack_info in knapsack_order.iter_mut() {
            let mut score = 0.0;
            for r in 0..S {
                score += knapsacks[knapsack_info.j].capacity[r] -
                         knapsacks[knapsack_info.j].weights()[r] * 
                         d_rating[r];
            }
            
            knapsack_info.s = score;
        }

        //sort items and knapsacks into decreasing efficiency and and increasing score respectively.
        items_loulou.sort_by(|x, y| y.e.partial_cmp(&x.e).unwrap());
        knapsack_order.sort_by(|x, y| x.s.partial_cmp(&y.s).unwrap());

        //check if split values are greater once again.
        value = 0.0;
        split_value = 0.0;
        k_i = 0;
        k_w = [0.0; S];
        'item: for item_info in items_loulou.iter_mut() {
            for r in 0..S {
                if items[item_info.j].weights[r] + k_w[r] > 
                knapsacks[knapsack_order[k_i].j].capacity[r] -
                knapsacks[knapsack_order[k_i].j].weights()[r] { //item is split item
                    item_info.x = false;
                    split_value += items[item_info.j].value;
                    k_w = [0.0; S];
                    k_i += 1;
                    if k_i == knapsacks.len() {
                        break 'item;
                    }

                    continue 'item;
                }
            }

            //item fits
            item_info.x = true;
            value += items[item_info.j].value;
            for r in 0..S {
                k_w[r] += items[item_info.j].weights[r];
            }
        }

        //now insert items_loulou objects into the knapsack.
        k_i = 0;
        'item: for item_info in items_loulou {
            if value > split_value {
                if item_info.x {
                    while !knapsacks[knapsack_order[k_i].j]
                    .add(
                        Item::<f64, S> {
                            value: items[item_info.j].value,
                            weights: items[item_info.j].weights,
                            quantity: 1.0,
                        }
                    ) {
                        k_i += 1;

                        if k_i == knapsacks.len() {
                            break 'item;
                        }
                    }
                }
            } else {
                if !item_info.x {
                    knapsacks[knapsack_order[k_i].j].add(
                        Item::<f64, S> {
                            value: items[item_info.j].value,
                            weights: items[item_info.j].weights,
                            quantity: 1.0,
                        }
                    );
                    k_i += 1;
                }
            }
        }

        knapsacks
    }
}
