pub fn num_digits(value: u32) -> u8 {
    value.checked_ilog10().unwrap_or(0) as u8 + 1
}
