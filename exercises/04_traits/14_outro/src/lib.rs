
use std::ops::Add;
use std::ops;


// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.

//   It should be possible to print its debug representation.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct SaturatingU16{
    value : u16,
}

impl SaturatingU16{
    pub fn new(value : u16) -> SaturatingU16{
        SaturatingU16{value : value}
    }
}

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        Self::new(value)
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        Self::new(value as u16)
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        Self::new(*value)
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> SaturatingU16 {
        Self::new(*value as u16)
    }
}


//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
impl ops::Add for SaturatingU16 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.value.saturating_add(other.value))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: &SaturatingU16) -> Self::Output {
        self + *other
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, value: u16) -> Self::Output {
        self + SaturatingU16::from(value)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: &u16) -> Self::Output {
        self +  SaturatingU16::from(*other)
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
