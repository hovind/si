#![feature(const_generics)]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x : Kilogram<i64> = Si(1);
        let y : Kilogram<i64> = Si(2);
        assert_eq!(x + x, y);
    }
}

use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Si<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>(T);

type Kilogram<T> = Si<T, 0, 0, 1, 0, 0, 0, 0>;

impl<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>
    Si<T, { S }, { MET }, { KG }, { A }, { K }, { MOL }, { CD }>
{
    pub fn kg(a: T) -> Kilogram<T> {
        Si(a)
    }
}

impl<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>
    Add for Si<T, { S }, { MET }, { KG }, { A }, { K }, { MOL }, { CD }>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0))
    }
}

impl<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>
    Sub for Si<T, { S }, { MET }, { KG }, { A }, { K }, { MOL }, { CD }>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0))
    }
}

impl<T
    , const S1: isize, const MET1: isize, const KG1: isize, const A1: isize, const K1: isize, const MOL1: isize, const CD1: isize
    , const S2: isize, const MET2: isize, const KG2: isize, const A2: isize, const K2: isize, const MOL2: isize, const CD2: isize>
    Mul<Si<T, { S2 }, { MET2 }, { KG2 }, { A2 }, { K2 }, { MOL2 }, { CD2 }>>
    for Si<T, { S1 }, { MET1 }, { KG1 }, { A1 }, { K1 }, { MOL1 }, { CD1 }>
where
    T: Mul<T, Output = T>,
{
    type Output = Si<T, { S1 + S2 }, { MET1 + MET2 }, { KG1 + KG2 }, { A1 + A2 }, { K1 + K2 }, { MOL1 + MOL2 }, { CD1 + CD2 }>;

    fn mul(self, rhs: Si<T, { S2 }, { MET2 }, { KG2 }, { A2 }, { K2 }, { MOL2 }, { CD2 }>) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl<T
    , const S1: isize, const MET1: isize, const KG1: isize, const A1: isize, const K1: isize, const MOL1: isize, const CD1: isize
    , const S2: isize, const MET2: isize, const KG2: isize, const A2: isize, const K2: isize, const MOL2: isize, const CD2: isize>
    Div<Si<T, { S2 }, { MET2 }, { KG2 }, { A2 }, { K2 }, { MOL2 }, { CD2 }>>
    for Si<T, { S1 }, { MET1 }, { KG1 }, { A1 }, { K1 }, { MOL1 }, { CD1 }>
where
    T: Div<T, Output = T>,
{
    type Output = Si<T, { S1 - S2 }, { MET1 - MET2 }, { KG1 - KG2 }, { A1 - A2 }, { K1 - K2 }, { MOL1 - MOL2 }, { CD1 - CD2 }>;

    fn div(self, rhs: Si<T, { S2 }, { MET2 }, { KG2 }, { A2 }, { K2 }, { MOL2 }, { CD2 }>) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}
