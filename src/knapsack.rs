use crate::compatible_problem_type_trait::{
    CompatibleProblemType, UnboundCompatibility
};
use crate::item::Item;

use indexmap::IndexMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Knapsack<T, const S: usize>
where
    T: CompatibleProblemType,
{
    value: f64,
    items: IndexMap<(u64, [u64; S]), Item<T, S>>,
    weights: [T; S],
    pub capacity: [T; S], 
}

impl<T, const S: usize> Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn new(capacity: [T; S]) -> Self {
        Self {
            value: 0.0,
            items: IndexMap::new(),
            weights: [<T as Default>::default(); S],
            capacity: capacity,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn weights(&self) -> &[T; S] {
        &self.weights
    }

    pub fn get_item(&self, key: (f64, [T; S])) -> Option<&Item<T, S>> {
        self.items.get(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn get_item_mut(&mut self, key: (f64, [T; S])) -> Option<&mut Item<T, S>> {
        self.items.get_mut(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn get_index_of(&self, key: (f64, [T; S])) -> Option<usize> {
        self.items.get_index_of(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn add(&mut self, item: Item<T, S>) -> bool {
        for r in 0..S {
            if item.weights[r] * item.quantity + self.weights[r] > 
            self.capacity[r] {
                return false;
            }
        }

        self.value += item.value * item.quantity.into();
        for r in 0..S {
            self.weights[r] += item.weights[r] * item.quantity;
        }

        if let Some(stored_item) = self.items.get_mut(&item.to_key()) {
            stored_item.quantity += item.quantity;
        } else {
            self.items.insert(item.to_key(), item);
        }

        return true;
    }

    pub fn add_mut<R>(&mut self, item: &mut Item<T, S, R>, quantity: T)
    -> bool where
        R: UnboundCompatibility + PartialOrd<T> + std::ops::SubAssign<T>,
    {
        if item.quantity < quantity {
            return false;
        }

        for r in 0..S {
            if item.weights[r] * quantity + self.weights[r] > 
            self.capacity[r] {
                return false;
            }
        }

        self.value += item.value * <T as Into<f64>>::into(quantity);
        item.quantity -= quantity;
        for r in 0..S {
            self.weights[r] += item.weights[r] * quantity;
        }

        if let Some(stored_item) = self.items.get_mut(&item.to_key()) {
            stored_item.quantity += quantity;
        } else {
            self.items.insert(
                item.to_key(), 
                Item::<T, S> {
                    value: item.value,
                    weights: item.weights,
                    quantity: quantity,
                }
            );
        }

        return true;
    }

    pub fn take(&mut self, item: Item<T, S>) -> Option<Item<T, S>> {
        if let Some(stored_item) = self.items.get_mut(&item.to_key()) {
            match stored_item
                  .quantity
                  .partial_cmp(&item.quantity)
                  .unwrap() {
                Ordering::Less => {
                    return None;
                },

                Ordering::Greater | Ordering::Equal => {
                    stored_item.quantity -= item.quantity;
                },
            }

            self.value -= item.value * item.quantity.into();
            for r in 0..S {
                self.weights[r] -= item.weights[r] * item.quantity;
            }

            return Some(item);
        } else {
            return None;
        }
    }

    pub fn remove_item(&mut self, key: (f64, [T; S])) -> Option<Item<T, S>> {
        self.items.shift_remove(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn remove_index(&mut self, index: usize) -> Option<Item<T, S>> {
        match self.items.shift_remove_index(index) {
            Some((_, item)) => Some(item),
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.value = 0.0;
        for r in 0..S {
            self.weights[r] = T::default();
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn into_iter(self) -> indexmap::map::IntoValues<(u64, [u64; S]), Item<T, S>> {
        self.items.into_values()
    }

    pub fn iter<'a>(&'a self) -> indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S>> {
        self.items.values()
    }
}

impl<T, const S: usize> IntoIterator for Knapsack<T, S>
where
    T: CompatibleProblemType,
{
    type Item = Item<T, S>;
    type IntoIter = indexmap::map::IntoValues<(u64, [u64; S]), Item<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_values()
    }
}

impl<'a, T, const S: usize> IntoIterator for &'a Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    type Item = <indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S>> as Iterator>::Item;
    type IntoIter = indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}

impl<T, const S: usize> std::ops::Index<usize> for Knapsack<T, S>
where
    T: CompatibleProblemType,
{
    type Output = Item<T, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[derive(Default, Clone)]
pub struct ProblemKnapsacks<T, const S: usize>
where
    T: CompatibleProblemType,
{
    knapsacks: Vec<Knapsack<T, S>>,
}

impl<T, const S: usize> ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    pub fn new() -> Self {
        ProblemKnapsacks::<T, S> {
            knapsacks: Vec::new(),
        }
    }

    pub fn add(&mut self, knapsack: Knapsack<T, S>) {
        self.knapsacks.push(knapsack);
    }

    pub fn value(&self) -> f64 {
        let mut value = 0.0;
        for knapsack in &self.knapsacks {
            value += knapsack.value();
        }

        value
    }

    pub fn len(&self) -> usize {
        self.knapsacks.len()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Knapsack<T, S>> {
        self.knapsacks.into_iter()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Knapsack<T, S>> {
        self.knapsacks.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) 
    -> std::slice::IterMut<'a, Knapsack<T, S>> {
        self.knapsacks.iter_mut()
    }
}

impl<T, const S: usize> IntoIterator for ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    type Item = Knapsack<T, S>;
    type IntoIter = std::vec::IntoIter<Knapsack<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.knapsacks.into_iter()
    }
}

impl<'a, T, const S: usize> IntoIterator for &'a ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    type Item = <std::slice::Iter<'a, Knapsack<T, S>> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, Knapsack<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.knapsacks.as_slice().into_iter()
    }
}

impl<'a, T, const S: usize> IntoIterator 
for &'a mut ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    type Item = <std::slice::IterMut<'a, Knapsack<T, S>> as Iterator>::Item;
    type IntoIter = std::slice::IterMut<'a, Knapsack<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.knapsacks.as_mut_slice().into_iter()
    }
}

impl<T, const S: usize> std::ops::Index<usize> for ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    type Output = Knapsack<T, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.knapsacks[index]
    }
}

impl<T, const S: usize> std::ops::IndexMut<usize> 
for ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.knapsacks[index]
    }
}
