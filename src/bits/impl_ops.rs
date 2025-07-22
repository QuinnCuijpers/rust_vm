use crate::bits::Bits;
use std::ops::{Add, AddAssign, Index, Not, Shl, Shr, Sub, SubAssign};

impl<const N: usize> Add for Bits<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [false; N];
        let mut carry = false;
        for ((o, a), b) in out
            .iter_mut()
            .zip(self.bit_array.iter())
            .zip(rhs.bit_array.iter())
        {
            *o = *a ^ *b ^ carry;
            carry = (*a & *b) | (*a & carry) | (*b & carry);
        }
        Bits { bit_array: out }
    }
}

impl<const N: usize> AddAssign for Bits<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> Sub for Bits<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        // Two's complement subtraction: a - b = a + (!b + 1)
        let not_rhs = !rhs;
        let one = Bits::from(1).resize();
        self + not_rhs + one
    }
}

impl<const N: usize> SubAssign for Bits<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const N: usize> std::ops::BitAnd for Bits<N> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut out = [false; N];
        for (o, (a, b)) in out
            .iter_mut()
            .zip(self.bit_array.iter().zip(rhs.bit_array.iter()))
        {
            *o = *a & *b;
        }
        Bits { bit_array: out }
    }
}

impl<const N: usize> std::ops::BitOr for Bits<N> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut out = [false; N];
        for (o, (a, b)) in out
            .iter_mut()
            .zip(self.bit_array.iter().zip(rhs.bit_array.iter()))
        {
            *o = *a | *b;
        }
        Bits { bit_array: out }
    }
}

impl<const N: usize> std::ops::BitXor for Bits<N> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = [false; N];
        for (o, (a, b)) in out
            .iter_mut()
            .zip(self.bit_array.iter().zip(rhs.bit_array.iter()))
        {
            *o = *a ^ *b;
        }
        Bits { bit_array: out }
    }
}

impl<const N: usize> Not for Bits<N> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let out = self.bit_array.map(|b| !b);
        Bits { bit_array: out }
    }
}

impl<const N: usize> Shr for Bits<N> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let shift_amount = rhs.to_usize();
        assert!(shift_amount < N, "Shift amount out of bounds");
        let mut out = [false; N];
        out[..(N - shift_amount)].copy_from_slice(&self.bit_array[shift_amount..N]);
        Bits { bit_array: out }
    }
}

impl<const N: usize> Shl for Bits<N> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let shift_amount = rhs.to_usize();
        assert!(shift_amount < N, "Shift amount out of bounds");
        let mut out = [false; N];
        out[shift_amount..N].copy_from_slice(&self.bit_array[..(N - shift_amount)]);
        Bits { bit_array: out }
    }
}

impl<const N: usize> Index<usize> for Bits<N> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.bit_array[index]
    }
}

impl<const N: usize> PartialOrd for Bits<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_usize().cmp(&other.to_usize()))
    }
}

impl<const N: usize> Ord for Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}
