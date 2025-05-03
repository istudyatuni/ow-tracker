import { writable } from "svelte/store";

export const OPENED_FACT = writable(null)
export const LOADING = writable('base')
export const SAVE_FOUND = writable(false)

export function open_fact(id) {
	OPENED_FACT.set(id)
}

export function close_fact() {
	OPENED_FACT.set(null)
}
