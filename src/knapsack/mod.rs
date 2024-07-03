use std::collections::HashMap;

pub use impl_itemtype::ItemType;
pub use impl_itemtype::ItemData;
use impl_itemtype::ItemDataKey;

pub use impl_unbound::unbound;
use impl_unbound::Quantity;

pub mod solver01;
//pub mod solver_bounded;
//pub mod solver_unbounded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item<T, N, const S: usize> 
where 
    T: ItemType<S>,
    N: Quantity,
{
    pub data: ItemData::<T, S>,
    pub quantity: N,
}

pub type ItemBound<T, const S: usize> = Item::<T, usize, S>;
pub type ItemUnbound<T, const S: usize> = Item::<T, unbound, S>;

#[derive(Debug)]
pub struct Knapsack<T, const S: usize> 
where
    T: ItemType<S>,
{
    value: f64,
    items: HashMap<ItemDataKey::<S>, Item::<T, usize, S>>,
    weights: [T; S],
    pub capacity: [T; S],
}

impl<T, const S: usize> Item<T, usize, S> 
where
    T: ItemType<S>,
{
    pub fn new(
        value: f64,
        quantity: usize,
        weights: [T; S],
    ) -> Self {
        Self {
            data: ItemData::<T, S> {
                value: value,
                weights: weights,
            },

            quantity: quantity,
        }
    }
}

impl<T, const S: usize> Item<T, unbound, S> 
where
    T: ItemType<S>,
{
    pub fn new(
        value: f64,
        weights: [T; S],
    ) -> Self {
        Self {
            data: ItemData::<T, S> {
                value: value,
                weights: weights,
            },

            quantity: unbound,
        }
    }
}

impl<T, const S: usize> Knapsack<T, S> 
where
    T: ItemType<S>,
{
    pub fn new(capacity: [T; S]) -> Self {
        Self {
            value: <f64 as Default>::default(),
            items: HashMap::new(),
            weights: [<T as Default>::default(); S],
            capacity: capacity,
        }
    }

    pub fn read_value(&self) -> f64 {
        self.value
    }

    pub fn read_items(&self) 
        -> &HashMap<ItemDataKey<S>, Item<T, usize, S>> {
        &self.items
    }

    pub fn read_weights(&self) -> &[T; S] {
        &self.weights
    }

    pub fn add_item(&mut self, item: Item<T, usize, S>) {
        let mut new_item: bool = false;

        if let Some(item_data) = self.items.get_mut(&item.data.to_key()) {
            item_data.quantity += item.quantity;
        } else {
            new_item = true;
        }

        for i in 0..S {
            self.weights[i] += item.data.weights[i].mul_usize(item.quantity);
        }

        self.value += item.data.value * item.quantity as f64;

        if new_item {
            self.items.insert(item.data.to_key(), item);
        }
    }

    pub fn remove_item(&mut self, item_data: &ItemData<T, S>) {
        self.items.remove(&item_data.to_key());
    }
}

mod impl_itemtype;
mod impl_unbound;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_bounded_usize_item_1() {
        let item = ItemBound::<usize, 2>::new(1.0, 1, [1, 1]);
        let mut knapsack = Knapsack::<usize, 2>::new([1, 1]);
        let item_data = item.data.clone();
        knapsack.add_item(item.clone());
        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Some(&item)); //check item correctly inserted using ItemData
        assert_eq!(knapsack.read_value(), 1.0); //check value of knapsack matches
        assert_eq!(knapsack.read_weights(), &[1_usize, 1_usize]); //check weights of knapsack matches
    }

    #[test]
    fn test_add_bounded_usize_item_2() {
        let item1 = ItemBound::<usize, 2>::new(1.0, 1, [1, 1]);
        let item2 = ItemBound::<usize, 2>::new(1.0, 2, [1, 1]);
        let item_test = ItemBound::<usize, 2>::new(1.0, 3, [1, 1]);
        let mut knapsack = Knapsack::<usize, 2>::new([5, 5]);
        knapsack.add_item(item1);
        knapsack.add_item(item2);
        let item_data = ItemData::<usize, 2> {
            value: 1.0,
            weights: [1, 1],
        };

        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Some(&item_test));
        assert_eq!(knapsack.read_value(), 3.0);
        assert_eq!(knapsack.read_weights(), &[3_usize, 3_usize]);
    }

    #[test]
    fn test_add_bounded_f64_item_1() {
        let item = ItemBound::<f64, 2>::new(1.2, 1, [1.5, 1.5]);
        let mut knapsack = Knapsack::<f64, 2>::new([5.5, 5.5]);
        let item_data = item.data.clone();
        knapsack.add_item(item.clone());
        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Some(&item)); //check item correctly inserted using ItemData
        assert_eq!(knapsack.read_value(), 1.2); //check value of knapsack matches
        assert_eq!(knapsack.read_weights(), &[1.5, 1.5]); //check weights of knapsack matches
    }

    #[test]
    fn test_add_bounded_f64_item_2() {
        let item1 = ItemBound::<f64, 2>::new(1.0, 1, [1.5, 1.5]);
        let item2 = ItemBound::<f64, 2>::new(1.0, 2, [1.5, 1.5]);
        let item_test = ItemBound::<f64, 2>::new(1.0, 3, [1.5, 1.5]);
        let mut knapsack = Knapsack::<f64, 2>::new([5.0, 5.0]);
        knapsack.add_item(item1);
        knapsack.add_item(item2);
        let item_data = ItemData::<f64, 2> {
            value: 1.0,
            weights: [1.5, 1.5],
        };

        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Some(&item_test));
        assert_eq!(knapsack.read_value(), 3.0);
        assert_eq!(knapsack.read_weights(), &[4.5, 4.5]);
    }

    #[test]
    fn test_remove_bounded_usize_item() {
        let item = ItemBound::<usize, 2>::new(1.0, 1, [1, 1]);
        let mut knapsack = Knapsack::<usize, 2>::new([1, 1]);
        let item_data = item.data.clone();
        knapsack.add_item(item.clone());
        knapsack.remove_item(&item_data);
        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Option::<_>::None);
    }

    #[test]
    fn test_remove_bounded_f64_item() {
        let item = ItemBound::<f64, 2>::new(1.0, 1, [1.0, 1.0]);
        let mut knapsack = Knapsack::<f64, 2>::new([1.0, 1.0]);
        let item_data = item.data.clone();
        knapsack.add_item(item.clone());
        knapsack.remove_item(&item_data);
        let items = knapsack.read_items();
        assert_eq!(items.get(&item_data.to_key()), Option::<_>::None);
    }
}
