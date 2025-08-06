pub type Packed = u8;

const PACKED_SIZE: usize = Packed::BITS as usize;

const KEYS_COUNT: usize = 374;

pub fn pack_bools(bools: &[bool]) -> Vec<Packed> {
    let mut bytes = Vec::with_capacity(bools.len() / PACKED_SIZE + 1);
    for chunk in bools.chunks(PACKED_SIZE) {
        let mut byte = 0;
        for (i, b) in chunk.iter().enumerate() {
            if *b {
                byte |= 1 << (PACKED_SIZE - 1 - i);
            }
        }
        bytes.push(byte);
    }
    bytes
}

/// Check whether save contains valid number of keys
pub fn is_valid_number_of_keys(packed: &[Packed]) -> bool {
    packed.len() == KEYS_COUNT / 8 + 1
        && packed.len() * PACKED_SIZE >= KEYS_COUNT
        && packed
            .last()
            // check that last number does not fill zeros to the right of KEYS_COUNT
            .is_some_and(|n| n << (KEYS_COUNT % PACKED_SIZE) == 0)
}

/// Number of enabled keys in save
pub fn enabled_num(packed: &[Packed]) -> u32 {
    packed.iter().map(|num| num.count_ones()).sum()
}
