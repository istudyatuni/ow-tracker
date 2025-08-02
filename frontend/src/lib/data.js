// i guess global state shouldn't work this way..
// todo: rewrite
// todo: why set_ functions are async?

const RUMOR_REGEX = /_R\d+$/;

const MORE_TO_EXPLORE_TR = "MORE_TO_EXPLORE";

let opened_facts = new Set();
let opened_cards_only_rumors = new Set();
let has_unexplored_cards = new Set();
let entries_facts = {};
/** @type {Object<string, string>} */
let tr = {};
/** @type {Object<string, { entries: string[]; rumors: string[] }>} */
let joined_rumors = {};

// used and filled only in debug
let all_save_keys = new Set();
let all_save_keys_list = [];

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

export async function set_all_save_keys(data) {
	console.assert(import.meta.env.DEV, "can be set only in dev mode");
	all_save_keys = new Set(data);
	all_save_keys_list = data;
}

export function get_all_save_keys(data) {
	return [...all_save_keys_list];
}

export function save_key_valid(key) {
	return all_save_keys.has(key);
}

/** @param {Object<string, { entries: string[]; rumors: string[] }>} data */
export async function set_joined_rumors(data) {
	joined_rumors = data;
}

/**
 * @param {string}  id
 * @param {boolean} show_unexplored
 * @returns {{ text: string; more_to_explore: boolean }[]}
 */
export function get_facts_for(id, show_unexplored) {
	return get_facts_ids_for(id, show_unexplored).map((fact) => ({
		text: tr[fact.id],
		more_to_explore: fact.more_to_explore || false,
	}));
}

/**
 * @param {string}  id
 * @param {boolean} show_unexplored
 * @returns {{ id: string; more_to_explore?: boolean }[]}
 */
export function get_facts_ids_for(id, show_unexplored) {
	let is_joined = id.includes(",");

	if (id.match(RUMOR_REGEX)) {
		// clicked on rumor
		return [{ id }];
	}

	let facts;
	if (opened_cards_only_rumors.has(id)) {
		facts = entries_facts[id]?.rumor;
	} else if (is_joined) {
		facts = joined_rumors[id]?.rumors;
	} else {
		facts = entries_facts[id]?.explore;
	}
	if (!show_unexplored) {
		facts = facts.filter((f) => opened_facts.has(f));
	}

	// when vite reloads this file in dev mode, site breaks
	if (import.meta.env.DEV) {
		if (facts === undefined) {
			return [];
		}
	}

	facts = facts.map((f) => {
		return { id: f, more_to_explore: !opened_facts.has(f) };
	});

	if (has_more_to_explore(id) && !show_unexplored) {
		facts.push({
			id: MORE_TO_EXPLORE_TR,
			more_to_explore: true,
		});
	}

	return facts;
}

function has_more_to_explore(id) {
	return !opened_cards_only_rumors.has(id) && has_unexplored_cards.has(id);
}

/** @param {string} lang */
export async function load_tr(lang) {
	tr = await (
		await fetch(`${import.meta.env.BASE_URL}/translations/${lang}.json`)
	).json();
	return tr;
}
