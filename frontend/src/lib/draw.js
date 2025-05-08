import { expand_thin_bounds, make_rumor_arrow } from './arrow';
import { CARD_HEIGHT, CARD_WIDTH, make_card_svg, STAR_SIZE } from './card';
import { category_to_curiosity, CURIOSITY, curiosity_to_category, should_show_curiosity } from './categories';
import { load_tr, set_entries_facts, set_joined_rumors, set_has_unexplored_cards, set_opened_cards_only_rumors, set_opened_facts } from './data';
import { to_data_url } from './dataurl';
import { detect_language } from './language';
import { get_save_from_browser_url } from './saves';
import { LOADING, MAP_SIZE, SAVE_FOUND, SAVE_FOUND_CATEGORIES, SELECTED_CATEGORIES, SETTINGS } from './stores';
import { t as i18n } from './i18n';
import { get } from 'svelte/store';

const DEFAULT_MULT = 0.7;
const BIG_MULT = 1.2;
const SMALL_MULT = 0.4;

const BIG_CARDS = new Set([
	"COMET_INTERIOR",
	"DB_VESSEL",
	"IP_RING_WORLD",
	"ORBITAL_PROBE_CANNON",
	"QUANTUM_MOON",
	"TT_TIME_LOOP_DEVICE",
]);

// pane names doesn't mean anything here, just panes with increasing z-index
const RUMOR_PANE = 'mapPane'
const NORMAL_PANE = 'overlayPane'
const SMALL_PANE = 'markerPane'

/**
 * @param  {Array} entries
 * @param  {Object} result
 */
function flatten_entries(entries, result) {
	for (let e of entries || []) {
		result[e.id] = e
		flatten_entries(e.entries, result)
	}
	return result
}

