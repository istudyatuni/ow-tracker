const V15_KEYS_COUNT = 374

/**
 * @param  {string[]} keys
 * @param  {Set.<string>} opened
 * @return {string|null}
 */
export function encode_save(keys, opened) {
	keys = keys.sort()
	let keys_count = keys.length
	if (keys_count != V15_KEYS_COUNT) {
		console.error('trying to load save with wrong number of keys:', keys_count, 'expected', V15_KEYS_COUNT)
		return null
	}

	let packed = pack_bools(keys.map((id) => opened.has(id)))
	return btoa(packed.join(','))
}

/**
 * @param  {string[]} keys
 * @param  {string} encoded
 * @return {Set.<string>|null}
 */
export function decode_save(keys, encoded) {
	let unpacked = unpack_bools(atob(encoded).split(',').map((s) => parseInt(s)))
	keys = keys.sort()
	let keys_count = keys.length
	let unpacked_count = unpacked.length
	// can't check !== because at the end can be padding of zeroes
	if (unpacked_count < keys_count) {
		console.error('trying to load save with wrong number of keys:', unpacked_count, 'expected', keys_count)
		return null
	}

	let opened = new Set()
	for (let [i, key] of Object.entries(keys)) {
		if (unpacked[i]) {
			opened.add(key)
		}
	}
	return opened
}

function pack_bools(bools) {
	let bytes = [];
	for (let i = 0; i < bools.length; i += 8) {
		let byte = 0;
		for (let bit = 0; bit < 8; bit++) {
			if (bools[i + bit]) {
				byte |= 1 << (7 - bit);
			}
		}
		bytes.push(byte);
	}
	return bytes;
}

function unpack_bools(numbers) {
	const bools = [];
	for (let byte of numbers) {
		for (let bit = 7; bit >= 0; bit--) {
			bools.push(Boolean((byte >> bit) & 1));
		}
	}
	return bools;
}
