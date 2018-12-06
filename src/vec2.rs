use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg};
use std::str::FromStr;
use num_traits::{
    Num,
    identities::{One, Zero},
    sign::{Signed, Unsigned},
};

pub type Vec2i = Vec2<i32>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Clone
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait for Vec2<T>
        where
            T : $trait<Output = T>
        {
            type Output = Self;
            fn $fn(self, other:  Self) -> Self {
                Vec2 {
                    x: T::$fn(self.x, other.x),
                    y: T::$fn(self.y, other.y),
                }
            }
        }

        impl<T> $assign_trait for Vec2<T>
        where
            T : $assign_trait
        {
            fn $assign_fn(&mut self, other: Self) {
                T::$assign_fn(&mut self.x, other.x);
                T::$assign_fn(&mut self.y, other.y);
            }
        }
    };
}

macro_rules! impl_scalar {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
        impl<T> $trait<T> for Vec2<T>
        where
            T : $trait<Output=T> + Clone
        {
            type Output = Self;
            fn $fn(self, other: T) -> Self {
                Vec2 {
                    x: T::$fn(self.x, other.clone()),
                    y: T::$fn(self.y, other),
                }
            }
        }

        impl<T> $assign_trait<T> for Vec2<T>
        where
            T : $assign_trait + Clone
        {
            fn $assign_fn(&mut self, other: T) {
                T::$assign_fn(&mut self.x, other.clone());
                T::$assign_fn(&mut self.y, other);
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign);
impl_op!(Sub, sub, SubAssign, sub_assign);
impl_op!(Mul, mul, MulAssign, mul_assign);
impl_op!(Div, div, DivAssign, div_assign);
impl_op!(Rem, rem, RemAssign, rem_assign);

impl_scalar!(Mul, mul, MulAssign, mul_assign);
impl_scalar!(Div, div, DivAssign, div_assign);
impl_scalar!(Rem, rem, RemAssign, rem_assign);

impl<T> Neg for Vec2<T>
where
    T : Neg<Output=T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 {
            x: T::neg(self.x),
            y: T::neg(self.y),
        }
    }
}

impl<T> Zero for Vec2<T>
where
    T : Zero
{
    fn zero() -> Self {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        T::is_zero(&self.x) && T::is_zero(&self.y)
    }
}

impl<T> One for Vec2<T>
where
    T : One
{
    fn one() -> Self {
        Vec2 {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T> Num for Vec2<T>
where
    T: Num
{
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut iter = s.split(',');
        if let Some(value) = iter.next() {
            let x = T::from_str_radix(value.trim(), radix)?;
            if let Some(value) = iter.next() {
                let y = T::from_str_radix(value.trim(), radix)?;
                if let None = iter.next() {
                    return Ok(Vec2 { x, y })
                }
            }
        }

        if let Err(e) = T::from_str_radix("", radix) {
            return Err(e);
        }
        unreachable!()
    }
}

impl<T> Signed for Vec2<T>
where
    T : Signed + Clone
{
    fn abs(&self) -> Self {
        Vec2 {
            x: T::abs(&self.x),
            y: T::abs(&self.y),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let delta = self.clone() - other.clone();
        <Self as Signed>::abs(&delta)
    }

    fn signum(&self) -> Self {
        Vec2 {
            x: T::signum(&self.x),
            y: T::signum(&self.y),
        }
    }

    fn is_positive(&self) -> bool {
        T::is_positive(&self.x) && T::is_positive(&self.y)
    }

    fn is_negative(&self) -> bool {
        T::is_negative(&self.x) && T::is_negative(&self.y)
    }
}

impl<T> Unsigned for Vec2<T> where T : Unsigned { }

impl<T> FromStr for Vec2<T>
where
    T: Num
{
    type Err = <Self as Num>::FromStrRadixErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Num>::from_str_radix(s, 10)
    }
}