export async function* generate_all_svg() {
	let save_loaded = window.location.hash !== ''
	SAVE_FOUND.set(save_loaded)

	let t = get(i18n)
	let hide_curiosities = new Set(Object.entries(get(SELECTED_CATEGORIES))
		.filter(([_, enabled]) => !enabled)
		.map(([category, _]) => category_to_curiosity(category)))

	LOADING.set(t('loading-stage-save-keys'))

	let save_keys = await (await fetch(import.meta.env.BASE_URL + "/save_keys.json")).json();
	let opened_facts
	if (save_loaded) {
		opened_facts = get_save_from_browser_url(save_keys)
	} else {
		opened_facts = new Set(save_keys)
	}
	set_opened_facts(opened_facts)

	LOADING.set(t('loading-stage-connections-data'))

	/**
	 * load ids data and rumors source ids
	 * @type {Object.<string, { curiosity: string }>}
	 */
	let library = {};
	/**
	 * rumor's source id -> [{entry_id, rumor_id}]
	 * @type {Object.<string, Object.<string, string>[]>}
	 */
	let sources = {};
	let entries_data = await (await fetch(import.meta.env.BASE_URL + "/entries.json")).json();
	let entries = flatten_entries(entries_data, {})
	// opened cards ids
	let opened_cards = new Set();
	// cards ids where img is opened
	let opened_card_imgs = new Set();
	//
	//
	/**
	 * facts by entry id
	 *
	 * `entry id -> [{ rumor_id, explore_id }]`
	 * @type {Object.<string, { rumor: string[], explore: string[] }>}
	 */
	let entries_facts = {}

	// rumors which should be shown on the same arrow
	// [entry1_id, entry2_id] -> [rumor id]
	let joined_rumors = {}

	// cards where not all explore facts are opened, excluding ignore_more_to_explore
	let has_unexplored_cards = new Set()

	/**
	 * cards alternative names
	 *
	 * entry_id -> alt_name_id
	 * @type {Object.<string, string>}
	 */
	let cards_alt_names = {}

	let found_categories = new Set()

	// fill opened_cards and opened_card_imgs
	for (let e of Object.values(entries)) {
		for (let fact of e?.facts?.explore || []) {
			if (opened_facts.has(fact.id)) {
				opened_cards.add(e.id);
				opened_card_imgs.add(e.id);
			}
		}
		for (let fact of e?.facts?.rumor || []) {
			if (opened_facts.has(fact.id)) {
				opened_cards.add(e.id);
			}
		}
	}

	for (let e of Object.values(entries)) {
		library[e.id] = {
			curiosity: e.curiosity,
		};

		let rumor_facts = []
		let explore_facts = []

		for (let fact of e?.facts?.explore || []) {
			if (
				!opened_facts.has(fact.id)
				&& !e.ignore_more_to_explore
				&& !fact.ignore_more_to_explore
			) {
				has_unexplored_cards.add(e.id)
			}
			explore_facts.push(fact.id)
		}

		let last_name_priority = -1
		for (let fact of e?.facts?.rumor || []) {
			if (opened_facts.has(fact.id)) {
				// remember rumors on same arrow
				if (fact.source_id !== undefined) {
					let key = [e.id, fact.source_id].sort()
					if (joined_rumors[key] !== undefined) {
						joined_rumors[key].rumors.push(fact.id);
					} else {
						joined_rumors[key] = {
							entries: key,
							rumors: [fact.id],
						}
					}
				}

				// not all facts with name_id has name priority, so use 0 in this case
				let name_priority = fact.name_priority || 0
				// remember card alternative name
				if (fact.name_id !== undefined && name_priority > last_name_priority) {
					cards_alt_names[e.id] = fact.name_id
					last_name_priority = name_priority
				}
			} else if (
				// todo: this check is still incomplete
				// fix for TH_VILLAGE
				!entries[fact.source_id]?.ignore_more_to_explore
				// not sure about it
				&& !fact.ignore_more_to_explore
				&& !opened_card_imgs.has(e.id)
			) {
				has_unexplored_cards.add(fact.source_id)
			}
			rumor_facts.push(fact.id)
		}
		entries_facts[e.id] = {
			rumor: rumor_facts,
			explore: explore_facts,
		}

		// fill source_ids
		for (let fact of e?.facts?.rumor || []) {
			if (fact.source_id === undefined) {
				continue;
			}

			let obj = {
				entry_id: e.id,
				rumor_id: fact.id,
			};
			if (sources[fact.source_id] !== undefined) {
				sources[fact.source_id].push(obj);
			} else {
				sources[fact.source_id] = [obj];
			}
		}

		if (opened_cards.has(e.id)) {
			found_categories.add(curiosity_to_category(e.curiosity || CURIOSITY.COMET_CORE))
		}
	}

	// leave only when >= 2 rumors on same arrow
	for (let [key, value] of Object.entries(joined_rumors)) {
		if (value.rumors.length <= 1) {
			delete joined_rumors[key]
		}
	}

	set_opened_cards_only_rumors(opened_cards.difference(opened_card_imgs))
	set_entries_facts(entries_facts)
	set_joined_rumors(joined_rumors)
	set_has_unexplored_cards(has_unexplored_cards)

	SAVE_FOUND_CATEGORIES.set(found_categories)

	// hiding while some features not ready
	if (!get(SETTINGS).welcome_popup_done) {
		return []
	}

	cards_alt_names = Object.fromEntries(Object.entries(cards_alt_names).filter(([id, _]) => !opened_card_imgs.has(id)))

	LOADING.set(t('loading-stage-coordinates'))

	/**
	 * load coordinates and images
	 * @type {Object.<string, { coordinates: import('leaflet').LatLngTuple, sprite: string | null }>}
	 */
	let cards = {};
	let [minX, maxX, minY, maxY] = [4000, -1000, 2000, -2000]
	let coordinates_data = await (await fetch(import.meta.env.BASE_URL + "/coordinates.json")).json();
	for (let [id, [x, y]] of Object.entries(coordinates_data)) {
		if (opened_cards.has(id)) {
			minX = Math.min(minX, x)
			minY = Math.min(minY, y)
			maxX = Math.max(maxX, x)
			maxY = Math.max(maxY, y)
		}
		cards[id] = {
			coordinates: coord_to_leaflet(x, y),
			sprite: opened_card_imgs.has(id) ? `${import.meta.env.BASE_URL}/sprites/${id}.jpg` : null,
		};
	}
	MAP_SIZE.set([[minX, minY], [maxX, maxY]])

	LOADING.set(t('loading-stage-parents'))

	/** @type {Object.<string, string>} */
	let parents = await (await fetch(import.meta.env.BASE_URL + "/parents.json")).json();

	// load translations
	let lang = detect_language();
	LOADING.set(t('loading-stage-translation', { lang }))

	/** @type {Object.<string, string>} */
	let tr = await load_tr(lang);

	// centers is filled inside of generate_cards
	/** @type {Object.<string, import('leaflet').LatLngTuple>} */
	let centers = {};
	yield* generate_cards(cards, library, parents, centers, opened_cards, has_unexplored_cards, tr, cards_alt_names, hide_curiosities, save_loaded)
	yield* generate_arrows(sources, library, opened_cards, opened_facts, centers, joined_rumors, hide_curiosities, save_loaded)
}

