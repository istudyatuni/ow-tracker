// i guess global state shouldn't work this way..
// todo: rewrite

const RUMOR_REGEX = /_R\d+$/;

const MORE_TO_EXPLORE_TR = "MORE_TO_EXPLORE";

// cache of facts
let opened_facts_by_id = {};
let opened_facts = new Set();
let opened_cards_only_rumors = new Set();
let has_unexplored_cards = new Set();
let entries_facts = {};
/** @type {Object.<string, string>} */
let tr = {};
let joined_rumors = {};

export async function set_opened_facts(data) {
	opened_facts = data;
}

export async function set_opened_cards_only_rumors(data) {
	opened_cards_only_rumors = data;
}

export async function set_has_unexplored_cards(data) {
	has_unexplored_cards = data;
}

export async function set_entries_facts(data) {
	entries_facts = data;
}

export async function set_joined_rumors(data) {
	joined_rumors = data;
}

/**
 * @param  {string} id
 * @return {string[]}
 */
export function get_facts_for(id) {
	let is_joined = id.includes(",");

	if (id.match(RUMOR_REGEX)) {
		// clicked on rumor
		return [tr[id]];
	}

	if (opened_facts_by_id[id] !== undefined) {
		return opened_facts_by_id[id];
	}

	let facts;
	if (opened_cards_only_rumors.has(id)) {
		facts = entries_facts[id]?.rumor;
	} else if (is_joined) {
		facts = joined_rumors[id]?.rumors;
	} else {
		facts = entries_facts[id]?.explore;
	}

	// when vite reloads this file in dev mode, site breaks
	if (import.meta.env.DEV) {
		if (facts === undefined) {
			return [];
		}
	}

	opened_facts_by_id[id] = facts
		.filter((f) => opened_facts.has(f))
		.map((f) => tr[f]);
	return opened_facts_by_id[id];
}

export function has_more_to_explore(id) {
	return !opened_cards_only_rumors.has(id) && has_unexplored_cards.has(id);
}

export function get_more_to_explore_tr() {
	return tr[MORE_TO_EXPLORE_TR];
}

/**
 * @param  {string} lang
 */
export async function load_tr(lang) {
	tr = await (
		await fetch(`${import.meta.env.BASE_URL}/translations/${lang}.json`)
	).json();
	return tr;
}
