pub mod error;
pub mod from_impl;
pub mod impl_ops;
pub mod macros;

pub use error::*;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Bits<const N: usize> {
    pub(crate) bit_array: [bool; N],
}

impl<const N: usize> Bits<N> {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.bit_array.iter()
    }

    pub(crate) fn slice<const M: usize>(&self, start: usize) -> Bits<M> {
        assert!(start + M <= N, "Slice out of bounds");
        let mut out = [false; M];
        out.copy_from_slice(&self.bit_array[start..start + M]);
        Bits { bit_array: out }
    }

    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, bool> {
        self.bit_array.iter_mut()
    }

    pub fn to_usize(self) -> usize {
        self.bit_array
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &b)| acc | ((b as usize) << i))
    }

    pub(crate) fn to_signed(self) -> isize {
        let mut out = 0;
        for (i, &b) in self.bit_array.iter().enumerate() {
            if b {
                out |= 1 << i;
            }
        }
        if self.bit_array[N - 1] {
            // Check sign bit
            out |= -(1 << (N - 1));
        }
        out
    }

    pub fn try_from_signed_number<T>(value: T) -> Result<Self, super::BitsParseError>
    where
        T: Into<i64> + Copy,
    {
        let val_i64: i64 = value.into();

        if val_i64 < -(1 << (N - 1)) || val_i64 >= (1 << (N - 1)) {
            return Err(super::BitsParseError::OutOfBounds {
                value: val_i64 as usize,
                max: (1 << (N - 1)) - 1,
            });
        }

        let mut res = [false; N];
        let val_u64 = val_i64 as u64;
        res.iter_mut()
            .enumerate()
            .for_each(|(i, b)| *b = ((val_u64 >> i) & 1) != 0);

        Ok(Bits { bit_array: res })
    }

    pub fn try_from_unsigned_number<T>(value: T) -> Result<Self, super::BitsParseError>
    where
        T: Into<u64> + Copy,
    {
        let val_u64: u64 = value.into();

        if val_u64 >= (1u64 << N) {
            return Err(super::BitsParseError::OutOfBounds {
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

    fn from_str_binary(stripped: &str) -> Result<Bits<N>, crate::error::VmError> {
        let bits = [false; N];
        if stripped.len() > N {
            return Err(crate::bits::BitsParseError::Length {
                expected: N,
                found: stripped.len(),
                string: stripped.to_string(),
            }
            .into());
        } else if stripped.len() < N {
            let mut padded = String::with_capacity(N);
            for _ in 0..(N - stripped.len()) {
                padded.push('0');
            }
            padded.push_str(stripped);
            return Self::from_str_binary(&padded);
        }

        let mut bits = bits;
        for (i, c) in stripped.chars().rev().enumerate() {
            bits[i] = match c {
                '0' => false,
                '1' => true,
                _ => {
                    return Err(crate::bits::BitsParseError::Character { character: c }.into());
                }
            };
        }

        Ok(Bits { bit_array: bits })
    }
}

impl<const N: usize> Default for Bits<N> {
    fn default() -> Self {
        Bits {
            bit_array: [false; N],
        }
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

impl<const N: usize, const L: usize> PartialEq<Bits<L>> for Bits<N> {
    fn eq(&self, other: &Bits<L>) -> bool {
        self.to_usize() == other.to_usize()
    }
}

#[cfg(test)]
mod tests;
