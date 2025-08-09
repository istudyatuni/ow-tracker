pub type Packed = u8;

const PACKED_SIZE: usize = Packed::BITS as usize;

pub const KEYS_COUNT: usize = 374;
const PACKED_LEN: usize = {
    if KEYS_COUNT % PACKED_SIZE == 0 {
        KEYS_COUNT / PACKED_SIZE
    } else {
        KEYS_COUNT / PACKED_SIZE + 1
    }
};

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
        && packed.len() == PACKED_LEN
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

#[cfg(debug_assertions)]
pub fn has_bool_enabled(save: &[Packed], index: usize) -> bool {
    index < KEYS_COUNT
        && save.len() == PACKED_LEN
        && (save[index / PACKED_SIZE] >> (PACKED_SIZE - index % PACKED_SIZE - 1)) & 1 == 1
}

#[cfg(debug_assertions)]
pub fn enable_bool(save: &[Packed], index: usize) -> Vec<Packed> {
    let mut save = save.to_vec();
    save[index / PACKED_SIZE] |= 1 << (PACKED_SIZE - index % PACKED_SIZE - 1);
    save
}

/*/// Number of enabled keys in save
pub fn enabled_num(packed: &[Packed]) -> u32 {
    packed.iter().map(|num| num.count_ones()).sum()
}

pub fn learned_percent(count: u32) -> f32 {
    count as f32 / KEYS_COUNT as f32 * 100_f32
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_bools() {
        let table = [
            (&[1, 1, 1, 1, 1, 1, 1, 1], vec![255]),
            (&[0, 1, 1, 1, 1, 1, 1, 1], vec![127]),
            (&[0, 1, 0, 0, 0, 0, 0, 0], vec![64]),
            (&[0, 0, 0, 0, 0, 0, 0, 0], vec![0]),
        ];
        for (bools, expected) in table {
            let bools = bools.iter().map(|&n| n != 0).collect::<Vec<_>>();
            assert_eq!(pack_bools(&bools), expected);
        }
    }
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
    #[test]
    fn test_has_bool_enabled() {
        fn generate_bool(num: usize) -> Vec<bool> {
            assert!(num < KEYS_COUNT);
            let mut res = (0..KEYS_COUNT).map(|_| false).collect::<Vec<_>>();
            res[num] = true;
            res
        }

        for index in 0..KEYS_COUNT {
            let save = pack_bools(&generate_bool(index));
            eprintln!("packed save: {save:?}");
            assert!(has_bool_enabled(&save, index), "checking for index {index}");
            for i in (0..KEYS_COUNT).filter(|&i| i != index) {
                assert!(
                    !has_bool_enabled(&save, i),
                    "checking {i} for test {index}, value: {}",
                    save[i / PACKED_SIZE],
                );
            }
        }
    }
    #[test]
    fn test_enable_bool() {
        let empty_save = (0..KEYS_COUNT).map(|_| false).collect::<Vec<_>>();

        for i in 0..KEYS_COUNT {
            assert!(!has_bool_enabled(&pack_bools(&empty_save), i));
            assert!(has_bool_enabled(
                &enable_bool(&pack_bools(&empty_save), i),
                i
            ));
            assert!(!has_bool_enabled(
                &enable_bool(&pack_bools(&empty_save), i),
                i + 1
            ));
        }
    }
}
