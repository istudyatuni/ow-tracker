import { derived, get, writable } from "svelte/store";

import { localStore, sessionStore } from "svelte-storages";

import { CATEGORIES, default_enabled_categories } from "./categories";
import { detect_language } from "./language";

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
export const MAP_EMPTY = writable(false);

const DEFAULT_SETTINGS = {
	version: 5,
	hide_spoilers: true,
	consider_ignored_facts: false,
	show_ignored_facts: false,
};
const DEFAULT_SESSION_SETTINGS = {
	version: 1,
	welcome_popup_done: false,
};
export const SETTINGS = localStore("ow-settings", DEFAULT_SETTINGS);
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

/**
 * @type {import("svelte/store").Writable.<import("@fluent/bundle").FluentBundle> | null}
 */
const tr_bundle = writable(null);

export const translator = derived(tr_bundle, (bundle) => (id, args = {}) => {
	if (bundle === null) {
		return "";
	}

	let msg = bundle.getMessage(id);
	if (msg?.value) {
		return bundle.formatPattern(msg.value, args);
	}
	console.warn("no value for message with id", id);
	return id;
});

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

/**
 * @param {import("@fluent/bundle").FluentBundle} bundle
 */
export function set_tr_bundle(bundle) {
	tr_bundle.set(bundle);
}

SETTINGS.subscribe(({ hide_spoilers }) => {
	const CLASS = "hide-spoilers";

	let c = document.body.classList;
	if (hide_spoilers) {
		c.add(CLASS);
	} else {
		c.remove(CLASS);
	}
});

export function migrate_storage() {
	function migrate(store, default_kv) {
		let s = get(store);
		if (s.version < default_kv.version) {
			Object.keys(s).forEach((k) => store.delete(k));
			Object.entries(default_kv).forEach(([k, v]) => store.set(k, v));
		}
	}

	migrate(SETTINGS, DEFAULT_SETTINGS);
	migrate(SESSION_SETTINGS, DEFAULT_SESSION_SETTINGS);

	let s = get(SETTINGS);
	if (s.version < DEFAULT_SETTINGS.version) {
		let c = get(SELECTED_CATEGORIES);
		Object.keys(c).forEach((c) => SELECTED_CATEGORIES.delete(c));
		Object.entries(default_enabled_categories()).forEach(([k, v]) =>
			SELECTED_CATEGORIES.set(k, v),
		);
	}
}
