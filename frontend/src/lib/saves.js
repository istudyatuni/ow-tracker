const V15_KEYS_COUNT = 374

export function get_save_opened_facts(facts_data) {
	// todo: not sure if read and newlyRevealed affect showing
	// || fact.read || fact.newlyRevealed
	let is_fact_opened = (fact) => fact.revealOrder >= 0;

	// which facts in save are opened
	let opened_facts = new Set();
	for (let [id, fact] of Object.entries(facts_data)) {
		if (is_fact_opened(fact)) {
			opened_facts.add(id);
		}
	}
	return opened_facts
}

export function export_save_to_browser_url(keys, opened) {
	let encoded = encode_save(keys, opened)
	window.location.hash = `save=${encoded}`
}

export function get_save_from_browser_url(keys) {
	let encoded = window.location.hash.split('=')[1]
	let opened = decode_save(keys, encoded)
	return opened
}

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
	return btoa(String.fromCharCode(...packed))
}

/**
 * @param  {string[]} keys
 * @param  {string} encoded
 * @return {Set.<string>|null}
 */
export function decode_save(keys, encoded) {
	let bytes = [...atob(encoded)].map((char) => char.charCodeAt(0))
	let unpacked = unpack_bools(bytes)
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
