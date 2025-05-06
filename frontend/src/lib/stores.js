import { derived, writable } from "svelte/store";

import { detect_language } from "./language";

export const OPENED_FACT = writable(null)
export const LOADING = writable('base')
export const SAVE_FOUND = writable(false)
export const LANGUAGE = writable(detect_language())

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
