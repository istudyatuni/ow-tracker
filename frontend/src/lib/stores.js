import { writable } from "svelte/store";
import { detect_language } from "./language";

export const OPENED_FACT = writable(null)
export const LOADING = writable('base')
export const SAVE_FOUND = writable(false)
export const LANGUAGE = writable(detect_language())

export function open_fact(id) {
	OPENED_FACT.set(id)
}

export function close_fact() {
	OPENED_FACT.set(null)
}
