use crate::bits::Bits;
use crate::BitsParseError;

#[test]
fn try_from_unsigned() {
    let bits: Bits<8> = Bits::try_from_unsigned_number(255u8).unwrap();
    assert_eq!(
        bits.bit_array,
        [true, true, true, true, true, true, true, true]
    );

    let bits: Bits<4> = Bits::try_from_unsigned_number(15u8).unwrap();
    assert_eq!(bits.bit_array, [true, true, true, true]);

    let out_of_bounds = Bits::<4>::try_from_unsigned_number(16u8);
    assert_eq!(
        out_of_bounds,
        Err(BitsParseError::OutOfBounds {
            value: 16,
            max: (1 << 4) - 1
        })
    );
}
