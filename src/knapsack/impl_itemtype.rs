use std::ops::AddAssign;

#[derive(Debug, Clone)]
pub struct ItemData<T, const S: usize> 
where 
    T: ItemType<S>
{
    pub value: f64,
    pub weights: [T; S],
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ItemDataKey<const S: usize> {
    pub value_key: u64,
    pub weight_key: [u64; S],
}

impl<T, const S: usize> ItemData<T, S>
where
    T: ItemType<S>,
{
    pub fn to_key(&self) -> ItemDataKey<S> {
        T::to_key(self)
    }
}

impl<T, const S: usize> PartialEq for ItemData<T, S> 
where
    T: ItemType<S>
{
    fn eq(&self, other: &Self) -> bool {
        T::eq_data(self, other)
    }
}

impl<T, const S: usize> Eq for ItemData<T, S> 
where
    T: ItemType<S>,
{}

pub trait ItemType<const S: usize>: Default + Clone + Copy + AddAssign {
    fn to_key(item_data: &ItemData<Self, S>) -> ItemDataKey<S>;
    fn eq_data(item_data1: &ItemData<Self, S>, 
               item_data2: &ItemData<Self, S>) -> bool;
    fn mul_usize(self, other: usize) -> Self;
}

macro_rules! impl_unsigned_ItemType {
    ( $( $type:ty ),* ) => {
        $(
            impl<const S: usize> ItemType<S> for $type {
                fn to_key(item_data: &ItemData<Self, S>) -> ItemDataKey<S> {
                    ItemDataKey::<S> {
                        value_key: item_data.value.to_bits() as u64,
                        weight_key: item_data.weights.map(|x| x as u64),
                    }
                }

                fn eq_data(item_data1: &ItemData<Self, S>,
                           item_data2: &ItemData<Self, S>) -> bool {
                    item_data1.value.to_bits() == item_data2.value.to_bits() && 
                        item_data1.weights.iter().eq(item_data2.weights.iter())
                }

                fn mul_usize(self, other: usize) -> Self {
                    self * other as $type
                }
            }
        )*
    };
}

impl_unsigned_ItemType!(usize, u64, u32, u16, u8);

macro_rules! impl_float_ItemType {
    ( $( $type:ty ),* ) => {
        $(
            impl<const S: usize> ItemType<S> for $type {
                fn to_key(item_data: &ItemData<Self, S>) -> ItemDataKey<S> {
                    ItemDataKey::<S> {
                        value_key: item_data.value.to_bits() as u64,
                        weight_key: item_data.weights.map(|x| x.to_bits() as u64),
                    }
                }

                fn eq_data(item_data1: &ItemData<Self, S>,
                           item_data2: &ItemData<Self, S>) -> bool {
                    item_data1.value.to_bits() == item_data2.value.to_bits() &&
                        item_data1.weights.map(|x| x.to_bits()).iter()
                        .eq(item_data2.weights.map(|x| x.to_bits()).iter())
                }

                fn mul_usize(self, other: usize) -> Self {
                    self * other as $type 
                }
            }
        )*
    }
}

impl_float_ItemType!(f32, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_key_usize() {
        let item_data = ItemData::<usize, 1> {
            value: 10.0,
            weights: [10; 1]
        };

        let item_key = usize::to_key(&item_data);
        let item_key_test = ItemDataKey::<1> {
            value_key: 10.0_f64.to_bits(),
            weight_key: [10; 1],
        };

        assert_eq!(item_key, item_key_test);
    }

    #[test]
    fn test_eq_data_usize() {
        let item_data = ItemData::<usize, 1> {
            value: 10.0,
            weights: [10; 1]
        };

        assert_eq!(item_data, item_data);
    }

    #[test]
    #[should_panic]
    fn test_eq_data_usize_panic() {
        let item_data1 = ItemData::<usize, 1> {
            value: 1.0,
            weights: [1; 1],
        };

        let item_data2 = ItemData::<usize, 1> {
            value: 2.0,
            weights: [1; 1],
        };

        assert_eq!(item_data1, item_data2);
    }   

    #[test]
    fn test_mul_usize() {
        assert_eq!(<usize as ItemType<1>>::mul_usize(3, 2), 6);
    }

    #[test]
    fn test_to_key_f64() {
        let item_data = ItemData::<f64, 1> {
            value: 10.0,
            weights: [10.0; 1]
        };

        let item_key = f64::to_key(&item_data);
        let item_key_test = ItemDataKey::<1> {
            value_key: 10.0_f64.to_bits(),
            weight_key: [10.0_f64.to_bits(); 1],
        };

        assert_eq!(item_key, item_key_test);
    }

    #[test]
    fn test_eq_data_f64() {
        let item_data = ItemData::<f64, 1> {
            value: 10.0,
            weights: [10.0; 1],
        };
        
        assert_eq!(item_data, item_data);
    }

    #[test]
    #[should_panic]
    fn test_eq_data_f64_panic() {
        let item_data1 = ItemData::<f64, 1> {
            value: 1.0,
            weights: [1.0; 1],
        };

        let item_data2 = ItemData::<f64, 1> {
            value: 1.0,
            weights: [2.0; 1],
        };
        
        assert_eq!(item_data1, item_data2);
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(<f64 as ItemType<1>>::mul_usize(1.5, 2), 3.0);
    }
}
