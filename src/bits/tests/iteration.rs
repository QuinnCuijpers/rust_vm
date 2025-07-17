use super::super::*;

#[test]
fn iteration() {
    let bits = Bits::<4>::from([true, false, true, false]);
    let mut iter = bits.iter();
    assert_eq!(iter.next(), Some(&true));
    assert_eq!(iter.next(), Some(&false));
}

#[test]
fn mutable_iteration() {
    let mut bits = Bits::<4>::from([false; 4]);
    for bit in bits.iter_mut() {
        *bit = true;
    }
    assert_eq!(bits.bit_array, [true; 4]);
}

#[test]
fn into_iter() {
    let bits = Bits::<3>::from([true, false, true]);
    let collected: Vec<bool> = bits.into_iter().collect();
    assert_eq!(collected, vec![true, false, true]);
}
