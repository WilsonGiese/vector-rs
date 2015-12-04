#![crate_name = "vector"]
#![deny(missing_docs)]

//! Simple vector library

use std::ops::{ Add, Mul, Neg, Sub };

/// 2D Vector; A Vector with 2 components
#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: Copy> {
    /// Element at index 0
    pub e0: T,
    /// Element at index 1
    pub e1: T,
}

impl<T> Vector2<T>  where T: Copy {

    /// Create new Vector2 with two entries of type T
    #[inline(always)]
    pub fn new(e0: T, e1: T) -> Vector2<T> {
        Vector2::<T> { e0: e0, e1: e1 }
    }
    
    /// Get entry by index
    #[inline(always)]
    pub fn get(self, index: usize) -> T {
        match index {
            0 => self.e0,
            1 => self.e1,
            _ => panic!("index out of bounds: the len is 2 but the index is {}", index)
        }
    }
}

/// Adds other to self
impl<T> Add<Vector2<T>> for Vector2<T> where T: Copy + Add<T, Output = T> {
    type Output = Vector2<T>;

    #[inline(always)]
    fn add (self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T> { e0: self.e0 + other.e0, e1: self.e1 + other.e1 }
    }
}

/// Compute the DOT PRODUCT
impl<T> Mul<Vector2<T>> for Vector2<T> where T: Copy + Mul<T, Output = T> + Add<T, Output = T> {
    type Output = T;

    #[inline(always)]
    fn mul (self, other: Vector2<T>) -> T {
        self.e0 * other.e0 + self.e1 * other.e1
    }
}

/// Preform an element-wise multiplication
impl<T> Mul<T> for Vector2<T> where T: Copy + Mul<T, Output = T> {
    type Output = Vector2<T>;

    #[inline(always)]
    fn mul (self, scalar: T) -> Vector2<T> {
        Vector2::<T> { e0: self.e0 * scalar, e1: self.e1 * scalar }
    }
}

/// Subtracts other from self
impl<T> Sub<Vector2<T>> for Vector2<T> where T: Copy + Sub<T, Output = T> {
    type Output = Vector2<T>;

    #[inline(always)]
    fn sub (self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T> { e0: self.e0 - other.e0, e1: self.e1 - other.e1 }
    }
}

/// Negate each entry in the Vector2
impl<T> Neg for Vector2<T> where T: Copy + Neg<Output = T> {
    type Output = Vector2<T>;

    #[inline(always)]
    fn neg(self) -> Vector2<T> {
        Vector2::<T>{ e0: -self.e0, e1: -self.e1 }
    }
}

#[test]
fn test_vector2_add() {
    let v1 = Vector2::<i32> { e0: 2, e1: 6 };
    let v2 = Vector2::<i32> { e0: 4, e1: 8 };
    let v3 = v1 + v2;

    assert_eq!(v3.e0, 6);
    assert_eq!(v3.e1, 14);
}

#[test]
fn test_vector2_sub() {
    let v1 = Vector2::<i32> { e0: 7, e1: 8 };
    let v2 = Vector2::<i32> { e0: 2, e1: 9 };
    let v3 = v1 - v2;

    assert_eq!(v3.e0, 5);
    assert_eq!(v3.e1, -1);
}

#[test]
fn test_vector2_neg() {
    let v1 = Vector2::<i32> { e0: 1, e1: -1 };
    let v2 = -v1;

    assert_eq!(v2.e0, -1);
    assert_eq!(v2.e1, 1);
}

#[test]
fn test_vector2_dot() {
    let v1 = Vector2::<i32> { e0: 2, e1: 3 };
    let v2 = Vector2::<i32> { e0: 4, e1: 5 };
    let s = v1 * v2;

    assert_eq!(s, 23);
}

#[test]
fn test_vector2_mut_scalar() {
    let v1 = Vector2::<i32>::new(1, 2);
    let v2 = v1 * 2;

    assert_eq!(v2.e0, 2);
    assert_eq!(v2.e1, 4);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 2 but the index is 2")]
fn test_get_xy() {
    let v = Vector2::<i32> { e0: 1, e1: 2 };

    assert_eq!(v.get(0), 1);
    assert_eq!(v.get(1), 2);
    v.get(2);
}
