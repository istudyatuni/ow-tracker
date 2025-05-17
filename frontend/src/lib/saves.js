const V15_KEYS_COUNT = 374;
const V16_KEYS_COUNT = 374;
export const KEYS_COUNT = V16_KEYS_COUNT;
const GAME_VERSION = "1.1.16";
const ENCODING_VERSION = 1;
const ENCODED_SAVE_LEN = 64;

export function get_save_opened_facts(facts_data) {
	// todo: not sure if read and newlyRevealed affect showing
	// if this function changed, should increase ENCODING_VERSION
	// || fact.read || fact.newlyRevealed
	let is_fact_opened = (fact) => fact.revealOrder >= 0;

	// which facts in save are opened
	let opened_facts = new Set();
	let entries = Object.entries(facts_data);
	if (entries.length === 0) {
		return opened_facts;
	}

	for (let [id, fact] of entries) {
		if (is_fact_opened(fact)) {
			opened_facts.add(id);
		}
	}
	return opened_facts;
}

export function export_save_to_browser_url(keys, opened) {
	let encoded = encode_save(keys, opened);
	// save version for now
	window.location.hash = `v=${GAME_VERSION}&ev=${ENCODING_VERSION}&save=${encoded}`;
}

export function get_save_from_browser_url(keys) {
	let h = window.location.hash;
	let encoded = h.split("&save=")[1];
	// to not break early links. todo: remove
	if (h.startsWith("#save")) {
		encoded = h.split("save=")[1];
	}
	let opened = decode_save(keys, encoded);
	return opened;
}

export function has_save_in_url() {
	let h = window.location.hash;
	return (
		h.includes("save=") && h.split("save=")[1].length == ENCODED_SAVE_LEN
	);
}

/**
 * @param  {string[]} keys
 * @param  {Set<string>} opened
 * @return {string|null}
 */
export function encode_save(keys, opened) {
	keys = keys.sort();
	let keys_count = keys.length;
	if (keys_count != KEYS_COUNT && keys_count > 0) {
		console.error(
			"trying to load save with wrong number of keys:",
			keys_count,
			"expected",
			KEYS_COUNT,
		);
		return null;
	}

	let booled = [];
	if (keys_count > 0) {
		booled = keys.map((id) => opened.has(id));
	} else {
		// empty save
		booled = Array.from({ length: KEYS_COUNT }, () => false);
	}
	let packed = pack_bools(booled);
	return btoa(String.fromCharCode(...packed));
}

/**
 * @param  {string[]} keys
 * @param  {string} encoded
 * @return {Set<string>|null}
 */
export function decode_save(keys, encoded) {
	let bytes = [...atob(encoded)].map((char) => char.charCodeAt(0));
	let unpacked = unpack_bools(bytes);
	keys = keys.sort();
	let keys_count = keys.length;
	let unpacked_count = unpacked.length;
	// can't check !== because at the end can be padding of zeroes
	if (unpacked_count < keys_count) {
		console.error(
			"trying to load save with wrong number of keys:",
			unpacked_count,
			"expected",
			keys_count,
		);
		return null;
	}

	let opened = new Set();
	for (let [i, key] of Object.entries(keys)) {
		if (unpacked[i]) {
			opened.add(key);
		}
	}
	return opened;
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
