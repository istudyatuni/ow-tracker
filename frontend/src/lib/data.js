// i guess global state shouldn't work this way..
// todo: rewrite

// cache of facts
let opened_facts_by_id = {}
let opened_facts = new Set()
let opened_cards_only_rumors = new Set()
let tr = {}

export async function set_opened_facts(data) {
	opened_facts = data
}

export async function set_opened_cards_only_rumors(data) {
	opened_cards_only_rumors = data
}

export function get_facts_for(id) {
	let parts = id.split('_')
	let last = parts[parts.length - 1]
	if (last.startsWith('R')) {
		// clicked on rumor
		return [tr[id]]
	}

	if (opened_facts_by_id[id] !== undefined) {
		return opened_facts_by_id[id]
	}

	let search_postfix_marker = opened_cards_only_rumors.has(id) ? 'R' : 'X'

	let facts = []
	let id_len = id.length
	for (let f of opened_facts) {
		if (f.startsWith(id) && f[id_len + 1] === search_postfix_marker) {
			facts.push(f)
		}
	}
	opened_facts_by_id[id] = facts.sort().map((f) => tr[f])
	return opened_facts_by_id[id]
}

export async function load_tr(lang) {
	tr = await (await fetch(`translations/${lang}.json`)).json()
	return tr
}
