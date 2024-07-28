pub fn combine_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}
