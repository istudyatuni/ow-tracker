/** @import {LatLngTuple} from  "leaflet" */

import { get } from "svelte/store";

import { expand_thin_bounds } from "@/lib/arrow";
import { CARD_HEIGHT, CARD_WIDTH, STAR_SIZE } from "@/lib/card";
import {
	CATEGORIES,
	CATEGORY,
	CURIOSITY,
	curiosity_to_category,
} from "@/lib/categories";
import {
	load_tr,
	set_entries_facts,
	set_joined_rumors,
	set_has_unexplored_cards,
	set_opened_cards_only_rumors,
	set_opened_facts,
} from "@/lib/data";
import { detect_language } from "@/lib/language";
import { coord_to_leaflet } from "@/lib/leaflet";
import { get_save_from_browser_url, has_save_in_url } from "@/lib/saves";
import {
	LOADING,
	LOADING_TOTAL,
	MAP_SIZE,
	OPENED_FACTS_COUNT,
	SAVE_EMPTY,
	SAVE_FOUND,
	SAVE_FOUND_CATEGORIES,
	SAVE_KNOWN_CATEGORIES_NAMES,
	SESSION_SETTINGS,
	SETTINGS,
} from "@/lib/stores";

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
const RUMOR_PANE = "mapPane";
const NORMAL_PANE = "overlayPane";
const SMALL_PANE = "markerPane";

/**
 * @param {Array}  entries
 * @param {Object} result
 */
function flatten_entries(entries, result) {
	for (let e of entries || []) {
		result[e.id] = e;
		flatten_entries(e.entries, result);
	}
	return result;
}

