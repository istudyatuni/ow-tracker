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

/// Check if save contains valid number of keys
pub fn is_valid_number_of_keys(packed: &[Packed]) -> bool {
    packed.len() == KEYS_COUNT / 8 + 1
        && packed.len() * PACKED_SIZE >= KEYS_COUNT
        && packed
            .last()
            // check that last number does not fill zeros to the right of KEYS_COUNT
            .is_some_and(|n| n << (KEYS_COUNT % PACKED_SIZE) == 0)
}

/// Check if new save only add new keys to old and not override old
pub fn is_allowed_to_override(old: &[Packed], new: &[Packed]) -> bool {
    old.iter().zip(new.iter()).all(|(&o, &n)| {
        (0..PACKED_SIZE).all(|i| {
            let o = (o >> i) & 1;
            let n = (n >> i) & 1;
            // allowed cases (old -> new):
            // 0 -> 0
            // 0 -> 1
            // 1 -> 1
            // disallowed cases:
            // 0 -> 1
            o == 0 || n == 1
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subset_of_old() {
        let table = [
            (&[1, 1, 1, 1, 1, 1, 1, 1], &[1, 1, 1, 1, 1, 1, 1, 1], true),
            (&[0, 1, 1, 1, 1, 1, 1, 1], &[1, 1, 1, 1, 1, 1, 1, 1], true),
            (&[1, 1, 1, 1, 1, 1, 1, 1], &[0, 1, 1, 1, 1, 1, 1, 1], false),
            (&[0, 1, 1, 1, 1, 1, 1, 1], &[0, 1, 1, 1, 1, 1, 1, 1], true),
            (&[0, 0, 0, 0, 0, 0, 0, 0], &[0, 0, 0, 0, 0, 0, 0, 0], true),
        ];
        for (old, new, expected) in table {
            let old = pack_bools(&old.iter().map(|&n| n != 0).collect::<Vec<_>>());
            let new = pack_bools(&new.iter().map(|&n| n != 0).collect::<Vec<_>>());
            assert_eq!(is_allowed_to_override(&old, &new), expected);
        }
    }
}
