use ndarray::ArrayD;
use ndarray::IxDyn;

use crate::knapsack::ItemBound;
use crate::knapsack::Knapsack;

pub fn dynamic<const S: usize>(items: &mut [&mut ItemBound<usize, S>],
                               knapsack: &mut Knapsack<usize, S>) {
    //find and create the dimensions of the memo matrix
    let mut dim: Vec<usize> = (0..=S).collect();
    for item in items.iter() {
        dim[0] += item.quantity;
    }

    dim[0] += 1;
    let mut capacity = vec![0; S];
    capacity.clone_from_slice(knapsack.capacity.as_slice());
    let weight = knapsack.read_weights();
    for i in 0..S {
        if weight[i] > capacity[i] {
            capacity[i] = 0;
        } else {
            capacity[i] -= weight[i];
        }
    }

    for (i, cap) in capacity.iter().enumerate() {
        dim[i+1] = cap + 1;
    }

    let mut memo = ArrayD::<f64>::zeros(IxDyn(&dim));

    //iterate over each item before iterating over the capacity (cascadingly).
    //allocate vectors before loop
    let mut index = vec![1; dim.len()];
    let mut prev_index = vec![1; dim.len()];
    let mut ref_index = vec![0; dim.len()];
    loop {
        prev_index[0] -= 1;
        for item in items.iter() {
            for _ in 0..item.quantity {
                let mut excess_weight: bool = false;
                //find ref_index by decreasing corresponding elements of index with item weights
                for i in 1..=S {
                    if item.data.weights[i-1] > index[i] {
                        excess_weight = true;
                        break;
                    } else {
                        ref_index[i] = index[i] - item.data.weights[i-1];
                    }
                }
                
                //excess_weight similar to w_1 > c_1 V w_2 > c_2 V ... lazily evaluated
                if excess_weight {
                    memo[IxDyn(&index)] = memo[IxDyn(&prev_index)];
                } else {
                    let prev_value = memo[IxDyn(&prev_index)];
                    let ref_value = memo[IxDyn(&ref_index)] + item.data.value;
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
                index[i] = 1;
                index[i+1] += 1;
                prev_index[i] = 1;
                prev_index[i+1] += 1;
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
    for (i, item) in items.into_iter().enumerate().rev() {
        for _ in 0..item.quantity {
            index[0] -= 1;
            if current_val != memo[IxDyn(&index)] {
                item_quantity[i] += 1;
                for j in 1..=S {
                    index[j] -= item.data.weights[j - 1];
                }
            }

            current_val = memo[IxDyn(&index)];
        }
    }

    for (i, quantity) in item_quantity.iter().enumerate() {
        knapsack.add_item(ItemBound::<usize, S> {
                data: items[i].data.clone(),
                quantity: *quantity,
            }
        );

        items[i].quantity -= quantity;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dynamic_insert_into_knapsack() {
        let mut item1 = ItemBound::<usize, 1>::new(1.0, 2, [1]);
        let mut items = [&mut item1];
        let mut knapsack = Knapsack::<usize, 1>::new([2]);
        dynamic(&mut items, &mut knapsack);
        let knapsack_items = knapsack.read_items();
        assert_eq!(items[0].quantity, 0);
        items[0].quantity = 2;
        let item_data = item1.data.clone();
        assert_eq!(knapsack_items.get(&item_data.to_key()), Some(&item1));
        assert_eq!(knapsack.read_value(), 2.0);
        assert_eq!(knapsack.read_weights(), &[2_usize]);
    }

    #[test]
    fn dynamic_test_1() {
        let mut item1 = ItemBound::<usize, 1>::new(1.0, 2, [1]);
        let mut items = [&mut item1];
        let mut knapsack = Knapsack::<usize, 1>::new([1]);
        dynamic(&mut items, &mut knapsack);
        assert_eq!(item1.quantity, 1);
        assert_eq!(knapsack.read_value(), 1.0);
        assert_eq!(knapsack.read_weights(), &[1_usize]);
    }

    #[test]
    fn dynamic_test_2() {
        let mut item1 = ItemBound::<usize, 2>::new(1.0, 70, [2, 2]);
        let mut item2 = ItemBound::<usize, 2>::new(2.5, 70, [2, 5]);
        let mut item3 = ItemBound::<usize, 2>::new(5.0, 70, [2, 10]);
        let mut items = [&mut item1, &mut item2, &mut item3];
        let mut knapsack = Knapsack::<usize, 2>::new([70, 100]);
        dynamic(&mut items, &mut knapsack);
        let knapsack_items = knapsack.read_items();
        if let Some(x) = knapsack_items.get(&item1.data.to_key()) {
            assert_eq!(x.quantity, 25);
        }

        if let Some(x) = knapsack_items.get(&item2.data.to_key()) {
            assert_eq!(x.quantity, 10);
        }

        if let Some(x) = knapsack_items.get(&item3.data.to_key()) {
            assert_eq!(x.quantity, 0);
        }
    }
}