export async function generate_all_svg() {
	let save_loaded = has_save_in_url();
	SAVE_FOUND.set(save_loaded);

	let consider_ignored = get(SETTINGS).consider_ignored_facts;

	// number of fetches before fetching images
	LOADING_TOTAL.set(4);
	LOADING.set(0);

	let save_keys = await (
		await fetch(import.meta.env.BASE_URL + "/save_keys.json")
	).json();
	let opened_facts;
	if (save_loaded) {
		opened_facts = get_save_from_browser_url(save_keys);
	} else {
		opened_facts = new Set(save_keys);
	}
	set_opened_facts(opened_facts);
	OPENED_FACTS_COUNT.set(opened_facts.size);

	LOADING.update((n) => n + 1);

	/**
	 * Load ids data and rumors source ids.
	 *
	 * @type {Object<string, { curiosity: string }>}
	 */
	let library = {};
	/**
	 * `rumor's source id -> [{entry_id, rumor_id}]`
	 *
	 * @type {Object<string, Object<string, string>[]>}
	 */
	let sources = {};
	let entries_data = await (
		await fetch(import.meta.env.BASE_URL + "/entries.json")
	).json();
	let entries = flatten_entries(entries_data, {});
	// opened cards ids
	let opened_cards = new Set();
	// cards ids where img is opened
	let opened_card_imgs = new Set();
	/**
	 * Facts by entry id
	 *
	 * `entry id -> [{ rumor_id, explore_id }]`
	 *
	 * @type {Object<string, { rumor: string[]; explore: string[] }>}
	 */
	let entries_facts = {};

	/**
	 * rumors which should be shown on the same arrow
	 *
	 * `[entry1_id, entry2_id] -> [rumor id]`
	 *
	 * @type {Object<string, { entries: string[]; rumors: string[] }>}
	 */
	let joined_rumors = {};

	// cards where not all explore facts are opened
	let has_unexplored_cards = new Set();

	/**
	 * Cards alternative names
	 *
	 * `entry_id -> alt_name_id`
	 *
	 * @type {Object<string, string>}
	 */
	let cards_alt_names = {};

	// cards with which categories opened in save
	let cards_categories_in_save = new Set();

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
			curiosity: e.curiosity || CURIOSITY.OTHER,
		};

		let rumor_facts = [];
		let explore_facts = [];

		for (let fact of e?.facts?.explore || []) {
			if (
				!opened_facts.has(fact.id) &&
				(consider_ignored ||
					(!e.ignore_more_to_explore && !fact.ignore_more_to_explore))
			) {
				has_unexplored_cards.add(e.id);
			}
			explore_facts.push(fact.id);
		}

		let last_name_priority = -1;
		for (let fact of e?.facts?.rumor || []) {
			if (opened_facts.has(fact.id)) {
				// remember rumors on same arrow
				if (fact.source_id !== undefined) {
					let entries = [e.id, fact.source_id].sort();
					let key = entries.join(",");
					if (joined_rumors[key] !== undefined) {
						joined_rumors[key].rumors.push(fact.id);
					} else {
						joined_rumors[key] = {
							entries,
							rumors: [fact.id],
						};
					}
				}

				// not all facts with name_id has name priority, so use 0 in this case
				let name_priority = fact.name_priority || 0;
				// remember card alternative name
				if (
					fact.name_id !== undefined &&
					name_priority > last_name_priority
				) {
					cards_alt_names[e.id] = fact.name_id;
					last_name_priority = name_priority;
				}
			} else if (
				// todo: this check is still incomplete
				consider_ignored ||
				(!opened_card_imgs.has(e.id) &&
					// fix for TH_VILLAGE
					!entries[fact.source_id]?.ignore_more_to_explore &&
					// not sure about it
					!fact.ignore_more_to_explore)
			) {
				has_unexplored_cards.add(fact.source_id);
			}
			rumor_facts.push(fact.id);
		}
		entries_facts[e.id] = {
			rumor: rumor_facts,
			explore: explore_facts,
		};

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

		// fill cards categories in save
		if (opened_cards.has(e.id)) {
			cards_categories_in_save.add(curiosity_to_category(e.curiosity));
		}
	}

	// leave only when >= 2 rumors on same arrow
	for (let [key, value] of Object.entries(joined_rumors)) {
		if (value.rumors.length <= 1) {
			delete joined_rumors[key];
		}
	}

	// categories with known names (e.g card with the same name is opened)
	let categories_known_names = new Set(CATEGORIES);
	if (save_loaded) {
		categories_known_names = new Set([CATEGORY.OTHER]);

		function check_category_known_name(id, category) {
			if (opened_cards.has(id)) {
				categories_known_names.add(category);
			}
		}
		{
			let id = "ORBITAL_PROBE_CANNON";
			if (
				opened_cards.has(id) &&
				(cards_alt_names[id] === undefined ||
					cards_alt_names[id] === id)
			) {
				categories_known_names.add(CATEGORY.ORBITAL_CANON);
			}
		}
		check_category_known_name("QUANTUM_MOON", CATEGORY.QUANTUM_MOON);
		check_category_known_name("DB_VESSEL", CATEGORY.VESSEL);
		check_category_known_name(
			"TT_TIME_LOOP_DEVICE",
			CATEGORY.ASH_TWIN_PROJECT,
		);
		check_category_known_name("IP_RING_WORLD", CATEGORY.STRANGER);
		if (opened_card_imgs.has("COMET_INTERIOR")) {
			categories_known_names.add(CATEGORY.NOMAI_FATE);
		}
	}

	set_opened_cards_only_rumors(opened_cards.difference(opened_card_imgs));
	set_entries_facts(entries_facts);
	set_joined_rumors(joined_rumors);
	set_has_unexplored_cards(has_unexplored_cards);

	SAVE_FOUND_CATEGORIES.set(cards_categories_in_save);
	SAVE_KNOWN_CATEGORIES_NAMES.set(categories_known_names);
	SAVE_EMPTY.set(opened_cards.size === 0);

	if (!get(SESSION_SETTINGS).welcome_popup_done) {
		LOADING.set(null);
		return function* () {};
	}

	cards_alt_names = Object.fromEntries(
		Object.entries(cards_alt_names).filter(
			([id, _]) => !opened_card_imgs.has(id),
		),
	);

	LOADING.update((n) => n + 1);

	/**
	 * Load coordinates and images.
	 *
	 * @type {Object<
	 * 	string,
	 * 	{ coordinates: LatLngTuple; sprite: string | null }
	 * >}
	 */
	let cards = {};
	let [minX, maxX, minY, maxY] = [4000, -1000, 2000, -2000];
	let coordinates_data = await (
		await fetch(import.meta.env.BASE_URL + "/coordinates.json")
	).json();
	for (let [id, [x, y]] of Object.entries(coordinates_data)) {
		if (opened_cards.has(id)) {
			minX = Math.min(minX, x);
			minY = Math.min(minY, y);
			maxX = Math.max(maxX, x);
			maxY = Math.max(maxY, y);
		}
		cards[id] = {
			coordinates: coord_to_leaflet(x, y),
			sprite: opened_card_imgs.has(id)
				? `${import.meta.env.BASE_URL}/sprites/${id}.jpg`
				: null,
		};
	}
	MAP_SIZE.set([
		[minX, minY],
		[maxX, maxY],
	]);

	LOADING.update((n) => n + 1);

	/** @type {Object<string, string>} */
	let parents = await (
		await fetch(import.meta.env.BASE_URL + "/parents.json")
	).json();

	// load translations
	let lang = detect_language();
	LOADING.set(null);

	/** @type {Object<string, string>} */
	let tr = await load_tr(lang);

	return function* () {
		// centers is filled inside of generate_cards
		/** @type {Object<string, LatLngTuple>} */
		let centers = {};
		yield* generate_cards(
			cards,
			library,
			parents,
			centers,
			opened_cards,
			has_unexplored_cards,
			tr,
			cards_alt_names,
			save_loaded,
		);
		yield* generate_arrows(
			sources,
			library,
			opened_cards,
			opened_facts,
			centers,
			joined_rumors,
			save_loaded,
		);
	};
}

