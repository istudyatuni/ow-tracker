import { derived, get, writable } from "svelte/store";

import { localStore } from 'svelte-storages'

import { default_categories } from "./categories";
import { detect_language } from "./language";

export const OPENED_FACT = writable(null)
export const SAVE_FOUND = writable(null)
export const LOADING = writable('base')
export const LANGUAGE = writable(detect_language())
export const SELECTED_CATEGORIES = localStore('show-categories', default_categories())
export const SAVE_FOUND_CATEGORIES = writable(new Set())

const DEFAULT_SETTINGS = {
	version: 3,
	hide_spoilers: true,
	welcome_popup_done: false,
}
export const SETTINGS = localStore('ow-settings', DEFAULT_SETTINGS)

// max/min in normal coordinates:
// x: [-878, 3341.8005]
// y: [-1577, 1707]
export const MAP_SIZE = writable([[-900, -1600], [3300, 1700]])

/**
 * @type {import("svelte/store").Writable.<import("@fluent/bundle").FluentBundle> | null}
 */
const tr_bundle = writable(null)

export const translator = derived(tr_bundle, (bundle) => (id, args = {}) => {
	if (bundle === null) {
		return ''
	}

	let msg = bundle.getMessage(id)
	if (msg?.value) {
		return bundle.formatPattern(msg.value, args)
	}
	console.warn('no value for message with id', id)
	return id
})

export function open_fact(id) {
	OPENED_FACT.set(id)
}

export function close_fact() {
	OPENED_FACT.set(null)
}

/**
 * @param {import("@fluent/bundle").FluentBundle} bundle
 */
export function set_tr_bundle(bundle) {
	tr_bundle.set(bundle)
}

SETTINGS.subscribe(({ hide_spoilers }) => {
	const CLASS = 'hide-spoilers'

	let c = document.body.classList
	if (hide_spoilers) {
		c.add(CLASS)
	} else {
		c.remove(CLASS)
	}
})

export function migrate_storage() {
	let s = get(SETTINGS)
	if (s.version < DEFAULT_SETTINGS.version) {
		Object.keys(s).forEach((k) => SETTINGS.delete(k))
		Object.entries(DEFAULT_SETTINGS).forEach(([k, v]) => SETTINGS.set(k, v))

		let c = get(SELECTED_CATEGORIES)
		Object.keys(c).forEach((c) => SELECTED_CATEGORIES.delete(c))
		Object.entries(default_categories()).forEach(([k, v]) => SELECTED_CATEGORIES.set(k, v))
	}
}
