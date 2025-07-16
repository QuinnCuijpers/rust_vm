use core::error;
use std::{ops::Index, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitsParseError {
    Length { expected: usize, found: usize },
    Character { character: char },
    Number { source: std::num::ParseIntError },
    OutOfBounds { value: usize, max: usize },
}

impl std::fmt::Display for BitsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitsParseError::Length { expected, found } => {
                write!(f, "Invalid length: expected {}, found {}", expected, found)
            }
            BitsParseError::Character { character } => {
                write!(f, "Invalid character: '{}'", character)
            }
            BitsParseError::Number { source } => write!(f, "Invalid number: {}", source),
            BitsParseError::OutOfBounds { value, max } => {
                write!(f, "Value {} is out of bounds (max: {})", value, max)
            }
        }
    }
}

impl error::Error for BitsParseError {}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Bits<const N: usize> {
    pub(crate) bit_array: [bool; N],
}

impl<const N: usize> Bits<N> {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.bit_array.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, bool> {
        self.bit_array.iter_mut()
    }

    pub(crate) fn to_usize(self) -> usize {
        self.bit_array
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &b)| acc | ((b as usize) << i))
    }

    pub fn try_from_unsigned_number<T>(value: T) -> Result<Self, BitsParseError>
    where
        T: Into<u64> + Copy,
    {
        let val_u64: u64 = value.into();

        if val_u64 >= (1u64 << N) {
            return Err(BitsParseError::OutOfBounds {
                value: val_u64 as usize,
                max: (1 << N) - 1,
            });
        }

        let mut res = [false; N];
        res.iter_mut()
            .enumerate()
            .for_each(|(i, b)| *b = (val_u64 & (1 << i)) != 0);

        Ok(Bits { bit_array: res })
    }

    pub fn resize<const M: usize>(self) -> Bits<M> {
        let mut out = [false; M];
        let min_len = N.min(M);
        (0..min_len).for_each(|i| {
            out[i] = self.bit_array[i];
        });
        Bits { bit_array: out }
    }

    pub(crate) fn split_into_chunks<const CHUNK_SIZE: usize>(self) -> Vec<Bits<CHUNK_SIZE>> {
        assert!(N % CHUNK_SIZE == 0, "Size must divide N evenly");
        let mut chunks = Vec::new();
        for chunk in self.bit_array.chunks(CHUNK_SIZE) {
            let mut bits = [false; CHUNK_SIZE];
            bits[..chunk.len()].copy_from_slice(chunk);
            chunks.push(Bits { bit_array: bits });
        }
        chunks.reverse();
        chunks
    }
}

impl<const N: usize> Default for Bits<N> {
    fn default() -> Self {
        Bits {
            bit_array: [false; N],
        }
    }
}

impl<const N: usize> FromStr for Bits<N> {
    /// Parses a string of '0's and '1's into a Bits instance.
    type Err = BitsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = [false; N];

        match s.len() {
            len if len > N => {
                return Err(BitsParseError::Length {
                    expected: N,
                    found: len,
                });
            }
            len if len < N => {
                let num = s
                    .parse::<u64>()
                    .map_err(|e| BitsParseError::Number { source: e })?;

                if num >= (1 << N) {
                    return Err(BitsParseError::OutOfBounds {
                        value: num as usize,
                        max: (1 << N) - 1,
                    });
                }

                bits.iter_mut().enumerate().for_each(|(i, b)| {
                    *b = (num & (1 << i)) != 0;
                });
            }
            len if len == N => {
                for (i, c) in s.as_bytes().iter().rev().enumerate() {
                    bits[i] = match c {
                        b'0' => false,
                        b'1' => true,
                        _ => {
                            return Err(BitsParseError::Character {
                                character: *c as char,
                            })
                        }
                    };
                }
            }
            _ => unreachable!(), // This should never happen
        }

        Ok(Bits { bit_array: bits })
    }
}

impl<const N: usize> std::fmt::Display for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &bit in self.bit_array.iter().rev() {
            write!(f, "{}", if bit { '1' } else { '0' })?;
        }
        Ok(())
    }
}

macro_rules! impl_bits_from {
    ($($t:ty), *) => {
        $(
        impl From<$t> for Bits<{ std::mem::size_of::<$t>() * 8 }> {
            fn from(value: $t) -> Self {
                let mut res = [false; std::mem::size_of::<$t>() * 8];
                for (i, bit) in res.iter_mut().enumerate() {
                    *bit = (value >> i) & 1 != 0;
                }
                Bits { bit_array: res }
            }
        }
    )*
    };
}

macro_rules! impl_from_bits {
    ($($ty:ty), *) => {
        $(
        impl From<Bits<{ std::mem::size_of::<$ty>() * 8 }>> for $ty {
            fn from(bits: Bits<{ std::mem::size_of::<$ty>() * 8 }>) -> Self {
                bits.bit_array
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &b)| acc | ((b as $ty) << i))
            }
        }
    )*
    };
}

macro_rules! impl_from_ref_bits {
    ($($t:ty),*) => {
        $(
        impl From<&Bits<{ std::mem::size_of::<$t>() * 8 }>> for $t {
            fn from(bits: &Bits<{ std::mem::size_of::<$t>() * 8 }>) -> Self {
                bits.bit_array
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &b)| acc | ((b as $t) << i))
            }
        }
    )*
    };
}

impl_bits_from!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl_from_bits!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl_from_ref_bits!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl<const N: usize> From<[bool; N]> for Bits<N> {
    fn from(value: [bool; N]) -> Self {
        Bits { bit_array: value }
    }
}

impl<const N: usize> From<Bits<N>> for [bool; N] {
    fn from(value: Bits<N>) -> Self {
        value.bit_array
    }
}

impl<const N: usize> IntoIterator for Bits<N> {
    type Item = bool;

    type IntoIter = std::array::IntoIter<bool, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.bit_array.into_iter()
    }
}

impl<const N: usize> Index<usize> for Bits<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bit_array[index]
    }
}

impl<const N: usize, const L: usize> PartialEq<Bits<L>> for Bits<N> {
    fn eq(&self, other: &Bits<L>) -> bool {
        self.to_usize() == other.to_usize()
    }
}

#[cfg(test)]
mod tests;
