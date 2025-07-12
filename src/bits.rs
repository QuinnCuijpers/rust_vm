use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Bits<const N: usize> {
    pub bit_array: [bool; N],
}

impl<const N: usize> Bits<N> {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.bit_array.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, bool> {
        self.bit_array.iter_mut()
    }
}

macro_rules! impl_bits_from {
    ($ty:ty) => {
        impl From<$ty> for Bits<{ std::mem::size_of::<$ty>() * 8 }> {
            fn from(value: $ty) -> Self {
                let mut res = [false; std::mem::size_of::<$ty>() * 8];
                for (i, bit) in res.iter_mut().enumerate() {
                    *bit = (value >> i) & 1 != 0;
                }
                Bits { bit_array: res }
            }
        }
    };
}

macro_rules! impl_from_bits {
    ($ty:ty) => {
        impl From<Bits<{ std::mem::size_of::<$ty>() * 8 }>> for $ty {
            fn from(bits: Bits<{ std::mem::size_of::<$ty>() * 8 }>) -> Self {
                bits.bit_array
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &b)| acc | ((b as $ty) << i))
            }
        }
    };
}

impl_bits_from!(u8);
impl_bits_from!(u16);
impl_bits_from!(u32);
impl_bits_from!(u64);
impl_bits_from!(usize);

impl_from_bits!(u8);
impl_from_bits!(u16);
impl_from_bits!(u32);
impl_from_bits!(u64);
impl_from_bits!(usize);

impl_bits_from!(i8);
impl_bits_from!(i16);
impl_bits_from!(i32);
impl_bits_from!(i64);
impl_bits_from!(isize);

impl_from_bits!(i8);
impl_from_bits!(i16);
impl_from_bits!(i32);
impl_from_bits!(i64);
impl_from_bits!(isize);

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

#[cfg(test)]
mod tests;
