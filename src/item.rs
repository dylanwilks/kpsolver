use crate::compatible_problem_type_trait::{
    CompatibleProblemType, UnboundCompatibility
};
use crate::unbound_struct::unbound;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{
    BoundedProblem, BoundedProblemMut, UnboundedProblem
};

use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Item<T, const S: usize, N = T> 
where 
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    pub value: f64,
    pub weights: [T; S],
    pub quantity: N,
}

pub type ItemBound<T, const S: usize> = Item::<T, S>;
pub type ItemUnbound<T, const S: usize> = Item::<T, S, unbound>;

impl<T, const S: usize, N> Item<T, S, N>
where 
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    pub fn new(
        value: f64,
        weights: [T; S],
        quantity: N,
    ) -> Self {
        Self {
            value: value,
            weights: weights,
            quantity: quantity,
        }
    }

    pub fn to_key(&self) -> (u64, [u64; S]) {
        (self.value.to_bits() as u64, self.weights.map(|x| T::type_to_key(x)))
    }
}

#[derive(Clone)]
pub struct ProblemItems<T, const S: usize, N = T>
where
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    items: IndexMap<(u64, [u64; S]), Item<T, S, N>>,
}

impl<T, const S: usize> ProblemItems<T, S>
where T: CompatibleProblemType,
{
    pub fn new() -> Self {
        ProblemItems::<T, S> {
            items: IndexMap::new(),
        }
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

    pub fn add(&mut self, item: Item<T, S>) {
        if let Some(stored_item) = self.items.get_mut(&item.to_key()) {
            stored_item.quantity += item.quantity;
        } else {
            self.items.insert(item.to_key(), item);
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

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn into_iter(self) -> indexmap::map::IntoValues<(u64, [u64; S]), Item<T, S>> {
        self.items.into_values()
    }

    pub fn iter<'a>(&'a self) -> indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S>> {
        self.items.values()
    }

    pub fn iter_mut<'a>(&'a mut self) 
    -> indexmap::map::ValuesMut<'a, (u64, [u64; S]), Item<T, S>> {
        self.items.values_mut()
    }

    pub fn insert_into(self, knapsacks: ProblemKnapsacks<T, S>) -> BoundedProblem<T, S> {
        BoundedProblem::<T, S> {
            items: self,
            knapsacks: knapsacks,
        }
    }

    pub fn insert_mut_into<'a>(&'a mut self, knapsacks: ProblemKnapsacks<T, S>)
    -> BoundedProblemMut<'a, T, S> {
        BoundedProblemMut::<'a, T, S> {
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
            items: IndexMap::new(),
        }
    }

    pub fn get_item(&self, key: (f64, [T; S])) -> Option<&Item<T, S, unbound>> {
        self.items.get(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn get_item_mut(&mut self, key: (f64, [T; S])) -> Option<&mut Item<T, S, unbound>> {
        self.items.get_mut(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn get_index_of(&self, key: (f64, [T; S])) -> Option<usize> {
        self.items.get_index_of(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn add(&mut self, item: Item<T, S, unbound>) {
        if let None = self.items.get_mut(&item.to_key()) {
            self.items.insert(item.to_key(), item);
        }
    }

    pub fn remove_item(&mut self, key: (f64, [T; S])) -> Option<Item<T, S, unbound>> {
        self.items.shift_remove(&(key.0.to_bits() as u64, key.1.map(|x| T::type_to_key(x))))
    }

    pub fn remove_index(&mut self, index: usize) -> Option<Item<T, S, unbound>> {
        match self.items.shift_remove_index(index) {
            Some((_, item)) => Some(item),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn into_iter(self) -> indexmap::map::IntoValues<(u64, [u64; S]), Item<T, S, unbound>> {
        self.items.into_values()
    }

    pub fn iter<'a>(&'a self) -> indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S, unbound>> {
        self.items.values()
    }

    pub fn iter_mut<'a>(&'a mut self) 
    -> indexmap::map::ValuesMut<'a, (u64, [u64; S]), Item<T, S, unbound>> {
        self.items.values_mut()
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
    N: UnboundCompatibility,
{
    type Item = Item<T, S, N>;
    type IntoIter = indexmap::map::IntoValues<(u64, [u64; S]), Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_values()
    }
}

impl<'a, T, const S: usize, N> IntoIterator for &'a ProblemItems<T, S, N> 
where
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    type Item = <indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S, N>> as Iterator>::Item;
    type IntoIter = indexmap::map::Values<'a, (u64, [u64; S]), Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}

impl<'a, T, const S: usize, N> IntoIterator for &'a mut ProblemItems<T, S, N> 
where
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    type Item = <indexmap::map::ValuesMut<'a, (u64, [u64; S]), Item<T, S, N>> as Iterator>::Item;
    type IntoIter = indexmap::map::ValuesMut<'a, (u64, [u64; S]), Item<T, S, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values_mut()
    }
}

impl<T, const S: usize, N> std::ops::Index<usize> for ProblemItems<T, S, N>
where
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    type Output = Item<T, S, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T, const S: usize, N> std::ops::IndexMut<usize> for ProblemItems<T, S, N>
where
    T: CompatibleProblemType,
    N: UnboundCompatibility,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