/**
 * @param {Object<
 * 	string,
 * 	{ coordinates: LatLngTuple; sprite: string | null }
 * >} cards
 * @param {Object<string, { curiosity: string }>} library
 * @param {Object<string, string>} parents
 * @param {Object<string, LatLngTuple>} centers
 * @param {Set<string>} opened_cards
 * @param {Set<string>} has_unexplored_cards
 * @param {Object<string, string>} tr
 * @param {Object<string, string>} cards_alt_names
 * @param {boolean} save_loaded
 * @yields
 */
function* generate_cards(
	cards,
	library,
	parents,
	centers,
	opened_cards,
	has_unexplored_cards,
	tr,
	cards_alt_names,
	save_loaded,
) {
	for (let [id, card] of Object.entries(cards)) {
		let curiosity = library[id].curiosity;

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

		let has_unexplored = has_unexplored_cards.has(id);
		if (has_unexplored) {
			// increase when "more to explore" star is displayed
			w += STAR_SIZE * 2;
		}
		let start_bounds = [cx - h / 2, cy - w / 2];
		let end_bounds = [cx + h / 2, cy + w / 2];

		if (save_loaded && !opened_cards.has(id)) {
			continue;
		}

		let tr_id = cards_alt_names[id] || id;
		yield {
			// options for MapCard
			options: {
				is_arrow: false,
				id,
				text: tr[tr_id]
					.replaceAll("@@", "<br/>")
					.replaceAll("$$", "-<br/>"),
				image_url: card.sprite,
				has_unexplored,
				category_class: curiosity_to_category(curiosity),
			},
			coords: [start_bounds, end_bounds],
			pane: is_small ? SMALL_PANE : NORMAL_PANE,
		};
	}
}

/**
 * @param {Object<string, Object<string, string>[]>} sources
 * @param {Object<string, { curiosity: string }>} library
 * @param {Set<string>} opened_cards
 * @param {Set<string>} opened_facts
 * @param {Object<string, LatLngTuple>} centers
 * @param {Object<string, { entries: string[]; rumors: string[] }>} joined_rumors
 * @param {boolean} save_loaded
 * @yields
 */
function* generate_arrows(
	sources,
	library,
	opened_cards,
	opened_facts,
	centers,
	joined_rumors,
	save_loaded,
) {
	for (let [source_id, entry_ids] of Object.entries(sources)) {
		let source_curiosity = library[source_id].curiosity;

		if (save_loaded && !opened_cards.has(source_id)) {
			continue;
		}
		for (let { entry_id, rumor_id } of entry_ids) {
			let entry_curiosity = library[entry_id].curiosity;

			let key = [source_id, entry_id].sort().join(",");
			let rumors = joined_rumors[key]?.rumors;
			let should_join = rumors !== undefined;
			let is_not_first_rumor = should_join && rumor_id !== rumors[0];
			// draw line only for first rumor
			if (is_not_first_rumor) {
				continue;
			}

			if (save_loaded && !opened_facts.has(rumor_id)) {
				continue;
			}
			yield {
				// options for MapArrow
				options: {
					is_arrow: true,
					id: should_join ? key : rumor_id,
					center1: centers[source_id],
					center2: centers[entry_id],
					// use both categories as class names so that the arrow is
					// hidden if either of these categories is hidden
					category_class: [source_curiosity, entry_curiosity]
						.map(curiosity_to_category)
						.join(" "),
				},
				coords: expand_thin_bounds([
					centers[source_id],
					centers[entry_id],
				]),
				pane: RUMOR_PANE,
			};
		}
	}
}
