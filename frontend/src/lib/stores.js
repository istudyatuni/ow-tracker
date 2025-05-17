/** @import { SettingsStore, SessionSettingsStore } from "." */
/** @import { WritableKV } from "svelte-storages" */

import { get, writable } from "svelte/store";

import { localStore, sessionStore } from "svelte-storages";

import {
	CATEGORIES,
	CATEGORY,
	default_enabled_categories,
} from "@/lib/categories";
import { detect_language } from "@/lib/language";

export const LOADING = writable(null);
export const LOADING_TOTAL = writable(0);
export const LOADING_STAGE = writable("data");

export const OPENED_FACT = writable(null);
export const OPENED_FACTS_COUNT = writable(0);
export const SAVE_FOUND = writable(null);
export const LANGUAGE = writable(detect_language());
export const SELECTED_CATEGORIES = localStore(
	"show-categories",
	default_enabled_categories(),
);
export const SAVE_FOUND_CATEGORIES = writable(new Set());
export const SAVE_KNOWN_CATEGORIES_NAMES = writable(new Set());
export const SAVE_EMPTY = writable(false);

/** @type {SettingsStore} */
const DEFAULT_SETTINGS = {
	version: 7,
	selected_categories_version: 1,
	hide_spoilers: true,
	hide_dlc: false,
	consider_ignored_facts: false,
	show_ignored_facts: false,
};
/** @type {SessionSettingsStore} */
const DEFAULT_SESSION_SETTINGS = {
	version: 1,
	welcome_popup_done: false,
};

/** @type {WritableKV<SettingsStore>} */
export const SETTINGS = localStore("ow-settings", DEFAULT_SETTINGS);
/** @type {WritableKV<SessionSettingsStore>} */
export const SESSION_SETTINGS = sessionStore(
	"ow-settings",
	DEFAULT_SESSION_SETTINGS,
);

// max/min in normal coordinates:
// x: [-878, 3341.8005]
// y: [-1577, 1707]
export const MAP_SIZE = writable([
	[-900, -1600],
	[3300, 1700],
]);

export function hide_dlc_if_necessary() {
	if (get(SETTINGS).hide_dlc) {
		SELECTED_CATEGORIES.set(CATEGORY.STRANGER, false);
	}
}

export function reset_selected_categories() {
	for (let c of CATEGORIES) {
		SELECTED_CATEGORIES.set(c, true);
	}
}

export function open_fact(id) {
	OPENED_FACT.set(id);
}

export function close_fact() {
	OPENED_FACT.set(null);
}

export function migrate_storage() {
	function migrate(store, default_kv) {
		let s = get(store);
		if (s.version < default_kv.version) {
			let old_keys = new Set(Object.keys(s));
			let new_keys = new Set(Object.keys(default_kv));

			old_keys.difference(new_keys).forEach((k) => store.delete(k));
			new_keys
				.difference(old_keys)
				.forEach((k) => store.set(k, default_kv[k]));

			store.set("version", default_kv.version);
		}
	}

	migrate(SETTINGS, DEFAULT_SETTINGS);
	migrate(SESSION_SETTINGS, DEFAULT_SESSION_SETTINGS);

	let s = get(SETTINGS);
	if (
		s.selected_categories_version <
		DEFAULT_SETTINGS.selected_categories_version
	) {
		let c = get(SELECTED_CATEGORIES);
		Object.keys(c).forEach((c) => SELECTED_CATEGORIES.delete(c));
		Object.entries(default_enabled_categories()).forEach(([k, v]) =>
			SELECTED_CATEGORIES.set(k, v),
		);
	}
}
