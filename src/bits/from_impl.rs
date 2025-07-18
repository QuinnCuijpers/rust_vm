use std::str::FromStr;

use crate::bits::Bits;

macro_rules! impl_bits_from {
    ($($t:ty), *) => {
        $(
        impl From<$t> for crate::bits::Bits<{ std::mem::size_of::<$t>() * 8 }> {
            fn from(value: $t) -> Self {
                let mut res = [false; std::mem::size_of::<$t>() * 8];
                for (i, bit) in res.iter_mut().enumerate() {
                    *bit = (value >> i) & 1 != 0;
                }
                crate::bits::Bits { bit_array: res }
            }
        }
    )*
    };
}

macro_rules! impl_from_bits {
    ($($ty:ty), *) => {
        $(
        impl From<crate::bits::Bits<{ std::mem::size_of::<$ty>() * 8 }>> for $ty {
            fn from(bits: crate::bits::Bits<{ std::mem::size_of::<$ty>() * 8 }>) -> Self {
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
        impl From<&crate::bits::Bits<{ std::mem::size_of::<$t>() * 8 }>> for $t {
            fn from(bits: &crate::bits::Bits<{ std::mem::size_of::<$t>() * 8 }>) -> Self {
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

impl<const N: usize> FromStr for Bits<N> {
    type Err = crate::bits::BitsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = [false; N];

        match s.len() {
            len if len > N => {
                return Err(crate::bits::BitsParseError::Length {
                    expected: N,
                    found: len,
                });
            }
            len if len < N => {
                let num = s
                    .parse::<u64>()
                    .map_err(|e| crate::bits::BitsParseError::Number { source: e })?;

                if num >= (1 << N) {
                    return Err(crate::bits::BitsParseError::OutOfBounds {
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
                            return Err(crate::bits::BitsParseError::Character {
                                character: *c as char,
                            })
                        }
                    };
                }
            }
            _ => unreachable!(), // This should never happen
        }

        Ok(crate::bits::Bits { bit_array: bits })
    }
}
