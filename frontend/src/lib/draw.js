import { expand_thin_bounds, make_rumor_arrow } from './arrow';
import { CARD_HEIGHT, CARD_WIDTH, make_card_svg } from './card';
import { load_tr, set_entries_facts, set_joined_rumors, set_opened_cards_only_rumors, set_opened_facts } from './data';
import { to_data_url } from './dataurl';
import { CURIOSITY } from './info';
import { detect_language } from './language';
import { get_save_from_browser_url } from './saves';
import { LOADING, SAVE_FOUND } from './stores';

const HIDE_CURIOSITIES = [CURIOSITY.INVISIBLE_PLANET];

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

export async function* generate_all_svg() {
	let save_loaded = window.location.hash !== ''
	SAVE_FOUND.set(save_loaded)

	// hiding while some features not ready
	if (!save_loaded) {
		return []
	}

	LOADING.set('defined save keys')

	let save_keys = await (await fetch(import.meta.env.BASE_URL + "/save_keys.json")).json();
	let opened_facts
	if (save_loaded) {
		opened_facts = get_save_from_browser_url(save_keys)
	} else {
		opened_facts = new Set(save_keys)
	}
	set_opened_facts(opened_facts)

	LOADING.set('connections data')

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

	function handle_entries(entries) {
		for (let e of entries || []) {
			library[e.id] = {
				curiosity: e.curiosity,
			};

			let rumor_facts = []
			let explore_facts = []

			// fill opened_cards and opened_card_imgs
			for (let fact of e?.facts?.explore || []) {
				if (opened_facts.has(fact.id)) {
					opened_cards.add(e.id);
					opened_card_imgs.add(e.id);
				}
				explore_facts.push(fact.id)
			}
			for (let fact of e?.facts?.rumor || []) {
				if (opened_facts.has(fact.id)) {
					opened_cards.add(e.id);

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
			handle_entries(e.entries);
		}
	}
	handle_entries(entries_data);

	// leave only when >= 2 rumors on same arrow
	for (let [key, value] of Object.entries(joined_rumors)) {
		if (value.rumors.length <= 1) {
			delete joined_rumors[key]
		}
	}

	set_opened_cards_only_rumors(opened_cards.difference(opened_card_imgs))
	set_entries_facts(entries_facts)
	set_joined_rumors(joined_rumors)

	LOADING.set('coordinates')

	/**
	 * load coordinates and images
	 * @type {Object.<string, { coordinates: import('leaflet').LatLngTuple, sprite: string | null }>}
	 */
	let entries = {};
	let coordinates_data = await (await fetch(import.meta.env.BASE_URL + "/coordinates.json")).json();
	for (let [id, [x, y]] of Object.entries(coordinates_data)) {
		entries[id] = {
			coordinates: coord_to_leaflet(x, y),
			sprite: opened_card_imgs.has(id) ? `${import.meta.env.BASE_URL}/sprites/${id}.jpg` : null,
		};
	}

	LOADING.set('parents')

	/** @type {Object.<string, string>} */
	let parents = await (await fetch(import.meta.env.BASE_URL + "/parents.json")).json();

	// load translations
	let lang = detect_language();
	LOADING.set(`translation for ${lang}`)

	/** @type {Object.<string, string>} */
	let tr = await load_tr(lang);

	LOADING.set('theme')

	/** @type {Object.<string, { color: string, highlight: string }>} */
	let theme = await (await fetch(import.meta.env.BASE_URL + "/theme.json")).json();

	// centers is filled inside of generate_cards
	/** @type {Object.<string, import('leaflet').LatLngTuple>} */
	let centers = {};
	yield* generate_cards(entries, theme, library, parents, centers, opened_cards, tr, save_loaded)
	yield* generate_arrows(sources, library, opened_cards, opened_facts, centers, joined_rumors, save_loaded)
}

/**
 * @param {Object.<string, { coordinates: import('leaflet').LatLngTuple, sprite: string | null }>} entries
 * @param {Object.<string, { color: string, highlight: string }>} theme
 * @param {Object.<string, { curiosity: string }>} library
 * @param {Object.<string, string>} parents
 * @param {Object.<string, import('leaflet').LatLngTuple>} centers
 * @param {Set.<string>} opened_cards
 * @param {Object.<string, string>} tr
 * @param {boolean} save_loaded
 * @yield {}
 */
async function* generate_cards(entries, theme, library, parents, centers, opened_cards, tr, save_loaded) {
	for (let [id, e] of Object.entries(entries)) {
		let colors = theme[library[id]?.curiosity] || theme.neutral;

		let is_small = id in parents;
		let is_big = BIG_CARDS.has(id);
		let mult = DEFAULT_MULT;
		if (is_small) {
			mult = SMALL_MULT;
		} else if (is_big) {
			mult = BIG_MULT;
		}

		centers[id] = e.coordinates;

		let [cx, cy] = e.coordinates;
		let w = CARD_WIDTH * mult;
		let h = CARD_HEIGHT * mult;
		let start_bounds = [cx - h / 2, cy - w / 2];
		let end_bounds = [cx + h / 2, cy + w / 2];

		if (!save_loaded && HIDE_CURIOSITIES.includes(library[id]?.curiosity)) {
			continue;
		}
		if (save_loaded && !opened_cards.has(id)) {
			continue;
		}

		let img = await (async () => {
			if (e.sprite === null) {
				return null;
			}
			LOADING.set(`sprite for ${id}`)
			let img = await (await fetch(e.sprite)).blob();
			return await to_data_url(img);
		})();
		let svg = make_card_svg(
			id,
			tr[id].replaceAll("@@", "<br/>").replaceAll("$$", "-<br/>"),
			img,
			colors?.color,
			colors?.highlight,
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
 * @param {boolean} save_loaded
 * @yield {}
 */
function* generate_arrows(sources, library, opened_cards, opened_facts, centers, joined_rumors, save_loaded) {
	for (let [source_id, entry_ids] of Object.entries(sources)) {
		if (!save_loaded &&
			HIDE_CURIOSITIES.includes(library[source_id]?.curiosity)) {
			continue;
		}
		if (save_loaded && !opened_cards.has(source_id)) {
			continue;
		}
		for (let { entry_id, rumor_id } of entry_ids) {
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
