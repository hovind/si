#![feature(const_generics)]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Si::kg(1) + Si::kg(1), Si::kg(2));
    }
}

use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Si<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>(T);

type Kilogram<T> = Si<T, 0, 0, 1, 0, 0, 0, 0>;

impl<T, const S: isize, const MET: isize, const KG: isize, const A: isize, const K: isize, const MOL: isize, const CD: isize>
    Si<T, { S }, { MET }, { KG }, { A }, { K }, { MOL }, { CD }>
{
    fn kg(a: T) -> Kilogram<T> {
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
