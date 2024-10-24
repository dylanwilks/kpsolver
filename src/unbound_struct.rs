use crate::compatible_problem_type_trait::UnboundCompatibility;

use std::cmp::Ordering;
use std::ops::{
    Add, Sub, Mul, 
    AddAssign, SubAssign 
};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy)]
pub struct unbound;
impl UnboundCompatibility for unbound {
    fn null() -> Option<Self> { 
        None 
    }

    fn is_unbound() -> bool {
        true
    }
}

impl<T> Add<T> for unbound {
    type Output = Self;

    fn add(self, _rhs: T) -> Self::Output {
        self
    }
}

impl<T> AddAssign<T> for unbound {
    fn add_assign(&mut self, _rhs: T) {}
}

impl<T> Sub<T> for unbound {
    type Output = Self;

    fn sub(self, _rhs: T) -> Self::Output {
        self
    }
}

impl<T> SubAssign<T> for unbound {
    fn sub_assign(&mut self, _rhs: T) {}
}

impl<T> Mul<T> for unbound 
where T: UnboundCompatibility + std::cmp::PartialEq,
{
    type Output = Option<unbound>;

    fn mul(self, rhs: T) -> Self::Output {
        if let Some(null) = T::null() {
            if rhs == null {
                return None;
            }
        }

        Some(unbound)
    }
}

impl<T> PartialEq<T> for unbound
where T: UnboundCompatibility,
{
    fn eq(&self, _rhs: &T) -> bool {
        if T::is_unbound() {
            true
        } else {
            false
        }
    }
}

impl Eq for unbound {}

impl<T> PartialOrd<T> for unbound
where T: UnboundCompatibility,
{
    fn partial_cmp(&self, _rhs: &T) -> Option<Ordering> {
        if T::is_unbound() {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    const TEST_U32: u32 = 1;

    #[test]
    fn unbound_add_self() {
        assert_eq!(unbound + unbound, unbound);
    }

    #[test]
    fn unbound_add_usize() {
        assert_eq!(unbound + TEST_U32, unbound);
    }

    #[test]
    fn unbound_addassign_self() {
        let mut val = unbound;
        val += unbound;

        assert_eq!(val, unbound);
    }

    #[test]
    fn unbound_addassign_usize() {
        let mut val = unbound;
        val += TEST_U32;

        assert_eq!(val, unbound);
    }

    #[test]
    fn unbound_sub_unbound() {
        assert_eq!(unbound - unbound, unbound);
    }

    #[test]
    fn unbound_sub_usize() {
        assert_eq!(unbound - TEST_U32, unbound);
    }

    #[test]
    fn unbound_subassign_unbound() {
        let mut val = unbound;
        val -= unbound;

        assert_eq!(val, unbound);

        val -= TEST_U32;

        assert_eq!(val, unbound);
    }

    #[test]
    fn unbound_subassign_usize() {
        let mut val = unbound;
        val -= TEST_U32;

        assert_eq!(val, unbound);
    }

    #[test]
    fn unbound_mul_unbound() {
        assert_eq!(unbound * unbound, Some(unbound));
    }

    #[test]
    fn unbound_mul_usize() {
        assert_eq!(unbound * TEST_U32, Some(unbound));
    }

    #[test]
    fn unbound_partialeq_unbound() {
        assert_eq!(unbound, unbound);
    }

    #[test]
    #[should_panic]
    fn unbound_partialeq_usize() {
        assert_eq!(unbound, TEST_U32);
    }

    #[test]
    #[should_panic]
    fn unbound_partialord_lt_unbound() {
        assert!(unbound < unbound);
    }

    #[test]
    #[should_panic]
    fn unbound_partialord_lt_usize() {
        assert!(unbound < TEST_U32);
    }

    #[test]
    fn unbound_partialord_le_unbound() {
        assert!(unbound <= unbound);
    }

    #[test]
    #[should_panic]
    fn unbound_partialord_le_usize() {
        assert!(unbound <= TEST_U32);
    }

    #[test]
    #[should_panic]
    fn unbound_partialord_gt_unbound() {
        assert!(unbound > unbound);
    }

    #[test]
    fn unbound_partialord_gt_usize() {
        assert!(unbound > TEST_U32);
    }

    #[test]
    fn unbound_partialord_ge_unbound() {
        assert!(unbound >= unbound);
    }

    #[test]
    fn unbound_partialord_ge_usize() {
        assert!(unbound >= TEST_U32);
    }
}
