#[macro_export]
macro_rules! bits_concat {
    ($a:expr, $b:expr) => {{
        const N: usize = $a.bit_array.len();
        const M: usize = $b.bit_array.len();
        let mut arr = [false; N + M];
        arr[..N].copy_from_slice(&$a.bit_array);
        arr[N..].copy_from_slice(&$b.bit_array);
        Bits { bit_array: arr }
    }};
}
