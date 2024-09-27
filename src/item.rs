use crate::unbound_struct::{unbound, Quantity};
use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{BoundedProblem, BoundedProblemMut,
                          UnboundedProblem};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Item<T, const S: usize, N> 
where 
    T: CompatibleProblemType,
    N: Quantity,
{
    pub value: f64,
    pub weights: [T; S],
    pub quantity: N,
}

pub type ItemBound<T, const S: usize> = Item::<T, S, usize>;
pub type ItemUnbound<T, const S: usize> = Item::<T, S, unbound>;

impl<T, const S: usize, N> Item<T, S, N>
where
    T: CompatibleProblemType,
    N: Quantity,
{
    pub fn to_key(&self) -> (u64, [u64; S]) {
        (self.value.to_bits() as u64, self.weights.map(|x| T::type_to_key(x)))
    }
}

impl<T, const S: usize> ItemBound<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn new(
        value: f64,
        weights: [T; S],
        quantity: usize,
    ) -> Self {
        Self {
            value: value,
            weights: weights,
            quantity: quantity,
        }
    }
}

impl<T, const S: usize> ItemUnbound<T, S> 
where
    T: CompatibleProblemType,
{
    pub fn new(
        value: f64,
        weights: [T; S],
    ) -> Self {
        Self {
            value: value,
            weights: weights,
            quantity: unbound,
        }
    }
}

#[derive(Default, Clone)]
pub struct ProblemItems<T, const S: usize, N>
where
    T: CompatibleProblemType,
    N: Quantity,
{
    pub(crate) items_hash: HashMap<(u64, [u64; S]), usize>,
    pub(crate) items: Vec<Item<T, S, N>>, 
}

impl<T, const S: usize> ProblemItems<T, S, usize>
where
    T: CompatibleProblemType,
{
    pub fn new() -> Self {
        ProblemItems::<T, S, usize> {
            items_hash: HashMap::new(),
            items: Vec::<ItemBound<T, S>>::new(),
        }
    }

    pub fn add(&mut self, item: ItemBound<T, S>) {
        if item.quantity == 0 {
            return;
        }

        if let Some(index) = self.items_hash.get_mut(&item.to_key()) {
            self.items[*index].quantity += item.quantity;
        } else {
            self.items_hash.insert(item.to_key(), self.len());
            self.items.push(item);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ItemBound<T, S>> {
        self.items.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ItemBound<T, S>> {
        self.items.iter_mut()
    }

    pub fn insert_into(self, knapsacks: ProblemKnapsacks<T, S>) 
    -> BoundedProblem<T, S> {
        BoundedProblem::<T, S> {
            items: self,
            knapsacks: knapsacks,
        }
    }

    pub fn insert_mut_into<'a>(&'a mut self, knapsacks: ProblemKnapsacks<T, S>)
    -> BoundedProblemMut<'a, T, S> {
        BoundedProblemMut::<T, S> {
            items: self,
            knapsacks: knapsacks,
        }
    }
}

impl<T, const S: usize> ProblemItems<T, S, unbound>
where
    T: CompatibleProblemType,
{
    pub fn new() -> Self {
        ProblemItems::<T, S, unbound> {
            items_hash: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: ItemUnbound<T, S>) {
        if let None = self.items_hash.get_mut(&item.to_key()) {
            self.items_hash.insert(item.to_key(), self.len()); 
            self.items.push(item);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ItemUnbound<T, S>> {
        self.items.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, ItemUnbound<T, S>> {
        self.items.iter_mut()
    }

    pub fn insert_into(self, knapsacks: ProblemKnapsacks<T, S>)
    -> UnboundedProblem<T, S> {
        UnboundedProblem::<T, S> {
            items: self,
            knapsacks: knapsacks,
        }
    }
}

impl<T, const S: usize, N> IntoIterator for ProblemItems<T, S, N> 
where
    T: CompatibleProblemType,
    N: Quantity,
{
    type Item = Item<T, S, N>;
    type IntoIter = std::vec::IntoIter<Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T, const S: usize, N> IntoIterator for &'a ProblemItems<T, S, N> 
where
    T: CompatibleProblemType,
    N: Quantity,
{
    type Item = <std::slice::Iter<'a, Item<T, S, N>> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_slice().into_iter()
    }
}

impl<'a, T, const S: usize, N> IntoIterator for &'a mut ProblemItems<T, S, N> 
where
    T: CompatibleProblemType,
    N: Quantity,
{
    type Item = <std::slice::IterMut<'a, Item<T, S, N>> as Iterator>::Item;
    type IntoIter = std::slice::IterMut<'a, Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_mut_slice().into_iter()
    }
}

impl<T, const S: usize, N> std::ops::Index<usize> for ProblemItems<T, S, N>
where
    T: CompatibleProblemType,
    N: Quantity,
{
    type Output = Item<T, S, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T, const S: usize, N> std::ops::IndexMut<usize> for ProblemItems<T, S, N>
where
    T: CompatibleProblemType,
    N: Quantity,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
