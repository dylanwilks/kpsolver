use std::collections::HashMap;
use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::item::ItemBound;

#[derive(Debug, Clone, PartialEq)]
pub struct Knapsack<T, const S: usize> 
where
    T: CompatibleProblemType,
{
    value: f64,
    items: HashMap<(u64, [u64; S]), usize>,
    weights: [T; S],
    pub capacity: [T; S],
}

impl<T, const S: usize> Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn new(capacity: [T; S]) -> Self {
        Self {
            value: <f64 as Default>::default(),
            items: HashMap::new(),
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

    pub fn peek_item(&self, value: f64, weights: [T; S]) //return reference somehow?
    -> Option<ItemBound<T, S>> {
        let mut item = ItemBound::<T, S>::new(value, weights, 0);
        if let Some(item_q) = self.items.get(&item.to_key()) {
            item.quantity = *item_q;
            return Some(item);
        } else {
            return None;
        }
    }

    pub fn peek_item_list(&self) -> Vec<ItemBound<T, S>> {
        let mut item_v: Vec<ItemBound<T, S>> = 
            Vec::<ItemBound<T, S>>::with_capacity(self.items.len());
        for item in self {
            item_v.push(item);
        }

        item_v
    }

    pub fn add(&mut self, item: ItemBound<T, S>) -> bool {
        if item.quantity == 0 {
            return true;
        }

        for r in 0..S {
            if item.weights[r].scale(item.quantity) + self.weights[r] > 
                self.capacity[r] {
                return false;
            }
        }

        if let Some(item_q) = self.items.get_mut(&item.to_key()) {
            *item_q += item.quantity;
        } else {
            self.items.insert(item.to_key(), item.quantity); 
        }

        for r in 0..S {
            self.weights[r] += item.weights[r].scale(item.quantity);
        }

        self.value += item.value * item.quantity as f64;
        return true;
    }

    pub fn add_mut(&mut self, item: &mut ItemBound<T, S>, count: usize)
    -> bool {
            if count == 0 {
                return true;
            }

            if item.quantity < count {
                return false;
            }

            for r in 0..S {
                if item.weights[r].scale(count) + self.weights[r] > 
                    self.capacity[r] {
                    return false;
                }
            }

            if let Some(item_q) = self.items.get_mut(&item.to_key()) {
                *item_q += count;
            } else {
                self.items.insert(item.to_key(), count); 
            }

            for r in 0..S {
                self.weights[r] += item.weights[r].scale(count);
            }

            self.value += item.value * count as f64;
            item.quantity -= count;
            return true;
    }

    //removes the corresponding amount of items from the hashmap.
    //returns the items removed (as Option).
    pub fn remove(&mut self, item: ItemBound<T, S>) -> Option<ItemBound<T, S>> {
        if let Some(item_q) = self.items.get_mut(&item.to_key()) {
            match (*item_q).cmp(&item.quantity) {
                Ordering::Less => {
                    return None;
                },

                Ordering::Greater => {
                    *item_q -= item.quantity;
                },

                Ordering::Equal => {
                    self.items.remove(&item.to_key());
                },
            }

            self.value -= item.value * item.quantity as f64;
            for r in 0..S {
                self.weights[r] -= item.weights[r].scale(item.quantity);
            }

            return Some(item);
        } else {
            return None;
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.value = 0.0;
        for r in 0..S {
            self.weights[r] = T::default();
        }
    }

    pub fn iter<'a>(&'a self) -> KnapsackIter<'a, T, S> {
        KnapsackIter::<'a, T, S> {
            itr: self.items.iter(),
            phantom: PhantomData,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> KnapsackIterMut<'a, T, S> {
        KnapsackIterMut::<'a, T, S> {
            itr: self.items.iter_mut(),
            phantom: PhantomData,
        }
    }
}

pub struct KnapsackIntoIter<T, const S: usize> 
where
    T: CompatibleProblemType,
{
    itr: std::collections::hash_map::IntoIter<(u64, [u64; S]), usize>,
    phantom: PhantomData<T>,
}

pub struct KnapsackIter<'a, T, const S: usize> 
where
    T: CompatibleProblemType,
{
    itr: std::collections::hash_map::Iter<'a, (u64, [u64; S]), usize>,
    phantom: PhantomData<T>,
}

pub struct KnapsackIterMut<'a, T, const S: usize> 
where
    T: CompatibleProblemType,
{
    itr: std::collections::hash_map::IterMut<'a, (u64, [u64; S]), usize>,
    phantom: PhantomData<T>,
}

impl<T, const S: usize> Iterator for KnapsackIntoIter<T, S>
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((v, w), q)) = self.itr.next() {
            let item = ItemBound::<T, S> {
                value: f64::key_to_type(v),
                weights: w.map(|x| T::key_to_type(x)), 
                quantity: q,
            };

            return Some(item);
        } else {
            return None;
        }
    }
}

impl<'a, T, const S: usize> Iterator for KnapsackIter<'a, T, S>
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((v, w), q)) = self.itr.next() {
            let item = ItemBound::<T, S> {
                value: f64::key_to_type(*v),
                weights: w.map(|x| T::key_to_type(x)), 
                quantity: *q,
            };

            return Some(item);
        } else {
            return None;
        }
    }
}

impl<'a, T, const S: usize> Iterator for KnapsackIterMut<'a, T, S>
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((v, w), q)) = self.itr.next() {
            let item = ItemBound::<T, S> {
                value: f64::key_to_type(*v),
                weights: w.map(|x| T::key_to_type(x)), 
                quantity: *q,
            };

            return Some(item);
        } else {
            return None;
        }
    }
}

impl<T, const S: usize> IntoIterator for Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;
    type IntoIter = KnapsackIntoIter<T, S>;

    fn into_iter(self) -> Self::IntoIter {
        KnapsackIntoIter::<T, S> {
            itr: self.items.into_iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T, const S: usize> IntoIterator for &'a Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;
    type IntoIter = KnapsackIter<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        KnapsackIter::<'a, T, S> {
            itr: self.items.iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T, const S: usize> IntoIterator for &'a mut Knapsack<T, S> 
where
    T: CompatibleProblemType,
{
    type Item = ItemBound<T, S>;
    type IntoIter = KnapsackIterMut<'a, T, S>;

    fn into_iter(self) -> Self::IntoIter {
        KnapsackIterMut::<'a, T, S> {
            itr: self.items.iter_mut(),
            phantom: PhantomData,
        }
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

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Knapsack<T, S>> {
        self.knapsacks.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Knapsack<T, S>> {
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

impl<'a, T, const S: usize> IntoIterator for &'a mut ProblemKnapsacks<T, S> 
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

impl<T, const S: usize> std::ops::IndexMut<usize> for ProblemKnapsacks<T, S>
where
    T: CompatibleProblemType,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.knapsacks[index]
    }
}
