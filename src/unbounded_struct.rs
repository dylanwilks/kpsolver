use crate::compatible_problem_type_trait::UnboundedCompatibility;

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, Copy)]
pub struct unbounded;
impl UnboundedCompatibility for unbounded {
    fn null() -> Option<Self> {
        None
    }

    fn is_unbounded() -> bool {
        true
    }
}

impl<T> Add<T> for unbounded {
    type Output = Self;

    fn add(self, _rhs: T) -> Self::Output {
        self
    }
}

impl<T> AddAssign<T> for unbounded {
    fn add_assign(&mut self, _rhs: T) {}
}

impl<T> Sub<T> for unbounded {
    type Output = Self;

    fn sub(self, _rhs: T) -> Self::Output {
        self
    }
}

impl<T> SubAssign<T> for unbounded {
    fn sub_assign(&mut self, _rhs: T) {}
}

impl<T> Mul<T> for unbounded
where
    T: UnboundedCompatibility + std::cmp::PartialEq,
{
    type Output = Option<unbounded>;

    fn mul(self, rhs: T) -> Self::Output {
        if let Some(null) = T::null() {
            if rhs == null {
                return None;
            }
        }

        Some(unbounded)
    }
}

impl<T> PartialEq<T> for unbounded
where
    T: UnboundedCompatibility,
{
    fn eq(&self, _rhs: &T) -> bool {
        if T::is_unbounded() {
            true
        } else {
            false
        }
    }
}

impl Eq for unbounded {}

impl<T> PartialOrd<T> for unbounded
where
    T: UnboundedCompatibility,
{
    fn partial_cmp(&self, _rhs: &T) -> Option<Ordering> {
        if T::is_unbounded() {
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
    fn unbounded_add_self() {
        assert_eq!(unbounded + unbounded, unbounded);
    }

    #[test]
    fn unbounded_add_usize() {
        assert_eq!(unbounded + TEST_U32, unbounded);
    }

    #[test]
    fn unbounded_addassign_self() {
        let mut val = unbounded;
        val += unbounded;

        assert_eq!(val, unbounded);
    }

    #[test]
    fn unbounded_addassign_usize() {
        let mut val = unbounded;
        val += TEST_U32;

        assert_eq!(val, unbounded);
    }

    #[test]
    fn unbounded_sub_unbounded() {
        assert_eq!(unbounded - unbounded, unbounded);
    }

    #[test]
    fn unbounded_sub_usize() {
        assert_eq!(unbounded - TEST_U32, unbounded);
    }

    #[test]
    fn unbounded_subassign_unbounded() {
        let mut val = unbounded;
        val -= unbounded;

        assert_eq!(val, unbounded);

        val -= TEST_U32;

        assert_eq!(val, unbounded);
    }

    #[test]
    fn unbounded_subassign_usize() {
        let mut val = unbounded;
        val -= TEST_U32;

        assert_eq!(val, unbounded);
    }

    #[test]
    fn unbounded_mul_unbounded() {
        assert_eq!(unbounded * unbounded, Some(unbounded));
    }

    #[test]
    fn unbounded_mul_usize() {
        assert_eq!(unbounded * TEST_U32, Some(unbounded));
    }

    #[test]
    fn unbounded_partialeq_unbounded() {
        assert_eq!(unbounded, unbounded);
    }

    #[test]
    #[should_panic]
    fn unbounded_partialeq_usize() {
        assert_eq!(unbounded, TEST_U32);
    }

    #[test]
    #[should_panic]
    fn unbounded_partialord_lt_unbounded() {
        assert!(unbounded < unbounded);
    }

    #[test]
    #[should_panic]
    fn unbounded_partialord_lt_usize() {
        assert!(unbounded < TEST_U32);
    }

    #[test]
    fn unbounded_partialord_le_unbounded() {
        assert!(unbounded <= unbounded);
    }

    #[test]
    #[should_panic]
    fn unbounded_partialord_le_usize() {
        assert!(unbounded <= TEST_U32);
    }

    #[test]
    #[should_panic]
    fn unbounded_partialord_gt_unbounded() {
        assert!(unbounded > unbounded);
    }

    #[test]
    fn unbounded_partialord_gt_usize() {
        assert!(unbounded > TEST_U32);
    }

    #[test]
    fn unbounded_partialord_ge_unbounded() {
        assert!(unbounded >= unbounded);
    }

    #[test]
    fn unbounded_partialord_ge_usize() {
        assert!(unbounded >= TEST_U32);
    }
}
