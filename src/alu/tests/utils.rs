#[macro_export]
macro_rules! bits_8 {
    ($num:expr) => {{
        assert!(
            $num <= 0xFF,
            "bits_8! only accepts values fitting in 8 bits"
        );
        use $crate::bits::Bits;
        Bits::<8>::from($num as u8)
    }};
}

#[macro_export]
macro_rules! assert_bits {
    ($expr:expr, $expected:expr) => {
        assert_eq!(format!("{:08b}", u8::from($expr)), $expected);
    };
}
