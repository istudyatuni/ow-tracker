// i guess global state shouldn't work this way..
// todo: rewrite

const RUMOR_REGEX = /_R\d+$/

// cache of facts
let opened_facts_by_id = {}
let opened_facts = new Set()
let opened_cards_only_rumors = new Set()
let entries_facts = {}
/** @type {Object.<string, string>} */
let tr = {}

export async function set_opened_facts(data) {
	opened_facts = data
}

export async function set_opened_cards_only_rumors(data) {
	opened_cards_only_rumors = data
}

export async function set_entries_facts(data) {
	entries_facts = data
}

/**
 * @param  {string} id
 * @return {string[]}
 */
export function get_facts_for(id) {
	if (id.match(RUMOR_REGEX)) {
		// clicked on rumor
		return [tr[id]]
	}

	if (opened_facts_by_id[id] !== undefined) {
		return opened_facts_by_id[id]
	}

	let facts = opened_cards_only_rumors.has(id) ? entries_facts[id]?.rumor : entries_facts[id]?.explore

	// when vite reloads this file in dev mode, site breaks
	if (import.meta.env.DEV) {
		if (facts === undefined) {
			return []
		}
	}

	opened_facts_by_id[id] = facts.filter((f) => opened_facts.has(f)).map((f) => tr[f])
	return opened_facts_by_id[id]
}

/**
 * @param  {string} lang
 */
export async function load_tr(lang) {
	tr = await (await fetch(`${import.meta.env.BASE_URL}/translations/${lang}.json`)).json()
	return tr
}