/**
 * @param {Object.<string, { coordinates: import('leaflet').LatLngTuple, sprite: string | null }>} cards
 * @param {Object.<string, { curiosity: string }>} library
 * @param {Object.<string, string>} parents
 * @param {Object.<string, import('leaflet').LatLngTuple>} centers
 * @param {Set.<string>} opened_cards
 * @param {Set.<string>} has_unexplored_cards
 * @param {Object.<string, string>} tr
 * @param {Object.<string, string>} cards_alt_names
 * @param {Set.<string>} hide_curiosities
 * @param {boolean} save_loaded
 * @yield {}
 */
async function* generate_cards(
	cards,
	library,
	parents,
	centers,
	opened_cards,
	has_unexplored_cards,
	tr,
	cards_alt_names,
	hide_curiosities,
	save_loaded,
) {
	let t = get(i18n)

	for (let [id, card] of Object.entries(cards)) {
		let curiosity = library[id]?.curiosity

		let is_small = id in parents;
		let is_big = BIG_CARDS.has(id);
		let mult = DEFAULT_MULT;
		if (is_small) {
			mult = SMALL_MULT;
		} else if (is_big) {
			mult = BIG_MULT;
		}

		centers[id] = card.coordinates;

		let [cx, cy] = card.coordinates;
		let w = CARD_WIDTH * mult;
		let h = CARD_HEIGHT * mult;

		let has_unexplored = has_unexplored_cards.has(id)
		if (has_unexplored) {
			// increase when "more to explore" star is displayed
			w += STAR_SIZE * 2
		}
		let start_bounds = [cx - h / 2, cy - w / 2];
		let end_bounds = [cx + h / 2, cy + w / 2];

		if (should_show_curiosity(hide_curiosities, curiosity)) {
			continue;
		}
		if (save_loaded && !opened_cards.has(id)) {
			continue;
		}

		let img = await (async () => {
			if (card.sprite === null) {
				return null;
			}
			LOADING.set(t('loading-stage-sprite', { sprite: id }))
			let img = await (await fetch(card.sprite)).blob();
			return await to_data_url(img);
		})();
		let tr_id = cards_alt_names[id] || id
		let svg = make_card_svg(
			id,
			tr[tr_id].replaceAll("@@", "<br/>").replaceAll("$$", "-<br/>"),
			img,
			has_unexplored,
			curiosity_to_category(curiosity),
		);
		yield { svg, coords: [start_bounds, end_bounds], pane: is_small ? SMALL_PANE : NORMAL_PANE }
	}
}

/**
 * @param {Object.<string, Object.<string, string>[]>} sources
 * @param {Object.<string, { curiosity: string }>} library
 * @param {Set.<string>} opened_cards
 * @param {Set.<string>} opened_facts
 * @param {Object.<string, import('leaflet').LatLngTuple>} centers
 * @param {Set.<string>} hide_curiosities
 * @param {boolean} save_loaded
 * @yield {}
 */
function* generate_arrows(sources, library, opened_cards, opened_facts, centers, joined_rumors, hide_curiosities, save_loaded) {
	for (let [source_id, entry_ids] of Object.entries(sources)) {
		if (should_show_curiosity(hide_curiosities, library[source_id]?.curiosity)) {
			continue;
		}
		if (save_loaded && !opened_cards.has(source_id)) {
			continue;
		}
		for (let { entry_id, rumor_id } of entry_ids) {
			if (should_show_curiosity(hide_curiosities, library[entry_id]?.curiosity)) {
				continue;
			}

			let key = [source_id, entry_id].sort()
			// @ts-ignore
			let rumors = joined_rumors[key]?.rumors
			let should_join = rumors !== undefined
			let is_not_first_rumor = should_join && rumor_id !== rumors[0]
			// draw line only for first rumor
			if (is_not_first_rumor) {
				continue;
			}

			if (save_loaded && !opened_facts.has(rumor_id)) {
				continue;
			}
			let svg = make_rumor_arrow(
				should_join ? key.join(',') : rumor_id,
				centers[source_id],
				centers[entry_id],
			);
			let coords = expand_thin_bounds([centers[source_id], centers[entry_id]])
			yield { svg, coords, pane: RUMOR_PANE }
		}
	}
}

/** @return {import('leaflet').LatLngTuple} */
export function coord_to_leaflet(x, y) {
	const Y_CONV = 1;
	return [y * Y_CONV, x];
}
