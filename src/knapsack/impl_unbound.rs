use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::cmp::Ordering;

#[allow(non_camel_case_types)]
#[derive(Debug, Eq)]
pub struct unbound;

pub trait Quantity {
    fn is_unbound(&self) -> bool;
}

impl Quantity for usize {
    fn is_unbound(&self) -> bool {
        false
    }
}

impl Quantity for unbound {
    fn is_unbound(&self) -> bool {
        true
    }
}

//Overloading for the unbound type (unbound + T = unbound, unbound <= usize == true)
impl<N> Add<N> for unbound 
where
    N: Quantity,
{
    type Output = Self;

    fn add(self, _other: N) -> Self {
        Self {}
    }
}

impl<N> AddAssign<N> for unbound 
where
    N: Quantity,
{
    fn add_assign(&mut self, _other: N) {
        *self = Self {};
    }
}

impl<N> Sub<N> for unbound
where
    N: Quantity,
{
    type Output = Self;

    fn sub(self, _other: N) -> Self {
        Self {}
    }
}

impl<N> SubAssign<N> for unbound
where
    N: Quantity,
{
    fn sub_assign(&mut self, _other: N) {
        *self = Self {};
    }
}

impl<N> Mul<N> for unbound
where
    N: Quantity,
{
    type Output = Self;

    fn mul(self, _other: N) -> Self {
        Self {}
    }
}

impl<N> PartialEq<N> for unbound
where
    N: Quantity,
{
    fn eq(&self, other: &N) -> bool {
        N::is_unbound(other)
    }
}

impl<N> PartialOrd<N> for unbound
where
    N: Quantity,
{
    fn partial_cmp(&self, other: &N) -> Option<Ordering> {
        if self.gt(other) {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Less)
    }

    fn lt(&self, _other: &N) -> bool {
        false
    }

    fn le(&self, other: &N) -> bool {
        if N::is_unbound(other) {
            true 
        } else {
            false 
        }
    }

    fn gt(&self, other: &N) -> bool {
        if N::is_unbound(other) {
            false 
        } else {
            true
        }
    }

    fn ge(&self, _other: &N) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_USIZE: usize = 1;

    #[test]
    fn test_is_unbound() {
        assert!(unbound.is_unbound());
    }

    #[test]
    #[should_panic]
    fn test_not_unbound() {
        assert!(TEST_USIZE.is_unbound());
    }

    #[test]
    fn test_unbound_add_self() {
        assert_eq!(unbound + unbound, unbound);
    }

    #[test]
    fn test_unbound_add_usize() {
        assert_eq!(unbound + TEST_USIZE, unbound);
    }

    #[test]
    fn test_unbound_addassign_self() {
        let mut val = unbound;
        val += unbound;

        assert_eq!(val, unbound);
    }

    #[test]
    fn test_unbound_addassign_usize() {
        let mut val = unbound;
        val += TEST_USIZE;

        assert_eq!(val, unbound);
    }

    #[test]
    fn test_unbound_sub_unbound() {
        assert_eq!(unbound - unbound, unbound);
    }

    #[test]
    fn test_unbound_sub_usize() {
        assert_eq!(unbound - TEST_USIZE, unbound);
    }

    #[test]
    fn test_unbound_subassign_unbound() {
        let mut val = unbound;
        val -= unbound;

        assert_eq!(val, unbound);

        val -= TEST_USIZE;

        assert_eq!(val, unbound);
    }

    #[test]
    fn test_unbound_subassign_usize() {
        let mut val = unbound;
        val -= TEST_USIZE;

        assert_eq!(val, unbound);
    }

    #[test]
    fn test_unbound_mul_unbound() {
        assert_eq!(unbound * unbound, unbound);
    }

    #[test]
    fn test_unbound_mul_usize() {
        assert_eq!(unbound * TEST_USIZE, unbound);
    }

    #[test]
    fn test_unbound_partialeq_unbound() {
        assert_eq!(unbound, unbound);
    }

    #[test]
    #[should_panic]
    fn test_unbound_partialeq_usize() {
        assert_eq!(unbound, TEST_USIZE);
    }

    #[test]
    #[should_panic]
    fn test_unbound_partialord_lt_unbound() {
        assert!(unbound < unbound);
    }

    #[test]
    #[should_panic]
    fn test_unbound_partialord_lt_usize() {
        assert!(unbound < TEST_USIZE);
    }

    #[test]
    fn test_unbound_partialord_le_unbound() {
        assert!(unbound <= unbound);
    }

    #[test]
    #[should_panic]
    fn test_unbound_partialord_le_usize() {
        assert!(unbound <= TEST_USIZE);
    }

    #[test]
    #[should_panic]
    fn test_unbound_partialord_gt_unbound() {
        assert!(unbound > unbound);
    }

    #[test]
    fn test_unbound_partialord_gt_usize() {
        assert!(unbound > TEST_USIZE);
    }

    #[test]
    fn test_unbound_partialord_ge_unbound() {
        assert!(unbound >= unbound);
    }

    #[test]
    fn test_unbound_partialord_ge_usize() {
        assert!(unbound >= TEST_USIZE);
    }
}
