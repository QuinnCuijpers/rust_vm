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

macro_rules! impl_from_bits_unsigned {
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

macro_rules! impl_from_bits_signed {
    ($($ty:ty), *) => {
        $(
        impl From<crate::bits::Bits<{ std::mem::size_of::<$ty>() * 8 }>> for $ty {
            fn from(bits: crate::bits::Bits<{ std::mem::size_of::<$ty>() * 8 }>) -> Self {
                let n = std::mem::size_of::<$ty>() * 8;
                let mut value: $ty = 0;
                for (i, &b) in bits.bit_array.iter().enumerate() {
                    value |= (b as $ty) << i;
                }
                if n < <$ty>::BITS as usize && bits.bit_array[n - 1] {
                    let sign_extension = (!0 as $ty) << n;
                    value |= sign_extension;
                }
                value
            }
        }
        )*
    };
}

impl_bits_from!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_from_bits_unsigned!(u8, u16, u32, u64, usize);
impl_from_bits_signed!(i8, i16, i32, i64, isize);

impl<const N: usize> FromStr for Bits<N> {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = [false; N];

        if s.starts_with("-") {
            return Ok(Bits::try_from_signed_number(s.parse::<i64>()?)?);
        }

        let s = s.strip_prefix("0b").unwrap_or(s);

        match s.len() {
            len if len > N => {
                return Err(crate::bits::BitsParseError::Length {
                    expected: N,
                    found: len,
                    string: s.to_string(),
                }
                .into());
            }
            len if len < N => {
                let num = s
                    .parse::<u64>()
                    .map_err(|e| crate::bits::BitsParseError::Number {
                        source: e,
                        num: s.to_string(),
                    })?;

                if num >= (1 << N) {
                    return Err(crate::bits::BitsParseError::OutOfBounds {
                        value: num as usize,
                        max: (1 << N) - 1,
                    }
                    .into());
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
                            }
                            .into());
                        }
                    };
                }
            }
            _ => unreachable!(), // This should never happen
        }

        Ok(crate::bits::Bits { bit_array: bits })
    }
}
